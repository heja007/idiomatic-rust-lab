use axum::{Json, http::StatusCode, response::IntoResponse};

use super::types::{ErrorBody, ErrorInfo};

pub enum ApiError {
    Validation(String),
    Internal(String),
    TooLarge(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status, code, message) = match self {
            ApiError::Validation(msg) => (StatusCode::BAD_REQUEST, "VALIDATION_ERROR", msg),
            ApiError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", msg),
            ApiError::TooLarge(msg) => (StatusCode::PAYLOAD_TOO_LARGE, "PAYLOAD_TOO_LARGE", msg),
        };

        let body = ErrorBody {
            error: ErrorInfo { code, message },
        };

        (status, Json(body)).into_response()
    }
}
