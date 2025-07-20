mod cache;
mod db;
mod poe;

use anyhow::Result;
use dotenv::dotenv;
use oauth2::reqwest;
use reqwest::header::{ACCEPT, AUTHORIZATION, HeaderValue, USER_AGENT};
use serde::de;
use std::env;
use tracing::{debug, info};
use tracing_subscriber::{fmt, layer::SubscriberExt};

use crate::poe::public_stash::fetch_public_stashes;
use tokio::signal;

async fn get_access_token(http_client: &reqwest::Client) -> Result<String> {
    // Use get_cached_access_token if possible, otherwise use fetch_access_token
    match cache::get_cached_access_token().await {
        Ok(token) => {
            debug!("Using cached access token");
            Ok(token)
        }
        Err(_) => {
            debug!("Failed to retrieve cached access token, fetching a new one");
            let access_token = poe::auth::fetch_access_token(http_client).await?;
            cache::cache_access_token(&access_token).await?;
            debug!("New access token cached successfully");

            Ok(access_token)
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let package_version = env!("CARGO_PKG_VERSION");
    let package_name = env!("CARGO_PKG_NAME");
    let package_author = env!("CARGO_PKG_AUTHORS");

    let layer1 = fmt::Layer::default();
    // let layer2 = tracing_tracy::TracyLayer::default();
    let subscriber = tracing_subscriber::registry().with(layer1); //.with(layer2);

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    info!("Starting pashe-backend...");

    dotenv()?;

    let clickhouse_url =
        env::var("CLICKHOUSE_URL").expect("Missing the CLICKHOUSE_URL environment variable.");
    let clickhouse_user =
        env::var("CLICKHOUSE_USER").expect("Missing the CLICKHOUSE_USER environment variable.");
    let clickhouse_password = env::var("CLICKHOUSE_PASSWORD")
        .expect("Missing the CLICKHOUSE_PASSWORD environment variable.");
    let clickhouse_database = env::var("CLICKHOUSE_DATABASE")
        .expect("Missing the CLICKHOUSE_DATABASE environment variable.");

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        USER_AGENT,
        format!("OAuth {package_name}/{package_version} (contact: {package_author})").parse()?,
    );
    headers.insert(ACCEPT, HeaderValue::from_static("application/json"));

    debug!("Headers: {:?}", headers);

    let http_client = reqwest::ClientBuilder::new()
        .redirect(reqwest::redirect::Policy::none())
        .default_headers(headers.clone())
        .build()?;

    let access_token = get_access_token(&http_client).await?;

    info!("Access Token: {}", access_token);

    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", access_token))?,
    );

    let http_client = reqwest::ClientBuilder::new()
        .redirect(reqwest::redirect::Policy::none())
        .default_headers(headers)
        .build()?;

    let mut next_change_id: String =
        "2865201480-2824479311-2748622439-3058596118-2954720699".to_string();

    loop {
        tokio::select! {
            _ = signal::ctrl_c() => {
                info!("Received Ctrl+C, shutting down gracefully...");
                break;
            }
            _ = tokio::time::sleep(tokio::time::Duration::from_millis(100)) => {
                let stash_changes = fetch_public_stashes(&http_client, next_change_id.as_str()).await?;
                debug!("Fetched {} public stashes", stash_changes.stashes.len());
                let total_items: usize = stash_changes
                    .stashes
                    .iter()
                    .map(|stash| stash.items.len())
                    .sum();
                debug!("Total items across all stashes: {}", total_items);

                next_change_id = stash_changes.next_change_id;
            }
        }
    }

    // let client = db::get_client(
    //     &clickhouse_url,
    //     &clickhouse_user,
    //     &clickhouse_password,
    //     &clickhouse_database,
    // );

    // db::create_tables(&client).await?;

    // client
    //     .query(
    //         "
    //     INSERT INTO items (item) VALUES ('Sample Item 1'), ('Sample Item 2'), ('Sample Item 3')
    // ",
    //     )
    //     .execute()
    //     .await?;

    Ok(())
}
