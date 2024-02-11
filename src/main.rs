use zero2prod::configuration::get_configuration;
use zero2prod::startup::{create_db_pool, run};

#[tokio::main]
async fn main() {
    let configuration = get_configuration().expect("Failed to read configuration");
    let db_pool = create_db_pool(&configuration.database).await;
    run(configuration.application.port, db_pool).await
}
