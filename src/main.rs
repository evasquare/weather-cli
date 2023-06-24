use weather_cli::cmd_line;

#[tokio::main]
async fn main() {
    cmd_line::init().await;
}
