use zero2prod::configuration::get_configuration;
use zero2prod::startup::{connect_db, create_app, initialize_logger, serve, start_listener};

#[tokio::main]
async fn main() {
    initialize_logger();
    let configuration = get_configuration();
    let listener = start_listener(configuration.application.port).await;
    let db = connect_db(&configuration.database).await;
    let app = create_app(db);
    serve(listener, app).await;
}
