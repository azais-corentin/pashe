use anyhow::Result;
use clickhouse::Client;

pub fn get_client(url: &str, user: &str, password: &str, database: &str) -> clickhouse::Client {
    Client::default()
        .with_url(url)
        .with_user(user)
        .with_password(password)
        .with_database(database)
}

pub async fn create_tables(client: &Client) -> Result<()> {
    client
        .query(
            "
        CREATE OR REPLACE TABLE items
        (
            id Int64,
            timestamp DateTime DEFAULT now(),
            item LowCardinality(String),
        )
        ENGINE = MergeTree
        ORDER BY timestamp
    ",
        )
        .execute()
        .await?;
    Ok(())
}
