use adrive_api_rs::auth::Auth;

#[tokio::main]
async fn main() {
    // Auth::new().sign_in().await.unwrap();
    Auth::new().refresh_if_needed().await.unwrap();
}
