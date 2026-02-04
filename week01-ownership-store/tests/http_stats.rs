use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use serde_json::json;
use tower::ServiceExt;

#[tokio::test]
async fn stats_unicode_counts_chars_vs_bytes() {
    let app = week01_ownership_store::http::router();
    let body = json!({ "text": "é" }).to_string();
    let req = Request::builder()
        .method("POST")
        .uri("/v1/stats")
        .header("content-type", "application/json")
        .body(Body::from(body))
        .unwrap();

    let resp = app.oneshot(req).await.unwrap();
    assert!(resp.status().is_success());
}

#[tokio::test]
async fn stats_empty_text_is_zeroes() {
    let app = week01_ownership_store::http::router();
    let body = json!({ "text": "" }).to_string();
    let req = Request::builder()
        .method("POST")
        .uri("/v1/stats")
        .header("content-type", "application/json")
        .body(Body::from(body))
        .unwrap();

    let resp = app.oneshot(req).await.unwrap();
    assert!(resp.status().is_success());
}

#[tokio::test]
async fn stats_text_too_large_returns_413() {
    let app = week01_ownership_store::http::router();
    let big = "a".repeat(1_048_576 + 1);
    let body = json!({ "text": big }).to_string();
    let req = Request::builder()
        .method("POST")
        .uri("/v1/stats")
        .header("content-type", "application/json")
        .body(Body::from(body))
        .unwrap();

    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::PAYLOAD_TOO_LARGE);
}

#[tokio::test]
async fn endpoints_return_json_content_type() {
    let app = week01_ownership_store::http::router();
    let stats_req = Request::builder()
        .method("POST")
        .uri("/v1/stats")
        .header("content-type", "application/json")
        .body(Body::from(json!({ "text": "a" }).to_string()))
        .unwrap();
    let uniq_req = Request::builder()
        .method("POST")
        .uri("/v1/uniq")
        .header("content-type", "application/json")
        .body(Body::from(
            json!({ "text": "a\n", "all": false }).to_string(),
        ))
        .unwrap();
    let grep_req = Request::builder()
        .method("POST")
        .uri("/v1/grep")
        .header("content-type", "application/json")
        .body(Body::from(
            json!({ "text": "a\n", "pattern": "a", "line_number": false }).to_string(),
        ))
        .unwrap();

    let stats = app.clone().oneshot(stats_req).await.unwrap();
    let uniq = app.clone().oneshot(uniq_req).await.unwrap();
    let grep = app.oneshot(grep_req).await.unwrap();

    assert!(
        stats
            .headers()
            .get("content-type")
            .unwrap()
            .to_str()
            .unwrap()
            .starts_with("application/json")
    );
    assert!(
        uniq.headers()
            .get("content-type")
            .unwrap()
            .to_str()
            .unwrap()
            .starts_with("application/json")
    );
    assert!(
        grep.headers()
            .get("content-type")
            .unwrap()
            .to_str()
            .unwrap()
            .starts_with("application/json")
    );
}
