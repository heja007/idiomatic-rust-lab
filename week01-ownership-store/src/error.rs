use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;
use thiserror::Error;

#[allow(dead_code)]
#[derive(Debug, Error)]
pub enum StoreError {
    #[error("key already exists")]
    KeyAlreadyExists,
    #[error("key not found")]
    KeyNotFound,

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("unsupported version: {0}")]
    UnsupportedVersion(u32),
}

#[derive(Debug)]
pub enum ApiError {
    NotFound,
    InvalidJson(serde_json::Error),
    Io(std::io::Error),
    #[allow(dead_code)]
    Internal(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, code, message): (StatusCode, &str, String) = match self {
            ApiError::NotFound => (
                StatusCode::NOT_FOUND,
                "not_found",
                "key does not exist".to_string(),
            ),
            ApiError::InvalidJson(err) => {
                (StatusCode::BAD_REQUEST, "invalid_json", err.to_string())
            }
            ApiError::Io(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "io_error",
                err.to_string(),
            ),
            ApiError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, "internal", msg),
        };

        let body = Json(json!({"error": code, "message": message}));

        (status, body).into_response()
    }
}
