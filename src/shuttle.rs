use shuttle_axum::ShuttleAxum;
use shuttle_runtime::CustomError;
use sqlx::PgPool;
use zero2prod::startup::create_app;

#[shuttle_runtime::main]
async fn axum(#[shuttle_shared_db::Postgres] pool: PgPool) -> ShuttleAxum {
    sqlx::migrate!()
        .run(&pool)
        .await
        .map_err(CustomError::new)?;

    let app = create_app(pool);
    Ok(app.into())
}
