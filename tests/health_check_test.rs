#[tokio::test]
async fn health_check_returns_ok() {
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

async fn start_server() -> String {
    let app = zero2prod::app();
    let listener = zero2prod::listener("0").await; // Random available port
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::serve(app, listener);
    tokio::spawn(server);
    format!("http://0.0.0.0:{}", port)
}
