use crate::api::state::AppState;
use crate::storage::{file, new_store};

mod api;
mod error;
mod model;
mod storage;

#[tokio::main]
async fn main() {
    let store = new_store();

    // load data.json if exists
    if let Ok(map) = file::load_from_file("data.json").await {
        let mut guard = store.write().await;
        *guard = map;
    }

    let state = AppState {
        store,
        data_file: "data.json".to_string(),
    };

    let app = api::router(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("bind failed");

    axum::serve(listener, app).await.expect("server failed");
}
