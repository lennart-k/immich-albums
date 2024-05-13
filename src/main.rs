use std::fs::{self};
use std::path::Path;
use std::str::FromStr;

use anyhow::anyhow;
use chrono::{DateTime, Utc};
use clap::Parser;
use openapi::apis::album_api::{self};
use openapi::apis::configuration::{ApiKey, Configuration};
use openapi::apis::search_api::search_metadata;
use openapi::models::{BulkIdsDto, CreateAlbumDto, MetadataSearchDto, UpdateAlbumDto};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use walkdir::WalkDir;

#[derive(Serialize, Deserialize, Debug)]
struct AlbumInfo {
    metadata: AlbumMetadata,
    #[serde(default)]
    immich: AlbumFileImmich,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct AlbumMetadata {
    name: String,
    description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct AlbumFileImmich {
    album_id: Option<Uuid>,
    updated_at: Option<DateTime<Utc>>,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(
        long,
        env,
        help = "The API host of your Immich instance (must include the /api suffix)"
    )]
    immich_host: String,
    #[arg(
        long,
        env,
        help = "The Immich API token (create it in the user settings)"
    )]
    immich_token: String,
    #[arg(
        long,
        env,
        help = "The filesystem path where immich-albums can find your albums"
    )]
    real_album_path: String,
    #[arg(
        long,
        env,
        help = "The filesystem path where Immich will see your albums"
    )]
    immich_album_path: String,
    #[arg(long, env, help = "The id of the external library")]
    immich_library_id: Uuid,
}

async fn ensure_album(
    api_config: &Configuration,
    album_info: &mut AlbumInfo,
    filepath: &Path,
) -> anyhow::Result<Uuid> {
    // TODO: clean up jank
    if let Some(album_id) = album_info.immich.album_id {
        let album =
            album_api::get_album_info(&api_config, &album_id.to_string(), None, Some(false))
                .await?;
        // TODO: Maybe handle non-existing album?

        // Album already exists, now ensure metadata is correct
        let immich_updated_at = DateTime::<Utc>::from_str(&album.updated_at).unwrap();

        let immich_metadata = AlbumMetadata {
            name: album.album_name,
            description: Some(album.description),
        };

        if immich_metadata != album_info.metadata {
            // Versions differ, now update the correct one
            if let Some(file_updated_at) = album_info.immich.updated_at {
                if immich_updated_at > file_updated_at {
                    // Immich newer than local
                    println!("Updating the local album metadata because we've updated on Immich");
                    album_info.metadata = immich_metadata;
                } else {
                    // Local newer than Immich
                    println!("Updating the Immich album metadata because we've updated locally");
                    let updated_at = album_api::update_album_info(
                        api_config,
                        &album_id.to_string(),
                        UpdateAlbumDto {
                            album_name: Some(album_info.metadata.name.to_owned()),
                            description: album_info.metadata.description.to_owned(),
                            ..Default::default()
                        },
                    )
                    .await?
                    .updated_at;
                    album_info.immich.updated_at = Some(chrono::DateTime::from_str(&updated_at)?);
                }
            } else {
                // We don't know, we will choose the local version
                println!("Updating the Immich album metadata because we don't know what's newer");
                let updated_at = album_api::update_album_info(
                    api_config,
                    &album_id.to_string(),
                    UpdateAlbumDto {
                        album_name: Some(album_info.metadata.name.to_owned()),
                        description: album_info.metadata.description.to_owned(),
                        ..Default::default()
                    },
                )
                .await?
                .updated_at;
                album_info.immich.updated_at = Some(chrono::DateTime::from_str(&updated_at)?);
            };
            fs::write(filepath, toml::to_string_pretty(&album_info)?)?;
        }
        return Ok(album_id);
    }

    // Album doesn't exist, create one
    let album = album_api::create_album(
        api_config,
        CreateAlbumDto {
            album_name: album_info.metadata.name.to_owned(),
            description: album_info.metadata.description.to_owned(),
            ..Default::default()
        },
    )
    .await?;
    let album_id = Uuid::from_str(&album.id)?;
    album_info.immich.album_id = Some(album_id);
    album_info.immich.updated_at = Some(chrono::DateTime::from_str(&album.updated_at)?);
    fs::write(filepath, toml::to_string_pretty(&album_info)?)?;
    Ok(album_id)
}

async fn add_assets_to_album(
    api_config: &Configuration,
    album_id: &Uuid,
    ids: Vec<Uuid>,
) -> anyhow::Result<()> {
    album_api::add_assets_to_album(&api_config, &album_id.to_string(), BulkIdsDto { ids }, None)
        .await?;
    Ok(())
}

async fn update_album_dir(
    args: &Args,
    api_config: &Configuration,
    album_id: Uuid,
    album_dir: &Path,
) -> anyhow::Result<()> {
    let mut image_ids = Vec::new();
    for entry in WalkDir::new(album_dir) {
        let entry = entry?;
        if !entry.file_type().is_file() {
            continue;
        }
        let path = entry.path().canonicalize()?;
        let relpath = path.strip_prefix(&args.real_album_path)?;
        let immich_path = Path::new(&args.immich_album_path).join(relpath);
        let search_result = search_metadata(
            api_config,
            MetadataSearchDto {
                library_id: Some(args.immich_library_id),
                original_path: Some(immich_path.to_str().ok_or(anyhow!("wtf?!"))?.to_owned()),
                ..Default::default()
            },
        )
        .await?;
        if let Some(image) = search_result.assets.items.first() {
            image_ids.push(Uuid::from_str(&image.id).unwrap());
        } else {
            dbg!(format!(
                "Image not found by Immich: {}",
                &immich_path.display()
            ));
            continue;
        }
    }
    // TODO: Maybe add chunking for really large albums?
    add_assets_to_album(api_config, &album_id, image_ids).await?;
    Ok(())
}

async fn handle_album_folder(
    args: &Args,
    api_config: &Configuration,
    album_info_path: &Path,
) -> anyhow::Result<()> {
    let album_folder = album_info_path.parent().ok_or(anyhow!("wtf?"))?;

    let mut album_info: AlbumInfo = toml::from_str(&fs::read_to_string(album_info_path)?)?;
    let album_id = ensure_album(api_config, &mut album_info, album_info_path).await?;
    dbg!(&album_info.metadata.name);
    update_album_dir(args, api_config, album_id, album_folder).await?;
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let api_config = Configuration {
        base_path: args.immich_host.clone(),
        api_key: Some(ApiKey {
            prefix: None,
            key: args.immich_token.clone(),
        }),
        ..Default::default()
    };

    for entry in WalkDir::new(args.real_album_path.clone()) {
        let entry = entry?;
        if !entry.file_type().is_file() {
            continue;
        }
        let path = entry.path();
        match path.extension() {
            Some(path) => {
                if path != "toml" {
                    continue;
                }
            }
            None => continue,
        };
        let album_info_path = path;
        handle_album_folder(&args, &api_config, album_info_path).await?;
    }

    Ok(())
}
