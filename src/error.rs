use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Tori has done some stupiding")]
    Tori,
    #[error("Discord error {0}")]
    Discord(#[from] serenity::Error),
    #[error("Database error {0}")]
    Database(#[from] diesel::result::Error),
    #[error("Database Pool error {0}")]
    DbPool(#[from] r2d2::Error),
    #[error("Unknown url passed: {0}")]
    UnknownUrl(String),
    #[error("Json Error {0}")]
    Serde(#[from] serde_json::Error),
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("The specified Vahti already exists")]
    VahtiExists,
    #[error("Invalid Item passed")]
    InvalidItem,
}
