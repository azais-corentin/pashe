mod cache;
mod db;
mod poe;

use anyhow::Result;
use dotenv::dotenv;
use oauth2::reqwest;
use reqwest::header::{ACCEPT, HeaderValue, USER_AGENT};
use std::env;
use tracing::{debug, info};
use tracing_subscriber::{fmt, layer::SubscriberExt};

use crate::poe::public_stash::fetch_public_stashes;

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

    let layer1 = fmt::Layer::default().with_file(true).with_line_number(true);
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

    let http_client = reqwest::ClientBuilder::new()
        .redirect(reqwest::redirect::Policy::none())
        .default_headers(headers.clone())
        .build()?;

    let access_token = get_access_token(&http_client).await?;

    info!("Access Token: {}", access_token);

    headers.insert(
        "Authorization",
        HeaderValue::from_str(&format!("Bearer {}", access_token))?,
    );

    let http_client = reqwest::ClientBuilder::new()
        .redirect(reqwest::redirect::Policy::none())
        .default_headers(headers)
        .build()?;

    fetch_public_stashes(
        &http_client,
        "2865124210-2824412466-2748553974-3058514278-2954646931",
    )
    .await?;

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
