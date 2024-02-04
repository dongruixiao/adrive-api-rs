use adrive_api_rs::ADriveAPI;
use adrive_api_rs::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let api = ADriveAPI::new();
    let drive_id = api.get_backup_drive_id().await?;

    let resp = api.get_file_by_id(&drive_id, "id-is-not-found").await;
    println!("{:#?}", resp);
    Ok(())
}
