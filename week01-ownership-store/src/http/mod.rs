use axum::{Router, routing::post};

pub mod errors;
pub mod handlers;
pub mod types;

pub fn router() -> Router {
    Router::new()
        .route("/v1/stats", post(handlers::stats))
        .route("/v1/uniq", post(handlers::uniq))
        .route("/v1/grep", post(handlers::grep))
}
