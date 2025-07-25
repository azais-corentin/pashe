use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Clickhouse error: {0}")]
    Clickhouse(#[from] clickhouse::error::Error),
}
