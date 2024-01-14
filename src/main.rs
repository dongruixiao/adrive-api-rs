use adrive_api_rs::ADriveAPI;

#[tokio::main]
async fn main() {
    // Auth::new().sign_in().await.unwrap();
    // Auth::new().refresh_if_needed().await.unwrap();
    // let resp = ADriveAPI::new().user_info().await.unwrap();
    // println!("{:#?}", resp);
    let resp = ADriveAPI::new().drive_info().await.unwrap();
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
    let id = &resp.default_drive_id;
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
    let resp = ADriveAPI::new()
        .multiparts_upload_file(
            id,
            "root",
            "/Users/dongruixiao/PlayGround/adrive-api-rs/test.file2",
        )
        .await
        .unwrap();

    // let resp = ADriveAPI::new()
    //     .list_uploaded_parts(
    //         id,
    //         "65a2a8ad426e0e52ed3546269143a3a672f45500",
    //         "B345566FAF804027B89FA0F1E9475258",
    //     )
    //     .await
    //     .unwrap();
    println!("{:#?}", resp);
}
