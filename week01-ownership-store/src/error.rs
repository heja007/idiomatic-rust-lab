use thiserror::Error;

#[derive(Debug, Error)]
pub enum StoreError {
    #[error("key already exists")]
    KeyAlreadyExists,
    #[error("key not found")]
    KeyNotFound,
}
