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
    let layer1 = fmt::Layer::default().with_file(true).with_line_number(true);
    let layer2 = tracing_tracy::TracyLayer::default();
    let subscriber = tracing_subscriber::registry().with(layer1).with(layer2);

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    info!("Starting pashe-backend...");

    dotenv().ok();

    let package_version = env!("CARGO_PKG_VERSION");
    let package_name = env!("CARGO_PKG_NAME");
    let package_author = env!("CARGO_PKG_AUTHORS");

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        USER_AGENT,
        format!("OAuth {package_name}/{package_version} (contact: {package_author})").parse()?,
    );
    headers.insert(ACCEPT, HeaderValue::from_static("application/json"));

    let http_client = reqwest::ClientBuilder::new()
        .redirect(reqwest::redirect::Policy::none())
        .default_headers(headers)
        .build()?;

    let access_token = get_access_token(&http_client).await?;

    info!("Access Token: {}", access_token);

    let client = db::get_client();

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
