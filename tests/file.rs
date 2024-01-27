mod common;

use adrive_api_rs::{ADriveAPI, Result};
use chrono::Utc;

#[tokio::test]
#[ignore]
async fn test_file() -> Result<()> {
    let adrive_api: &ADriveAPI = common::adrive_api();
    let _adrive_core_api = common::adrive_core_api();

    let drive_id = adrive_api.get_backup_drive_id().await?;

    let parent_id = "root";
    let resp = adrive_api.list_files(&drive_id, parent_id).await?;
    println!("{:#?}", resp);

    let query_selector = "file_extension = 'mp4'";
    let resp = adrive_api.search_files(&drive_id, query_selector).await?;
    println!("{:#?}", resp);

    let resp = adrive_api.list_starred_files(&drive_id).await?;
    println!("{:#?}", resp);

    let file_id = "62e89bd3b9d6ab9196c949a8b6a0f63a4dc22857";
    let resp = adrive_api.get_file_by_id(&drive_id, file_id).await?;
    println!("{:#?}", resp);

    let file_path = "/张汉东 - Rust编程之道 (2019) - libgen.li.pdf";
    let resp = adrive_api.get_file_by_path(&drive_id, file_path).await?;
    println!("{:#?}", resp);

    let file_ids = [
        "64be5da63e2df2c57312431bb0ecd17c0960eb34",
        "62e89bd3b9d6ab9196c949a8b6a0f63a4dc22857",
    ];
    let resp = adrive_api.batch_get_files(&drive_id, &file_ids).await?;
    println!("{:#?}", resp);

    let file_id = "62e89bd3b9d6ab9196c949a8b6a0f63a4dc22857";
    let resp = adrive_api.get_download_url(&drive_id, file_id).await?;
    println!("{:#?}", resp);

    let file_id = "62e89bd3b9d6ab9196c949a8b6a0f63a4dc22857";
    let target_dir = "./tmp";
    let resp = adrive_api
        .download_file_continuously(&drive_id, file_id, target_dir)
        .await?;
    println!("{:#?}", resp);

    let dir_name = "test";
    let resp = adrive_api
        .create_folder(&drive_id, "root", dir_name)
        .await?;
    println!("{:#?}", resp);
    adrive_api.delete_file(&drive_id, &resp).await?;
    println!("{:#?}", resp);

    let parent_id: &str = "65abd292f867ee78856b4c1ba5db30af4b9213a6";
    let file_path = "./tmp/test.file2";
    adrive_api
        .upload_file(&drive_id, parent_id, file_path)
        .await?;
    println!("{:#?}", resp);

    let file_id = "62e89bd3b9d6ab9196c949a8b6a0f63a4dc22857";
    let resp = adrive_api.starred_file(&drive_id, file_id).await?;
    println!("{:#?}", resp);
    let resp = adrive_api.unstarred_file(&drive_id, file_id).await?;
    println!("{:#?}", resp);

    let file_id = "65ab71583bc7891eebd6491b8c1d67b0c450c306";
    let _resp = adrive_api
        .rename_file(
            &drive_id,
            file_id,
            &format!("abc-{}", &Utc::now().timestamp().to_string()),
        )
        .await?;

    let file_id = "63fcd09f609ce464d23944289fd4d583f8ca100b";
    let target_parent_id = "65ab71583bc7891eebd6491b8c1d67b0c450c306";
    adrive_api
        .move_file(&drive_id, file_id, target_parent_id, None)
        .await?;
    println!("{:#?}", resp);
    adrive_api
        .move_file(&drive_id, file_id, "root", None)
        .await?;
    println!("{:#?}", resp);

    let file_id = "63fcd09f609ce464d23944289fd4d583f8ca100b";
    let target_parent_id = "65ab71583bc7891eebd6491b8c1d67b0c450c306";
    let resp = adrive_api
        .copy_file(&drive_id, file_id, target_parent_id)
        .await?;
    println!("{:#?}", resp);
    adrive_api.delete_file(&drive_id, &resp).await?;
    println!("{:#?}", resp);

    let file_id = "65a3fd0ebed88b3dd64a4073be604310a2d946c7";
    adrive_api.recycle_file(&drive_id, file_id).await?;
    println!("{:#?}", resp);
    Ok(())
}
