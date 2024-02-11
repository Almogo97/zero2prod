use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[tokio::main]
async fn main() {
    let configuration = get_configuration().expect("Failed to read configuration");
    run(configuration.application.port).await
}
