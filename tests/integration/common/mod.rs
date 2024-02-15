use rstest::*;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;
use zero2prod::{configuration, startup};

pub async fn start_server(db: PgPool) -> String {
    let app = startup::app(db).await;
    let listener = startup::listener(0).await; // Random available port
    let port = listener.local_addr().unwrap().port();
    let server = startup::serve(app, listener);
    tokio::spawn(server);
    format!("http://0.0.0.0:{}", port)
}

pub fn assert_status_code(expected: u16, response: reqwest::Response) {
    assert_eq!(expected, response.status().as_u16());
}

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

#[fixture]
pub async fn db() -> PgPool {
    let mut configuration =
        configuration::get_configuration().expect("Failed to read configuration");
    configuration.database.database = Uuid::new_v4().to_string();

    let mut connection =
        PgConnection::connect(&configuration.database.connection_string_without_db())
            .await
            .expect("Failed to connect to Postgres");
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, configuration.database.database).as_str())
        .await
        .expect("Failed to create database.");

    let db_pool = startup::create_db_pool(&configuration.database).await;
    sqlx::migrate!("./migrations")
        .run(&db_pool)
        .await
        .expect("Failed to migrate the database");
    db_pool
}

#[fixture]
#[awt]
pub async fn client(#[future] db: PgPool) -> TestClient {
    let address = start_server(db.clone()).await;
    TestClient::new(address, db)
}
