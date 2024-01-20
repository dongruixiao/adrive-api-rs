mod common;
use adrive_api_rs::{ADriveAPI, Result};

#[tokio::test]
#[ignore]
async fn test_user() -> Result<()> {
    let adrive_api: &ADriveAPI = common::adrive_api();
    let _adrive_core_api = common::adrive_core_api();

    let resp = adrive_api.get_user_info().await?;
    println!("{:#?}", resp);

    let resp = adrive_api.get_drive_info().await?;
    println!("{:#?}", resp);

    let resp = adrive_api.get_space_info().await?;
    println!("{:#?}", resp);

    Ok(())
}
