use adrive_api_rs::ADriveAPI;
use adrive_api_rs::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let api = ADriveAPI::new();
    let drive_id = api.get_backup_drive_id().await?;

    api.get_file_by_id(&drive_id, "id-is-not-found").await?;
    Ok(())
}
