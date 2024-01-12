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
    // ADriveAPI::new()
    //     .download_small_file(
    //         &resp.default_drive_id,
    //         "64ce64e210a851618e484a07adb4664ba52976d8",
    //         "./test/a/b/c",
    //     )
    //     .await
    //     .unwrap();
    let id = &resp.default_drive_id;
    let st = std::time::Instant::now();
    let resp = ADriveAPI::new()
        .download_big_file(
            id,
            // "63fcd09f609ce464d23944289fd4d583f8ca100b",
            "62e8920f61dc3a4b20e841bbb05239862cdff8a4",
            "./test/a/b/c.mp4",
        )
        .await
        .unwrap();
    let ed = std::time::Instant::now();
    println!("{:?}", ed - st);

    let resp = ADriveAPI::new()
        .download_small_file(
            id,
            // "63fcd09f609ce464d23944289fd4d583f8ca100b",
            "62e8920f61dc3a4b20e841bbb05239862cdff8a4",
            "./test/a/b/d.mp4",
        )
        .await
        .unwrap();
    println!("{:#?}", std::time::Instant::now() - ed);
}
