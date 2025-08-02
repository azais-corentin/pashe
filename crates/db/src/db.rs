use anyhow::Result;
use tracing::info;

pub struct DatabaseConfig {
    pub url: String,
    pub user: String,
    pub password: String,
    pub database: String,
}

impl DatabaseConfig {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            url: std::env::var("CLICKHOUSE_URL")
                .expect("Missing the CLICKHOUSE_URL environment variable."),
            user: std::env::var("CLICKHOUSE_USER")
                .expect("Missing the CLICKHOUSE_USER environment variable."),
            password: std::env::var("CLICKHOUSE_PASSWORD")
                .expect("Missing the CLICKHOUSE_PASSWORD environment variable."),
            database: std::env::var("CLICKHOUSE_DATABASE")
                .expect("Missing the CLICKHOUSE_DATABASE environment variable."),
        })
    }

    pub fn new(url: String, user: String, password: String, database: String) -> Self {
        Self {
            url,
            user,
            password,
            database,
        }
    }

    pub fn create_client(&self) -> clickhouse::Client {
        clickhouse::Client::default()
            .with_url(&self.url)
            .with_user(&self.user)
            .with_password(&self.password)
            .with_database(&self.database)
    }
}

pub async fn reset(client: &clickhouse::Client, force: bool) -> Result<()> {
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
