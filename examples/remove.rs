#[tokio::main]
async fn main() {
    use adrive_api_rs::ADriveAPI;
    let mut api = ADriveAPI::new();
    let mut fileids = Vec::new();
    fileids.push("63cfe2370ef39baee09b4a6297a270579daa16b1");
    fileids.push("63fb489469169ac2ef58462cb7ccd17679ac79e5");
    api.remove_files(fileids).await.unwrap();
}
