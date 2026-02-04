use crate::api::handlers::*;
use axum::Router;
use axum::routing::{get, post};

use crate::api::state::AppState;
pub mod handlers;
pub mod state;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/kv", get(get_all))
        .route("/kv/{key}", post(put_key).get(get_key).delete(delete_key))
        .with_state(state)
}
