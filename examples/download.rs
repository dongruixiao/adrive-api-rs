use adrive_api_rs::ADriveAPI;
use adrive_api_rs::Result;
#[tokio::main]
async fn main() -> Result<()> {
    //     use adrive_api_rs::ADriveCoreAPI;
    //     let api = ADriveCoreAPI::new();
    //     let drive_id = api.get_drive_info().await.unwrap().backup_drive_id.unwrap();
    //     // let file_path = "/Users/ruixiao.dong/Desktop/张汉东 - Rust 编程之道 (2019) - libgen.li.pdf";
    //     let file_path = "/Users/ruixiao.dong/Desktop/zhanghandong-rust.file";
    //     let mut file = std::fs::File::open(file_path).unwrap();
    //     let resp = api
    //         .upload_file_with_check(
    //             &drive_id,
    //             "root",
    //             "raycast.dmg",
    //             &mut file,
    //             false,
    //             false,
    //         )
    //         .await
    //         .unwrap();
    //     println!("{:#?}", resp);
    //     ()
    let api = ADriveAPI::new();
    let drive_id = api.get_backup_drive_id().await?;

    let parent_id: &str = "root";
    let file_path = "./random-file.400m1";
    // let file_id = "65b286aff44b517fc5bc4198a39930bc92196c44";
    let resp = api.upload_file(&drive_id, parent_id, file_path).await?;
    // let resp = api.download_file_directly(&drive_id, file_id, "./").await?;
    println!("{:#?}", resp);
    Ok(())
}
