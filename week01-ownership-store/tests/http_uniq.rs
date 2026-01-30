mod test_server;
use test_server::spawn_app;

use reqwest::Client;
use serde_json::json;

#[tokio::test]
async fn uniq_basic_collapsing() {
    let app = week01_ownership_store::http::router();
    let (addr, _handle) = spawn_app(app).await;
    let url = format!("http://{}/v1/uniq", addr);

    let client = Client::new();
    let body = json!({
          "text": "a\na\nb\nb\nb\nc\n",
          "all": false
      });

    let resp = client.post(url).json(&body).send().await.unwrap();
    assert!(resp.status().is_success());

    let data: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(data["text"], "a\nb\nc\n");
    assert_eq!(data["removed"], 3);
}

#[tokio::test]
async fn uniq_all_true() {
    let app = week01_ownership_store::http::router();
    let (addr, _handle) = spawn_app(app).await;
    let url = format!("http://{}/v1/uniq", addr);

    let client = Client::new();
    let body = json!({
          "text": "a\na\nb\nb\nb\nc\n",
          "all": true
      });

    let resp = client.post(url).json(&body).send().await.unwrap();
    assert!(resp.status().is_success());

    let data: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(data["text"], "a\nb\nc\n");
    assert_eq!(data["removed"], 3);
}
