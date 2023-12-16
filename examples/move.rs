#[tokio::main]
async fn main() {
    use adrive_api_rs::ADriveAPI;
    let mut api = ADriveAPI::new();
    let resp = api
        .move_file(
            "643ab76e008df77d7ef54eab8bce15b95c58719d",
            "643ab76defc3d34ca5744bed84c57c51b68669ce",
        )
        .await
        .unwrap();
    println!("{:?}", resp.responses.get(0).unwrap().body);
}
