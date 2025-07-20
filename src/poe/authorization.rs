use anyhow::Result;
use oauth2::basic::BasicClient;
use oauth2::{ClientId, ClientSecret, Scope, TokenResponse, TokenUrl};
use std::env;

pub async fn fetch_access_token(http_client: &reqwest::Client) -> Result<String> {
    let client_id = env::var("CLIENT_ID").expect("Missing the CLIENT_ID environment variable.");
    let client_secret =
        env::var("CLIENT_SECRET").expect("Missing the CLIENT_SECRET environment variable.");

    let scope = "service:psapi";
    let token_url = TokenUrl::new("https://www.pathofexile.com/oauth/token".to_string())?;

    let client = BasicClient::new(ClientId::new(client_id))
        .set_client_secret(ClientSecret::new(client_secret))
        .set_token_uri(token_url);

    let token_result = client
        .exchange_client_credentials()
        .add_scope(Scope::new(scope.to_string()))
        .request_async(http_client)
        .await?;

    Ok(token_result.access_token().secret().to_string())
}
