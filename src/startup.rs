use axum::{
    body::Body,
    http::Request,
    routing::{get, post},
    Router,
};
use secrecy::ExposeSecret;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::time::Duration;
use tower_http::trace::TraceLayer;
use tower_request_id::{RequestId, RequestIdLayer};
// use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{configuration, routes};

pub fn create_app(db_pool: PgPool) -> Router {
    Router::new()
        .route("/health", get(routes::status::health_check))
        .route("/subscriptions", post(routes::subscriptions::subscribe))
        .with_state(db_pool)
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request<Body>| {
                let request_id = request
                    .extensions()
                    .get::<RequestId>()
                    .map(ToString::to_string)
                    .unwrap_or_else(|| "unknown".into());
                tracing::error_span!(
                    "request",
                    id = %request_id,
                    method = %request.method(),
                    uri = %request.uri(),
                    version = ?request.version(),
                )
            }),
        )
        .layer(RequestIdLayer)
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
        .connect(settings.connection_string().expose_secret())
        .await
        .expect("Failed to connect to Postgres")
}

#[cfg(not(tarpaulin_include))]
pub fn initialize_logger() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                "zero2prod=debug,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        // .with(JsonStorageLayer)
        // .with(BunyanFormattingLayer::new(
        //     "zero2prod".into(),
        //     std::io::stdout,
        // ))
        .with(tracing_subscriber::fmt::layer())
        .init();
}
