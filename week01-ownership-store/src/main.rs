use axum::Router;

mod http;

#[tokio::main]
async fn main() {
    let app: Router = http::router();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("bind failed");

    axum::serve(listener, app).await.expect("server failed");
}
