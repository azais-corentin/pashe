use redis::AsyncCommands;
use std::env;

pub async fn get_cached_access_token() -> redis::RedisResult<String> {
    let redis_url = env::var("REDIS_URL").expect("Missing the REDIS_URL environment variable.");

    let client = redis::Client::open(redis_url)?;
    let mut con = client.get_multiplexed_async_connection().await?;

    con.get("access_token").await
}

pub async fn cache_access_token(token: &str) -> redis::RedisResult<()> {
    let redis_url = env::var("REDIS_URL").expect("Missing the REDIS_URL environment variable.");

    let client = redis::Client::open(redis_url)?;
    let mut con = client.get_multiplexed_async_connection().await?;

    // 27 days validity period
    con.set_ex("access_token", token, 27 * 24 * 60 * 60).await
}
