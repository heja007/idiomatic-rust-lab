mod test_server;
use test_server::spawn_app;

use reqwest::Client;
use serde_json::json;

#[tokio::test]
async fn stats_unicode_counts_chars_vs_bytes() {
    let app = week01_ownership_store::http::router();
    let (addr, _handle) = spawn_app(app).await;
    let url = format!("http://{}/v1/stats", addr);

    let client = Client::new();
    let body = json!({ "text": "é" });
    let resp = client.post(url).json(&body).send().await.unwrap();
    assert!(resp.status().is_success());

    let data: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(data["chars"], 1);
    assert_eq!(data["bytes"], 2);
}

#[tokio::test]
async fn stats_empty_text_is_zeroes() {
    let app = week01_ownership_store::http::router();
    let (addr, _handle) = spawn_app(app).await;
    let url = format!("http://{}/v1/stats", addr);

    let client = Client::new();
    let body = json!({ "text": "" });
    let resp = client.post(url).json(&body).send().await.unwrap();
    assert!(resp.status().is_success());

    let data: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(data["lines"], 0);
    assert_eq!(data["words"], 0);
    assert_eq!(data["chars"], 0);
    assert_eq!(data["bytes"], 0);
}

#[tokio::test]
async fn stats_text_too_large_returns_413() {
    let app = week01_ownership_store::http::router();
    let (addr, _handle) = spawn_app(app).await;
    let url = format!("http://{}/v1/stats", addr);

    let client = Client::new();
    let big = "a".repeat(1_048_576 + 1);
    let body = json!({ "text": big });

    let resp = client.post(url).json(&body).send().await.unwrap();
    assert_eq!(resp.status().as_u16(), 413);

    let data: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(data["error"]["code"], "PAYLOAD_TOO_LARGE");
}


#[tokio::test]
async fn endpoints_return_json_content_type() {
    let app = week01_ownership_store::http::router();
    let (addr, _handle) = spawn_app(app).await;

    let client = Client::new();

    let stats_url = format!("http://{}/v1/stats", addr);
    let uniq_url = format!("http://{}/v1/uniq", addr);
    let grep_url = format!("http://{}/v1/grep", addr);

    let stats = client.post(stats_url).json(&json!({ "text": "a" })).send().await.unwrap();
    let uniq = client.post(uniq_url).json(&json!({ "text": "a\n", "all":
  false })).send().await.unwrap();
    let grep = client.post(grep_url).json(&json!({ "text": "a\n", "pattern": "a",
  "line_number": false })).send().await.unwrap();

    assert!(stats.headers()["content-type"].to_str().unwrap().starts_with("application/json"));
    assert!(uniq.headers()["content-type"].to_str().unwrap().starts_with("application/json"));
    assert!(grep.headers()["content-type"].to_str().unwrap().starts_with("application/json"));
}
