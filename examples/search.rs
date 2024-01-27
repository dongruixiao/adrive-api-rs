use adrive_api_rs::ADriveAPI;
use adrive_api_rs::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let api = ADriveAPI::new();
    let drive_id = api.get_backup_drive_id().await?;

    let resp = api.search_files(&drive_id, "file_extension='mp4'").await?;
    println!("{:#?}", resp);
    Ok(())
}
