use adrive_api_rs::ADriveAPI;

#[tokio::main]
async fn main() {
    let mut api = ADriveAPI::new();
    let res = api.listdir("root", 100).await.unwrap();
    println!("{:#?}", res);
}
