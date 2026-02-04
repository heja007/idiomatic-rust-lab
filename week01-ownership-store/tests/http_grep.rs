use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use serde_json::json;
use tower::ServiceExt;

#[tokio::test]
async fn grep_empty_pattern_returns_400() {
    let app = week01_ownership_store::http::router();
    let body = json!({ "text": "foo\nbar\n", "pattern": "", "line_number": false }).to_string();
    let req = Request::builder()
        .method("POST")
        .uri("/v1/grep")
        .header("content-type", "application/json")
        .body(Body::from(body))
        .unwrap();

    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn grep_with_line_numbers() {
    let app = week01_ownership_store::http::router();
    let body = json!({
        "text": "foo\nbar\nfood\n",
        "pattern": "foo",
        "line_number": true
    })
    .to_string();

    let req = Request::builder()
        .method("POST")
        .uri("/v1/grep")
        .header("content-type", "application/json")
        .body(Body::from(body))
        .unwrap();

    let resp = app.oneshot(req).await.unwrap();
    assert!(resp.status().is_success());
}
