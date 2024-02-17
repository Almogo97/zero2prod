use std::time::Duration;

use axum::{
    routing::{get, post},
    Router,
};
use sqlx::{postgres::PgPoolOptions, PgPool};

use crate::{configuration, routes};

pub fn create_app(db_pool: PgPool) -> Router {
    Router::new()
        .route("/health", get(routes::status::health_check))
        .route("/subscriptions", post(routes::subscriptions::subscribe))
        .with_state(db_pool)
}

pub async fn start_listener(port: u16) -> tokio::net::TcpListener {
    let address = format!("0.0.0.0:{}", port);
    tokio::net::TcpListener::bind(address).await.unwrap()
}

pub async fn serve(listener: tokio::net::TcpListener, app: Router) {
    axum::serve(listener, app).await.unwrap()
}

pub async fn connect_db(settings: &configuration::DatabaseSettings) -> sqlx::Pool<sqlx::Postgres> {
    PgPoolOptions::new()
        .acquire_timeout(Duration::from_secs(3))
        .connect(&settings.connection_string())
        .await
        .expect("Failed to connect to Postgres")
}
