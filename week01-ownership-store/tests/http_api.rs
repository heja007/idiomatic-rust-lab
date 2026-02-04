use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use serde_json::json;
use tempfile::tempdir;
use tower::ServiceExt;
use week01_ownership_store::api::state::AppState;

#[tokio::test]
async fn get_missing_key_returns_404() {
    let dir = tempdir().unwrap();
    let data_file = dir.path().join("data.json");

    let store = week01_ownership_store::storage::new_store();
    let state = AppState {
        store,
        data_file: data_file.to_string_lossy().to_string(),
    };
    let app = week01_ownership_store::api::router(state);

    let req = Request::builder()
        .method("GET")
        .uri("/kv/missing")
        .body(Body::empty())
        .unwrap();

    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn put_then_get_returns_value() {
    let dir = tempdir().unwrap();
    let data_file = dir.path().join("data.json");

    let store = week01_ownership_store::storage::new_store();
    let state = AppState {
        store,
        data_file: data_file.to_string_lossy().to_string(),
    };
    let app = week01_ownership_store::api::router(state);

    let body = json!({"name": "alice"}).to_string();
    let req = Request::builder()
        .method("POST")
        .uri("/kv/user")
        .header("content-type", "application/json")
        .body(Body::from(body))
        .unwrap();

    let _ = app.clone().oneshot(req).await.unwrap();

    let get_req = Request::builder()
        .method("GET")
        .uri("/kv/user")
        .body(Body::empty())
        .unwrap();

    let get_resp = app.oneshot(get_req).await.unwrap();
    assert_eq!(get_resp.status(), StatusCode::OK);
}

#[tokio::test]
async fn restart_preserves_data() {
    let dir = tempdir().unwrap();
    let data_file = dir.path().join("data.json");
    let data_path = data_file.to_string_lossy().to_string();

    let store1 = week01_ownership_store::storage::new_store();
    let state1 = AppState {
        store: store1,
        data_file: data_path.clone(),
    };
    let app1 = week01_ownership_store::api::router(state1);

    let body = json!({"x": 1}).to_string();
    let req = Request::builder()
        .method("POST")
        .uri("/kv/a")
        .header("content-type", "application/json")
        .body(Body::from(body))
        .unwrap();
    let _ = app1.oneshot(req).await.unwrap();

    let store2 = week01_ownership_store::storage::new_store();
    let map = week01_ownership_store::storage::file::load_from_file(&data_path)
        .await
        .unwrap();
    {
        let mut guard = store2.write().await;
        *guard = map;
    }
    let state2 = AppState {
        store: store2,
        data_file: data_path.clone(),
    };
    let app2 = week01_ownership_store::api::router(state2);

    let get_req = Request::builder()
        .method("GET")
        .uri("/kv/a")
        .body(Body::empty())
        .unwrap();
    let get_resp = app2.oneshot(get_req).await.unwrap();
    assert_eq!(get_resp.status(), StatusCode::OK);
}
