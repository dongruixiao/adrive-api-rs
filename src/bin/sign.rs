use adrive_api_rs::Auth;

#[tokio::main]
async fn main() {
    let auth = Auth {};
    auth.sign_in().await.unwrap();
}
