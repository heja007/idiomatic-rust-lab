use axum::body::{Body, to_bytes};
use axum::http::Request;
use serde_json::json;
use tower::ServiceExt;

#[tokio::test]
async fn uniq_basic_collapsing() {
    let app = week01_ownership_store::http::router();
    let body = json!({
        "text": "a\na\nb\nb\nb\nc\n",
        "all": false
    })
    .to_string();

    let req = Request::builder()
        .method("POST")
        .uri("/v1/uniq")
        .header("content-type", "application/json")
        .body(Body::from(body))
        .unwrap();

    let resp = app.oneshot(req).await.unwrap();
    assert!(resp.status().is_success());

    let bytes = to_bytes(resp.into_body(), usize::MAX).await.unwrap();
    let data: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(data["text"], "a\nb\nc\n");
    assert_eq!(data["removed"], 3);
}

#[tokio::test]
async fn uniq_all_true() {
    let app = week01_ownership_store::http::router();
    let body = json!({
        "text": "a\na\nb\nb\nb\nc\n",
        "all": true
    })
    .to_string();

    let req = Request::builder()
        .method("POST")
        .uri("/v1/uniq")
        .header("content-type", "application/json")
        .body(Body::from(body))
        .unwrap();

    let resp = app.oneshot(req).await.unwrap();
    assert!(resp.status().is_success());

    let bytes = to_bytes(resp.into_body(), usize::MAX).await.unwrap();
    let data: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(data["text"], "a\nb\nc\n");
    assert_eq!(data["removed"], 3);
}
