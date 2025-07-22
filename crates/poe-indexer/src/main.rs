mod cache;
mod db;
mod poe;

use anyhow::Result;
use dotenv::dotenv;
use oauth2::reqwest;
use reqwest::header::{ACCEPT, AUTHORIZATION, HeaderValue, USER_AGENT};
use std::{env, sync::Arc};
use tokio_util::sync::CancellationToken;
use tracing::{debug, info};
use tracing_subscriber::{fmt, layer::SubscriberExt};

use crate::poe::{public_stash::Crawl, rate_limiting::RateLimitingMiddleware};
use tokio::signal;

async fn get_access_token(http_client: &reqwest::Client) -> Result<String> {
    match cache::get_cached_access_token().await {
        Ok(token) => {
            debug!("Using cached access token");
            Ok(token)
        }
        Err(_) => {
            debug!("Failed to retrieve cached access token, fetching a new one");
            let access_token = poe::authorization::fetch_access_token(http_client).await?;
            cache::cache_access_token(&access_token).await?;
            debug!("New access token cached successfully");

            Ok(access_token)
        }
    }
}

fn setup_tracing() {
    let filter = tracing_subscriber::filter::Targets::new()
        .with_target("pashe_backend", tracing::level_filters::LevelFilter::TRACE);
    let layer1 = fmt::Layer::default();
    let layer2 = tracing_tracy::TracyLayer::default();
    let subscriber = tracing_subscriber::registry()
        .with(layer1)
        .with(layer2)
        .with(filter);

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}

fn setup_shutdown_handler() -> CancellationToken {
    let shutdown_token = CancellationToken::new();
    let cloned_shutdown_token = shutdown_token.clone();

    tokio::spawn(async move {
        signal::ctrl_c()
            .await
            .expect("Failed to listen for shutdown signal");
        info!("Shutdown signal received, shutting down gracefully...");
        shutdown_token.cancel();
    });

    cloned_shutdown_token
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv()?;
    const PACKAGE_VERSION: &str = env!("CARGO_PKG_VERSION");
    const PACKAGE_NAME: &str = env!("CARGO_PKG_NAME");
    const PACKAGE_AUTHOR: &str = env!("CARGO_PKG_AUTHORS");

    setup_tracing();
    let shutdown_token = setup_shutdown_handler();

    info!("Starting pashe-backend...");

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
        format!("OAuth {PACKAGE_NAME}/{PACKAGE_VERSION} (contact: {PACKAGE_AUTHOR})").parse()?,
    );
    headers.insert(ACCEPT, HeaderValue::from_static("application/json"));

    let http_client = reqwest::ClientBuilder::new()
        .redirect(reqwest::redirect::Policy::none())
        .default_headers(headers.clone())
        .build()?;

    let access_token = get_access_token(&http_client).await?;

    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {access_token}"))?,
    );

    let http_client = reqwest::ClientBuilder::new()
        .redirect(reqwest::redirect::Policy::none())
        .default_headers(headers.clone())
        .build()?;

    let http_client = reqwest_middleware::ClientBuilder::new(http_client)
        .with(RateLimitingMiddleware::default())
        .build();

    debug!("Fetching initial next_change_id from poe.ninja");
    let ninja = http_client
        .get("https://poe.ninja/api/data/getstats")
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;
    let next_change_id: String = ninja["next_change_id"]
        .as_str()
        .ok_or(anyhow::anyhow!(
            "Failed to get next_change_id from poe.ninja response"
        ))?
        .to_string();

    info!("Starting crawler at next_change_id: {}", next_change_id);

    let stash_crawler = Arc::new(poe::public_stash::Crawler::new(shutdown_token.clone()));
    stash_crawler
        .crawl(Arc::new(http_client), next_change_id.to_string())
        .await?;

    shutdown_token.cancelled().await;

    info!("Shutdown");

    /*
    while !shutdown_token.is_cancelled() {
        let my_stashes: Vec<_> = stash_changes
            .stashes
            .iter()
            .filter(|stash| {
                stash
                    .account_name
                    .as_ref()
                    .map_or(false, |name| name.to_lowercase().contains("haellsigh"))
            })
            .collect();

        if !my_stashes.is_empty() {
            info!("Found {} stashes belonging to Haellsigh", my_stashes.len());
        }

        for stash in my_stashes {
            for item in &stash.items {
                info!(
                    "Found item: {}, {} ({})",
                    item.name, item.type_line, item.base_type
                );
            }
        }

        next_change_id = stash_changes.next_change_id;
    }
    */

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
