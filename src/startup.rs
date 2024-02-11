use axum::{
    routing::{get, post},
    Router,
};

use crate::routes;

pub async fn run(port: &str) {
    let app = app();
    let listener = listener(port).await;
    serve(app, listener).await
}

pub fn app() -> Router {
    Router::new()
        .route("/health", get(routes::status::health_check))
        .route("/subscriptions", post(routes::subscriptions::subscribe))
}

pub async fn listener(port: &str) -> tokio::net::TcpListener {
    let address = format!("0.0.0.0:{}", port);
    tokio::net::TcpListener::bind(address).await.unwrap()
}

pub async fn serve(app: Router, listener: tokio::net::TcpListener) {
    axum::serve(listener, app).await.unwrap()
}
