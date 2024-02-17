use rstest::*;

use crate::common::{assert_status_code, client, TestClient};

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
async fn subscribe_returns_400_when_missing_data(
    #[future] client: TestClient,
    #[case] body: String,
) {
    let response = client.post("/subscriptions", body).await;

    assert_status_code(422, response);
}
