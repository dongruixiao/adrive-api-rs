#[tokio::main]
async fn main() {
    use ::adrive_api_rs::ADriveAPI;
    let mut api = ADriveAPI::new();
    api.upload_file(
        "root",
        "/Users/dongruixiao/PlayGround/adrive-api-rs/randomxyz.file.88m",
        None,
        None,
    )
    .await
    .unwrap();
}
