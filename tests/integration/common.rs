use std::env;

use rstest::*;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;
use zero2prod::{configuration, startup};

pub use self::assertions::assert_status_code;
pub use self::test_client::TestClient;

mod assertions;
mod test_client;

#[fixture]
pub async fn client(_initialize_logger: ()) -> TestClient {
    let db = get_db().await;
    let address = start_server(db.clone()).await;
    TestClient::new(address, db)
}

#[fixture]
#[once]
fn initialize_logger() {
    if env::var("RUST_LOG").is_err() {
        // Remove log output unless RUST_LOG variable is set up in the environment
        env::set_var("RUST_LOG", "")
    }
    startup::initialize_logger();
}

async fn start_server(db: PgPool) -> String {
    let app = startup::create_app(db);
    let listener = startup::start_listener(0).await; // Random available port
    let port = listener.local_addr().unwrap().port();
    let server = startup::serve(listener, app);
    tokio::spawn(server);
    format!("http://0.0.0.0:{}", port)
}

async fn get_db() -> PgPool {
    let mut configuration = configuration::get_configuration();
    configuration.database.name = Uuid::new_v4().to_string();

    create_db(&configuration.database).await;

    let db = startup::connect_db(&configuration.database).await;
    sqlx::migrate!("./migrations")
        .run(&db)
        .await
        .expect("Failed to migrate the database");
    db
}

async fn create_db(settings: &configuration::DatabaseSettings) {
    let mut connection = PgConnection::connect(&settings.connection_string_without_db())
        .await
        .expect("Failed to connect to Postgres");
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, settings.name).as_str())
        .await
        .expect("Failed to create database.");
}
