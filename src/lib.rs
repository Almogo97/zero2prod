use axum::{
    http::StatusCode,
    routing::{get, post},
    Form, Router,
};

pub async fn run(port: &str) {
    let app = app();
    let listener = listener(port).await;
    serve(app, listener).await
}

pub async fn serve(app: Router, listener: tokio::net::TcpListener) {
    axum::serve(listener, app).await.unwrap()
}

pub fn app() -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/subscriptions", post(subscribe))
}

pub async fn listener(port: &str) -> tokio::net::TcpListener {
    let address = format!("0.0.0.0:{}", port);
    tokio::net::TcpListener::bind(address).await.unwrap()
}

async fn health_check() -> StatusCode {
    StatusCode::OK
}

async fn subscribe(Form(payload): Form<FormData>) -> StatusCode {
    StatusCode::OK
}

#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String,
}
