#[tokio::main]
async fn main() {
    let x_device_id = "TtJyG9Dr1xACAW/HvPmBEBpm";
    let x_signature = "";

    use adrive_api_rs::ADriveAPI;
    let mut api = ADriveAPI::new();
    api.download_file(
        // "643ad4e655588154356944f49739d5fe28c4e8b6",
        "63fcd09f609ce464d23944289fd4d583f8ca100b",
        "/Users/dongruixiao/Desktop/test/test.file",
    )
    .await
    .unwrap();
}
