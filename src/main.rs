use adrive_api_rs::{ADriveAPI, ADriveCoreAPI};

#[tokio::main]
async fn main() {
    // let resp = Auth {}.refresh_token().await.unwrap();
    // Auth::new().refresh_if_needed().await.unwrap();
    // let resp = ADriveAPI::new().user_info().await.unwrap();
    // println!("{:#?}", resp);
    // let resp = ADriveAPI::new().drive_info().await.unwrap();
    // let resp = ADriveAPI::new().space_info().await.unwrap();
    // println!("{:#?}", resp);
    // let resp = ADriveAPI::new()
    //     .get_file_list(&resp.resource_drive_id.unwrap(), "root")
    //     .await
    //     .unwrap();
    // println!("{:#?}", resp)
    // let resp = ADriveAPI::new()
    //     .search_for_file(&resp.default_drive_id, "name match '张汉东'")
    //     .await
    //     .unwrap();
    // println!("{:#?}", resp);
    // let resp = ADriveAPI::new()
    //     .download_small_file(
    //         &resp.default_drive_id,
    //         "65a3f073c16f7706600147ad8f63010bed3ee312",
    //         "../test.file.alipan",
    //     )
    //     .await
    //     .unwrap();
    // let id = &resp.default_drive_id;
    // let st = std::time::Instant::now();
    // let resp = ADriveAPI::new()
    //     .download_big_file(
    //         id,
    //         // "63fcd09f609ce464d23944289fd4d583f8ca100b",
    //         "64c1130b27cf0ebef36a48dc940f2c353cbbc86b",
    //         "./test/a/b/c.mp4",
    //     )
    //     .await
    //     .unwrap();
    // let ed = std::time::Instant::now();
    // println!("{:?}", ed - st);

    // let resp = ADriveAPI::new()
    //     .download_small_file(
    //         id,
    //         // "63fcd09f609ce464d23944289fd4d583f8ca100b",
    //         "64c1130b27cf0ebef36a48dc940f2c353cbbc86b",
    //         "./test/a/b/d.mp4",
    //     )
    //     .await
    //     .unwrap();
    // let resp = ADriveAPI::new()
    //     .multiparts_upload_file(
    //         id,
    //         "root",
    //         "/Users/dongruixiao/PlayGround/adrive-api-rs/test.file2",
    //     )
    //     .await
    //     .unwrap();

    // let resp = ADriveAPI::new()
    //     .list_uploaded_parts(
    //         id,
    //         "65a2a8ad426e0e52ed3546269143a3a672f45500",
    //         "B345566FAF804027B89FA0F1E9475258",
    //     )
    //     .await
    //     .unwrap();
    let api = ADriveAPI::new();
    let _drive_id = api.get_backup_drive_id().await.unwrap();
    // println!("{}", drive_id);
    // let parent_id = "63c9025cee4e56f1855947ffbc7944a25d5591e8";
    // let resp = api.list_files(&drive_id, parent_id).await.unwrap();

    // let items = resp
    //     .iter()
    //     .map(|item| item.file_id.as_str())
    //     .collect::<Vec<_>>();
    // println!("{}", items.len());
    // let resp = api.batch_get_files(&drive_id, &items).await.unwrap();
    // let resp = api
    //     .download_file_continuously(
    //         &drive_id,
    //         "64c1130b27cf0ebef36a48dc940f2c353cbbc86b",
    //         "./test/a/b",
    //     )
    //     .await
    //     .unwrap();
    let api = ADriveCoreAPI::new();
    let drive_id = api.get_drive_info().await.unwrap().backup_drive_id.unwrap();
    let resp = api.create_folder(&drive_id, "root", "a").await.unwrap();
    println!("{:#?}", resp);
}
