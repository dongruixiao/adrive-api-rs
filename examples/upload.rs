use adrive_api_rs::ADriveAPI;
use adrive_api_rs::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let api = ADriveAPI::new();
    let drive_id = api.get_backup_drive_id().await?;

    let parent_id: &str = "root";
    let file_path = "/path/to/file";
    let resp = api.upload_file(&drive_id, parent_id, file_path).await?;
    println!("{:#?}", resp);
    Ok(())
}
