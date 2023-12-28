use adrive_api_rs::auth::Auth;
use adrive_api_rs::ADriveAPI;

#[tokio::main]
async fn main() {
    // Auth::new().sign_in().await.unwrap();
    // Auth::new().refresh_if_needed().await.unwrap();
    let resp = ADriveAPI::new().user_info().await.unwrap();
    println!("{:#?}", resp);
    let resp = ADriveAPI::new().drive_info().await.unwrap();
    println!("{:#?}", resp);
    let resp = ADriveAPI::new().space_info().await.unwrap();
    println!("{:#?}", resp);
}
