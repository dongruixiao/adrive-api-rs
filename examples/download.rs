#[tokio::main]
async fn main() {
    use adrive_api_rs::ADriveCoreAPI;
    let api = ADriveCoreAPI::new();
    let drive_id = api.get_drive_info().await.unwrap().backup_drive_id.unwrap();
    // let file_path = "/Users/ruixiao.dong/Desktop/张汉东 - Rust 编程之道 (2019) - libgen.li.pdf";
    let file_path = "/Users/ruixiao.dong/Desktop/zhanghandong-rust.file";
    let mut file = std::fs::File::open(file_path).unwrap();
    let resp = api
        .upload_file_with_check(
            &drive_id,
            "root",
            "raycast.dmg",
            &mut file,
            false,
            false,
        )
        .await
        .unwrap();
    println!("{:#?}", resp);
    ()
}
