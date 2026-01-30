use axum::Router;
use std::net::SocketAddr;
use tokio::task::JoinHandle;

pub async fn spawn_app(app: Router) -> (SocketAddr, JoinHandle<()>) {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.expect("bind failed");

    let addr = listener.local_addr().expect("local_addr failed");

    let handle = tokio::spawn(async move {
        axum::serve(listener, app).await.expect("serve failed");
    });

    (addr, handle)
}
