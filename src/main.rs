use dotenv::dotenv;
use oauth2::basic::BasicClient;
use oauth2::http::HeaderValue;
use oauth2::reqwest;
use oauth2::{ClientId, ClientSecret, Scope, TokenResponse, TokenUrl};
use reqwest::header::{ACCEPT, USER_AGENT};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let client_id = env::var("CLIENT_ID").expect("Missing the CLIENT_ID environment variable.");
    let client_secret =
        env::var("CLIENT_SECRET").expect("Missing the CLIENT_SECRET environment variable.");
    let package_version = env!("CARGO_PKG_VERSION");
    let package_name = env!("CARGO_PKG_NAME");
    let package_author = env!("CARGO_PKG_AUTHORS");

    let scope = "service:psapi";

    let token_url = TokenUrl::new("https://www.pathofexile.com/oauth/token".to_string())?;

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

    let client = BasicClient::new(ClientId::new(client_id))
        .set_client_secret(ClientSecret::new(client_secret))
        .set_token_uri(token_url);

    let token_result = client
        .exchange_client_credentials()
        .add_scope(Scope::new(scope.to_string()))
        .request_async(&http_client)
        .await?;

    println!("Access Token: {}", token_result.access_token().secret());

    Ok(())
}
