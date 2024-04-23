use weather_cli::cli;

#[tokio::main]
async fn main() {
    cli::init().await;
}
