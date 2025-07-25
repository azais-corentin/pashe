use clickhouse::{Client, Row};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::env;
use serde_repr::{Deserialize_repr, Serialize_repr};

/// Period types for statistics aggregation
#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(i8)]
pub enum PeriodType {
    Total = 0,
    Year = 1,
    Month = 2,
    Day = 3,
    Hour = 4,
    Minute = 5,
    What1 = 20,
}

#[derive(Debug, Row, Serialize, Deserialize)]
pub struct StatisticsPerPeriod {
    pub period_type: PeriodType,
    #[serde(with = "clickhouse::serde::chrono::datetime")]
    pub period_start: DateTime<Utc>,
    pub total_stash_count: u64,
    pub total_item_count: u64,
    pub total_bytes: u64,
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn get_statistics_per_periods() -> Result<Vec<StatisticsPerPeriod>, String> {
    // Load environment variables
    dotenv::dotenv().ok();

    let clickhouse_url =
        env::var("CLICKHOUSE_URL").expect("CLICKHOUSE_URL must be set in .env file");
    let clickhouse_user =
        env::var("CLICKHOUSE_USER").expect("CLICKHOUSE_USER must be set in .env file");
    let clickhouse_password =
        env::var("CLICKHOUSE_PASSWORD").expect("CLICKHOUSE_PASSWORD must be set in .env file");
    let clickhouse_database =
        env::var("CLICKHOUSE_DATABASE").expect("CLICKHOUSE_DATABASE must be set in .env file");

    let client = Client::default()
        .with_url(&clickhouse_url)
        .with_user(&clickhouse_user)
        .with_password(&clickhouse_password)
        .with_database(&clickhouse_database);

    let query = r#"
        SELECT
            period_type,
            period_start,
            sum(total_stash_count) AS total_stash_count,
            sum(total_item_count) AS total_item_count,
            sum(total_bytes) AS total_bytes
        FROM statistics_per_periods
        GROUP BY
            period_type,
            period_start
        ORDER BY
            period_type,
            period_start
    "#;

    match client.query(query).fetch_all::<StatisticsPerPeriod>().await {
        Ok(results) => Ok(results),
        Err(e) => Err(format!("Database error: {}", e)),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, get_statistics_per_periods])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
