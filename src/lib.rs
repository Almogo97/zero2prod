use axum::{http::StatusCode, routing::get, Router};

pub async fn run(port: &str) {
    let app = app();
    let listener = listener(port).await;
    serve(app, listener).await
}

pub async fn serve(app: Router, listener: tokio::net::TcpListener) {
    axum::serve(listener, app).await.unwrap()
}

pub fn app() -> Router {
    Router::new().route("/health", get(health_check))
}

pub async fn listener(port: &str) -> tokio::net::TcpListener {
    let address = format!("0.0.0.0:{}", port);
    tokio::net::TcpListener::bind(address).await.unwrap()
}

async fn health_check() -> StatusCode {
    StatusCode::OK
}
