use anyhow::Result;
use clap::Parser;
use db::{
    cli::{Cli, Commands, MigrationCommands},
    create, reset, to, version,
};
use tracing_subscriber::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv()?;

    let filter = tracing_subscriber::filter::Targets::new()
        .with_default(tracing::Level::TRACE)
        .with_target("hyper_util", tracing::Level::INFO);

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().compact())
        .with(filter)
        .init();

    let cli = Cli::parse();

    match &cli.command {
        Commands::Migration(migration) => match &migration.command {
            MigrationCommands::Create { name } => create(&cli, name).await?,
            MigrationCommands::To { version } => to(&cli, *version).await?,
        },
        Commands::Reset { .. } => reset(&cli).await?,
        Commands::Version => {
            let _ = version().await?;
        }
    }

    Ok(())
}
