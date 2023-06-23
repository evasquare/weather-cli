use dotenv::dotenv;
mod cmd_line;
mod weather;

#[tokio::main]
async fn main() {
    dotenv().ok();
    cmd_line::init().await;
}
