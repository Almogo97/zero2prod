use axum::{
    routing::{get, post},
    Router,
};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::time::Duration;
use tower_http::trace::TraceLayer;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{configuration, routes};

pub fn create_app(db_pool: PgPool) -> Router {
    Router::new()
        .route("/health", get(routes::status::health_check))
        .route("/subscriptions", post(routes::subscriptions::subscribe))
        .with_state(db_pool)
        .layer(TraceLayer::new_for_http())
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

pub fn initialize_logger() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                "zero2prod=debug,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(JsonStorageLayer)
        .with(BunyanFormattingLayer::new(
            "zero2prod".into(),
            std::io::stdout,
        ))
        .init();
}
