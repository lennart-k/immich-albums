use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
    str::FromStr,
};

use anyhow::anyhow;
use chrono::{DateTime, Utc};
use clap::Args;
use log::{info, warn};
use openapi::{
    apis::{album_api, configuration::Configuration, library_api, search_api::search_metadata},
    models::{BulkIdsDto, CreateAlbumDto, MetadataSearchDto},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use walkdir::WalkDir;

use crate::{
    album::AlbumMetadata,
    locator::{AlbumLocator, DefaultAlbumLocator},
};

#[derive(Debug, Args)]
pub struct ExternalAlbumsArgs {
    #[arg(
        long,
        env,
        help = "The filesystem path where immich-albums can find your albums"
    )]
    real_album_path: PathBuf,
    #[arg(
        long,
        env,
        help = "The filesystem path where Immich will see your albums"
    )]
    immich_album_path: String,
    #[arg(long, env, help = "The id of the external library")]
    immich_library_id: Uuid,
    #[arg(
        long,
        env,
        help = "Create an album.toml for discovered albums without one. (Otherwise the album will be ignored for now)"
    )]
    create_album_file: bool,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct AlbumInfo {
    metadata: AlbumMetadata,
    #[serde(default)]
    immich: AlbumFileImmich,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct AlbumFileImmich {
    album_id: Option<Uuid>,
    updated_at: Option<DateTime<Utc>>,
}

/// Ensures that an album exists and syncs its metadata
async fn sync_album_info(
    api_config: &Configuration,
    album_info: &mut AlbumInfo,
    filepath: &Path,
) -> anyhow::Result<Uuid> {
    // TODO: clean up jank
    if let Some(album_id) = album_info.immich.album_id {
        let album =
            album_api::get_album_info(api_config, &album_id.to_string(), None, Some(false)).await?;
        // TODO: Maybe handle non-existing album?

        // Album already exists, now ensure metadata is correct
        let immich_updated_at = DateTime::<Utc>::from_str(&album.updated_at).unwrap();

        let immich_metadata = AlbumMetadata {
            name: album.album_name,
            description: Some(album.description),
        };

        if immich_metadata != album_info.metadata {
            info!("The local album metadata is not in sync with Immich");
            // Versions differ, now update the correct one
            if let Some(file_updated_at) = album_info.immich.updated_at {
                if immich_updated_at > file_updated_at {
                    // Immich newer than local
                    info!("Updating the local album metadata because Immich's version is newer");
                    album_info.metadata = immich_metadata;
                } else {
                    // Local newer than Immich
                    info!("Updating the Immich album metadata because we've updated locally");
                    let updated_at = album_api::update_album_info(
                        api_config,
                        &album_id.to_string(),
                        album_info.metadata.clone().into(),
                    )
                    .await?
                    .updated_at;
                    album_info.immich.updated_at = Some(chrono::DateTime::from_str(&updated_at)?);
                }
            } else {
                // We don't know, we will choose the local version
                info!("Updating the Immich album metadata because we don't know what's newer");
                let updated_at = album_api::update_album_info(
                    api_config,
                    &album_id.to_string(),
                    album_info.metadata.clone().into(),
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
    info!("Adding {} new assets to album {album_id}", ids.len());
    if ids.is_empty() {
        return Ok(());
    }
    album_api::add_assets_to_album(api_config, &album_id.to_string(), BulkIdsDto { ids }, None)
        .await?;
    Ok(())
}

async fn update_album_dir(
    args: &ExternalAlbumsArgs,
    api_config: &Configuration,
    album_id: Uuid,
    album_dir: &Path,
    exclusion_patterns: &[glob::Pattern],
) -> anyhow::Result<()> {
    let existing_image_ids: HashSet<Uuid> =
        album_api::get_album_info(api_config, &album_id.to_string(), None, Some(false))
            .await?
            .assets
            .iter_mut()
            .map(|asset_response| Uuid::from_str(&asset_response.id).unwrap())
            .collect();

    let mut image_ids = Vec::new();
    for entry in WalkDir::new(album_dir) {
        let entry = entry?;
        if !entry.file_type().is_file() {
            continue;
        }
        let path = entry.path().canonicalize()?;
        if exclusion_patterns
            .iter()
            .any(|pattern| pattern.matches(path.to_str().unwrap()))
        {
            // If file is excluded, ignore
            continue;
        }
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
            warn!("Image not found by Immich: {}", &immich_path.display());
            continue;
        }
    }
    let new_image_ids = image_ids
        .into_iter()
        .filter(|id| !existing_image_ids.contains(id))
        .collect();
    // TODO: Maybe add chunking for really large albums?
    add_assets_to_album(api_config, &album_id, new_image_ids).await?;
    Ok(())
}

async fn handle_album_folder(
    args: &ExternalAlbumsArgs,
    api_config: &Configuration,
    album_info_path: &Path,
    exclusion_patterns: &[glob::Pattern],
) -> anyhow::Result<()> {
    let album_folder = album_info_path.parent().ok_or(anyhow!("wtf?"))?;

    let mut album_info: AlbumInfo = toml::from_str(&fs::read_to_string(album_info_path)?)?;
    let album_id = sync_album_info(api_config, &mut album_info, album_info_path).await?;
    update_album_dir(args, api_config, album_id, album_folder, exclusion_patterns).await?;
    Ok(())
}
pub async fn external_albums_command(
    api_config: &Configuration,
    args: &ExternalAlbumsArgs,
) -> anyhow::Result<()> {
    let library = library_api::get_library(api_config, &args.immich_library_id.to_string()).await?;

    let mut exclusion_patterns: Vec<glob::Pattern> = library
        .exclusion_patterns
        .iter()
        .map(|pattern| glob::Pattern::new(pattern).unwrap())
        .collect();
    exclusion_patterns.push(glob::Pattern::new("**/*.toml").unwrap());

    let locator = DefaultAlbumLocator::new(args.real_album_path.clone());

    for (path_match, dir_entry) in locator.locate_iter() {
        let album_path = dir_entry.path();
        info!("Found album at {}", album_path.to_str().unwrap());
        // Ensure album.toml exists
        let album_file_path = album_path.join("album.toml");
        if !album_file_path.is_file() {
            if !args.create_album_file {
                // TODO: proper handling
                return Err(anyhow!(
                    "Not allowed to create missing file: {}",
                    album_file_path.to_str().unwrap()
                ));
            }

            warn!("Album found without album.toml, creating one");
            fs::write(
                &album_file_path,
                toml::to_string_pretty(&AlbumInfo {
                    metadata: path_match,
                    ..Default::default()
                })?,
            )?;
        }

        handle_album_folder(args, api_config, &album_file_path, &exclusion_patterns).await?;
    }
    Ok(())
}
