use axum::{extract::State, http::StatusCode};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{domain::NewSubscriber, validators::ValidatedForm};

pub async fn subscribe(
    State(pool): State<PgPool>,
    ValidatedForm(payload): ValidatedForm<NewSubscriber>,
) -> StatusCode {
    tracing::info!(
        "Registering new subscriber: {} [{}]",
        payload.name,
        payload.email
    );
    match sqlx::query!(
        r#"
            INSERT INTO subscriptions (id, email, name, subscribed_at)
            VALUES ($1, $2, $3, $4)
            "#,
        Uuid::new_v4(),
        payload.email,
        payload.name,
        Utc::now()
    )
    .execute(&pool)
    .await
    {
        Ok(_) => StatusCode::OK,
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
