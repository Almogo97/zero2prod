use rstest::rstest;

#[tokio::test]
async fn health_check_returns_200() {
    let address = start_server().await;
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/health", address))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_200() {
    let address = start_server().await;
    let client = reqwest::Client::new();
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    let response = client
        .post(format!("{}/subscriptions", address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(200, response.status().as_u16());
}

#[rstest]
#[case::missing_email("name=le%20gu")]
#[case::missing_name("email=ursula_le_guin%40gmail.com")]
#[case::missing_all("")]
#[tokio::test]
async fn subscribe_returns_400_when_missing_data(#[case] body: String) {
    let address = start_server().await;
    let client = reqwest::Client::new();

    let response = client
        .post(format!("{}/subscriptions", address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(422, response.status().as_u16());
}

async fn start_server() -> String {
    let app = zero2prod::app();
    let listener = zero2prod::listener("0").await; // Random available port
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::serve(app, listener);
    tokio::spawn(server);
    format!("http://0.0.0.0:{}", port)
}
