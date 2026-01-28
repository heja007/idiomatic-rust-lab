use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum StoreError {
    #[error("key already exists")]
    KeyAlreadyExists,
    #[error("key not found")]
    KeyNotFound,

    #[error("io error: {0}")]
    Io(#[from] io::Error),

    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("unsupported version: {0}")]
    UnsupportedVersion(u32),
}
