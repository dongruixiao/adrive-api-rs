pub mod auth;
pub mod core;
pub mod data;

pub use core::{ADriveCoreAPI, Result};
pub use data::{
    FileListingItem as FileItem, GetDriveInfoResponse as DriveInfo,
    GetSpaceInfoResponse as SpaceInfo, GetUserInfoResponse as UserInfo, Request,
};

pub struct ADriveAPI {
    inner: ADriveCoreAPI,
}

impl ADriveAPI {
    pub fn new() -> Self {
        Self {
            inner: ADriveCoreAPI::new(),
        }
    }

    pub async fn user_info(&self) -> Result<UserInfo> {
        self.inner.user_info().await
    }

    pub async fn drive_info(&self) -> Result<DriveInfo> {
        self.inner.drive_info().await
    }

    pub async fn default_drive_id(&self) -> Result<String> {
        Ok(self.drive_info().await?.default_drive_id)
    }

    pub async fn resource_drive_id(&self) -> Result<String> {
        Ok(self.drive_info().await?.resource_drive_id.unwrap())
    }

    pub async fn backup_drive_id(&self) -> Result<String> {
        Ok(self.drive_info().await?.backup_drive_id.unwrap())
    }

    pub async fn space_info(&self) -> Result<SpaceInfo> {
        self.inner.space_info().await
    }

    pub async fn used_size(&self) -> Result<u64> {
        Ok(self.space_info().await?.personal_space_info.used_size)
    }

    pub async fn total_size(&self) -> Result<u64> {
        Ok(self.space_info().await?.personal_space_info.total_size)
    }

    pub async fn available_size(&self) -> Result<u64> {
        let space = self.space_info().await?.personal_space_info;
        Ok(space.total_size - space.used_size)
    }

    pub async fn list_dir(&self, drive_id: &str, parent_id: &str) -> Result<Vec<FileItem>> {
        Ok(self.inner.get_file_list(drive_id, parent_id).await?.items)
    }
}
