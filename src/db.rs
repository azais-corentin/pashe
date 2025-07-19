use anyhow::Result;
use clickhouse::Client;

pub fn get_client() -> clickhouse::Client {
    Client::default()
        .with_url("http://db:8123")
        .with_user("pashe")
        .with_password("pashe")
        .with_database("pashe")
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
