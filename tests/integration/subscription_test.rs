use crate::common::{assert_status_code, client, TestClient};
use chrono::Utc;
use rstest::*;
use uuid::Uuid;

#[rstest]
#[awt]
#[tokio::test]
async fn subscribe_returns_200(#[future] client: TestClient) {
    let response = client
        .post(
            "/subscriptions",
            "name=le%20guin&email=ursula_le_guin%40gmail.com".into(),
        )
        .await;

    assert_status_code(200, response);

    let saved = sqlx::query!("SELECT email, name FROM subscriptions")
        .fetch_one(&client.db)
        .await
        .expect("Failed to fetch saved subscription");
    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
}

#[rstest]
#[case::missing_email("name=le%20gu")]
#[case::missing_name("email=ursula_le_guin%40gmail.com")]
#[case::missing_all("")]
#[awt]
#[tokio::test]
async fn subscribe_returns_422_when_missing_data(
    #[future] client: TestClient,
    #[case] body: String,
) {
    let response = client.post("/subscriptions", body).await;

    assert_status_code(422, response);
}

#[rstest]
#[awt]
#[tokio::test]
async fn subscribe_returns_500_when_email_duplicated(#[future] client: TestClient) {
    // TODO: Must return 400 in the future
    sqlx::query!(
        r#"
            INSERT INTO subscriptions (id, email, name, subscribed_at)
            VALUES ($1, $2, $3, $4)
            "#,
        Uuid::new_v4(),
        "ursula_le_guin@gmail.com",
        "le guin",
        Utc::now()
    )
    .execute(&client.db)
    .await
    .unwrap();
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com".to_string();

    let response = client.post("/subscriptions", body).await;

    assert_status_code(500, response);
}
