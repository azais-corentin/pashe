mod cache;
mod db;
mod poe;

use anyhow::Result;
use http::header::ACCEPT_ENCODING;
use oauth2::reqwest;
use reqwest::header::{ACCEPT, AUTHORIZATION, HeaderValue, USER_AGENT};
use std::{env, sync::Arc};
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info};
use tracing_subscriber::{fmt, layer::SubscriberExt};

use crate::poe::rate_limit::RateLimitMiddleware;
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
    let subscriber = tracing_subscriber::registry().with(layer1).with(filter);

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
    dotenvy::dotenv()?;
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
    headers.insert(ACCEPT_ENCODING, HeaderValue::from_static("gzip"));

    debug!("Fetching initial next_change_id from poe.ninja");
    let ninja = reqwest::get("https://poe.ninja/api/data/getstats")
        .await?
        .json::<serde_json::Value>()
        .await?;

    let next_change_id: String = ninja["next_change_id"]
        .as_str()
        .ok_or(anyhow::anyhow!(
            "Failed to get next_change_id from poe.ninja response"
        ))?
        .to_string();

    let http_client = reqwest::ClientBuilder::new()
        .redirect(reqwest::redirect::Policy::none())
        .no_gzip()
        .default_headers(headers.clone())
        .build()?;

    let http_client = reqwest_middleware::ClientBuilder::new(http_client)
        .with(RateLimitMiddleware::default())
        .build();

    info!("Starting crawler at next_change_id: {}", next_change_id);

    let stash_crawler = Arc::new(poe::public_stash_worker::PublicStashWorker::new(
        shutdown_token.clone(),
    ));

    // Set up channels for concurrent crawling
    let (next_change_id_tx, mut next_change_id_rx) = mpsc::unbounded_channel::<String>();
    let (stash_changes_tx, stash_changes_rx) =
        mpsc::unbounded_channel::<(poe::types::PublicStashTabs, u32, u32)>();

    // Initialize the database client
    let db = db::Client::new(
        &clickhouse_url,
        &clickhouse_user,
        &clickhouse_password,
        &clickhouse_database,
    );

    // Start the stash processor task
    let processor_self = Arc::clone(&stash_crawler);
    let processor_handle = tokio::spawn(async move {
        processor_self.process_stash(stash_changes_rx, db).await;
    });

    // Send the initial change ID to start the process
    next_change_id_tx.send(next_change_id)?;

    // Main crawling loop
    loop {
        tokio::select! {
            // Check for shutdown
            _ = shutdown_token.cancelled() => {
                debug!("Shutting down crawler");
                break;
            }

            // Process new change IDs
            Some(change_id) = next_change_id_rx.recv() => {
                let client_clone = Arc::new(http_client.clone());
                let next_change_id_tx_clone = next_change_id_tx.clone();
                let stash_changes_tx_clone = stash_changes_tx.clone();
                let stash_crawler_clone = Arc::clone(&stash_crawler);

                // Spawn a new task for each stash crawler
                tokio::spawn(async move {
                    if let Err(e) = stash_crawler_clone.fetch_stash(
                        client_clone,
                        change_id,
                        next_change_id_tx_clone,
                        stash_changes_tx_clone,
                    ).await {
                        error!("Stash crawler failed: {}", e);
                    }
                });
            }

            // Break if the channel is closed and no more IDs are coming
            else => break,
        }
    }

    // Clean shutdown: drop the senders to signal processors to stop
    drop(next_change_id_tx);
    drop(stash_changes_tx);

    // Wait for the processor to finish
    if let Err(e) = processor_handle.await {
        error!("Processor task failed: {}", e);
    }

    shutdown_token.cancelled().await;

    info!("Shutdown");

    Ok(())
}
