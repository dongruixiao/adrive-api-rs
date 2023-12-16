// 643acb4ac6b3b766acd849a2a7d58f0edd870171

#[tokio::main]
async fn main() {
    use adrive_api_rs::ADriveAPI;
    let mut api = ADriveAPI::new();
    let resp = api
        .unstarred_file("643acb4ac6b3b766acd849a2a7d58f0edd870171")
        .await
        .unwrap();
    println!("{:#?}", resp);
}
