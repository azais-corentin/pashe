use thiserror::Error;

#[derive(Error, Debug)]
pub enum DbError {
    #[error("Clickhouse error")]
    Clickhouse(#[from] clickhouse::error::Error),
    #[error("number error")]
    Number(#[from] std::num::ParseIntError),
    #[error("unknown version")]
    UnknownVersion,
}
