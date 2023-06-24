use dotenv::dotenv;
use weather_cli::cmd_line;

#[tokio::main]
async fn main() {
    dotenv().ok();
    cmd_line::init().await;
}
