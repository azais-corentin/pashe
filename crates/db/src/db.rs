use anyhow::Result;
use tracing::info;

use crate::cli::{Cli, Commands};

pub async fn reset(cli: &Cli) -> Result<()> {
    let force = if let Commands::Reset { force } = cli.command {
        force
    } else {
        false
    };

    let client = get_db();
    let result = client.query("SHOW TABLES").fetch_all::<String>().await?;

    if result.is_empty() {
        info!("No tables found in the database.");
        return Ok(());
    }

    if !force {
        info!(
            "Are you sure you want to drop the table{} {}? [y/N]",
            if result.len() > 1 { "s" } else { "" },
            result.join(", ")
        );
        let mut confirmation = String::new();
        std::io::stdin()
            .read_line(&mut confirmation)
            .expect("Failed to read line");

        if confirmation.trim().to_lowercase() != "y" {
            info!("No changes made, exiting.");
            return Ok(());
        }
    }

    let tasks = result.iter().map(|table_name| {
        let client = client.clone();
        let table_name = table_name.clone();
        tokio::spawn(async move {
            client
                .query(format!("DROP TABLE IF EXISTS {table_name}").as_str())
                .execute()
                .await
                .expect("Failed to drop table");
            info!("Dropped table: {table_name}");
        })
    });

    for task in tasks {
        task.await?;
    }

    Ok(())
}

pub fn get_db() -> clickhouse::Client {
    let clickhouse_url =
        std::env::var("CLICKHOUSE_URL").expect("Missing the CLICKHOUSE_URL environment variable.");
    let clickhouse_user = std::env::var("CLICKHOUSE_USER")
        .expect("Missing the CLICKHOUSE_USER environment variable.");
    let clickhouse_password = std::env::var("CLICKHOUSE_PASSWORD")
        .expect("Missing the CLICKHOUSE_PASSWORD environment variable.");
    let clickhouse_database = std::env::var("CLICKHOUSE_DATABASE")
        .expect("Missing the CLICKHOUSE_DATABASE environment variable.");

    clickhouse::Client::default()
        .with_url(clickhouse_url)
        .with_user(clickhouse_user)
        .with_password(clickhouse_password)
        .with_database(clickhouse_database)
}
