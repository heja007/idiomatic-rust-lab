mod test_server;

use reqwest::Client;
use serde_json::json;
use test_server::spawn_app;

#[tokio::test]
async fn grep_empty_pattern_returns_400() {
    let app = week01_ownership_store::http::router();
    let (addr, _handle) = spawn_app(app).await;
    let uri = format!("http://{}/v1/grep", addr);

    let client = Client::new();
    let body = json!({ "text": "foo\nbar\n", "pattern": "", "line_number": false });
    let resp = client.post(uri).json(&body).send().await.unwrap();

    assert_eq!(resp.status().as_u16(), 400);

    let data: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(data["error"]["code"], "VALIDATION_ERROR");
}

#[tokio::test]
async fn grep_with_line_numbers() {
    let app = week01_ownership_store::http::router();
    let (addr, _handle) = spawn_app(app).await;
    let url = format!("http://{}/v1/grep", addr);

    let client = Client::new();
    let body = json!({
          "text": "foo\nbar\nfood\n",
          "pattern": "foo",
          "line_number": true
      });

    let resp = client.post(url).json(&body).send().await.unwrap();
    assert!(resp.status().is_success());

    let data: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(data["count"], 2);
    assert_eq!(data["matches"][0]["line"], 1);
    assert_eq!(data["matches"][0]["text"], "foo");
    assert_eq!(data["matches"][1]["line"], 3);
    assert_eq!(data["matches"][1]["text"], "food");
}