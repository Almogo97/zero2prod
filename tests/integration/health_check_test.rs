use rstest::*;

use crate::common::{client, TestClient};

#[rstest]
#[awt]
#[tokio::test]
async fn health_check_returns_200(#[future] client: TestClient) {
    let response = client.get("/health").await;

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
