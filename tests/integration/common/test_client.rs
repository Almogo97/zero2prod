use sqlx::PgPool;

pub struct TestClient {
    address: String,
    client: reqwest::Client,
    pub db: PgPool,
}

impl TestClient {
    pub fn new(address: String, db: PgPool) -> TestClient {
        TestClient {
            address,
            client: reqwest::Client::new(),
            db,
        }
    }

    pub async fn get(&self, uri: &str) -> reqwest::Response {
        let url = self.get_url(uri);
        self.client
            .get(url)
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn post(&self, uri: &str, body: String) -> reqwest::Response {
        let url = self.get_url(uri);
        self.client
            .post(url)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Failed to execute request")
    }

    fn get_url(&self, uri: &str) -> reqwest::Url {
        reqwest::Url::parse(&self.address)
            .unwrap()
            .join(uri)
            .unwrap()
    }
}
