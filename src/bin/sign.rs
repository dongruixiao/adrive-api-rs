use adrive_api_rs::Auth;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let auth = Auth {};
    auth.sign_in().await.unwrap();
}
