use zero2prod::startup::run;

#[tokio::main]
async fn main() {
    run("8000").await
}
