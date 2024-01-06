use adrive_api_rs::ADriveAPI;

#[tokio::main]
async fn main() {
    // Auth::new().sign_in().await.unwrap();
    // Auth::new().refresh_if_needed().await.unwrap();
    // let resp = ADriveAPI::new().user_info().await.unwrap();
    // println!("{:#?}", resp);
    let resp = ADriveAPI::new().drive_info().await.unwrap();
    println!("{:#?}", resp);
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
    ADriveAPI::new()
        .download_small_file(
            &resp.default_drive_id,
            "64ce64e210a851618e484a07adb4664ba52976d8",
            "./test/a/b/c",
        )
        .await
        .unwrap();
    println!("{:#?}", resp);
}
