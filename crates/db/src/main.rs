use anyhow::Result;
use clap::Parser;
use db::{
    cli::{Cli, Commands, MigrationCommands},
    create, reset, test, to, version,
};
use tracing_subscriber::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    if dotenvy::dotenv().is_err() {
        println!("No .env file found");
    }

    let filter = tracing_subscriber::filter::Targets::new()
        .with_default(tracing::Level::TRACE)
        .with_target("hyper_util", tracing::Level::INFO);

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().compact())
        .with(filter)
        .init();

    let cli = Cli::parse();

    let client = db::DatabaseConfig::from_env()?.create_client();

    match &cli.command {
        Commands::Migration(migration) => match &migration.command {
            MigrationCommands::Create { name } => create(cli.directory.as_str(), name).await?,
            MigrationCommands::To { version } => {
                to(&client, cli.directory.as_str(), version).await?
            }
            MigrationCommands::Test => {
                test(&client, cli.directory.as_str()).await?;
            }
        },
        Commands::Reset { force } => reset(&client, *force).await?,
        Commands::Version => {
            let _ = version(&client).await?;
        }
    }

    Ok(())
}
