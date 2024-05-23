use clap::{Parser, Subcommand};
use external_albums::{external_albums_command, ExternalAlbumsArgs};
use openapi::apis::configuration::{ApiKey, Configuration};

mod album;
mod external_albums;
mod locator;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct GlobalArgs {
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

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    ExternalAlbums(ExternalAlbumsArgs),
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();
    let args = GlobalArgs::parse();
    let api_config = Configuration {
        base_path: args.immich_host.clone(),
        api_key: Some(ApiKey {
            prefix: None,
            key: args.immich_token.clone(),
        }),
        ..Default::default()
    };

    match args.command {
        Command::ExternalAlbums(args) => external_albums_command(&api_config, &args).await?,
    };

    Ok(())
}
