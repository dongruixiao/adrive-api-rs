pub mod auth;
pub mod core;
pub mod data;

pub use core::{ADriveCoreAPI, Result};
pub use data::{
    FileEntry, GetDriveInfoResponse as DriveInfo, GetSpaceInfoResponse as SpaceInfo,
    GetUserInfoResponse as UserInfo, Request,
};
use std::path::PathBuf;

pub struct ADriveAPI {
    inner: ADriveCoreAPI,
}

impl Default for ADriveAPI {
    fn default() -> Self {
        Self::new()
    }
}

impl ADriveAPI {
    pub fn new() -> Self {
        Self {
            inner: ADriveCoreAPI::new(),
        }
    }

    pub async fn get_user_info(&self) -> Result<UserInfo> {
        self.inner.get_user_info().await
    }

    pub async fn get_drive_info(&self) -> Result<DriveInfo> {
        self.inner.get_drive_info().await
    }

    pub async fn get_default_drive_id(&self) -> Result<String> {
        Ok(self.get_drive_info().await?.default_drive_id)
    }

    pub async fn get_resource_drive_id(&self) -> Result<String> {
        Ok(self.get_drive_info().await?.resource_drive_id.unwrap())
    }

    pub async fn get_backup_drive_id(&self) -> Result<String> {
        Ok(self.get_drive_info().await?.backup_drive_id.unwrap())
    }

    pub async fn get_space_info(&self) -> Result<SpaceInfo> {
        self.inner.get_space_info().await
    }

    pub async fn get_used_size(&self) -> Result<u64> {
        Ok(self.get_space_info().await?.personal_space_info.used_size)
    }

    pub async fn get_total_size(&self) -> Result<u64> {
        Ok(self.get_space_info().await?.personal_space_info.total_size)
    }

    pub async fn get_available_size(&self) -> Result<u64> {
        let space = self.get_space_info().await?.personal_space_info;
        Ok(space.total_size - space.used_size)
    }

    pub async fn list_files(&self, drive_id: &str, parent_id: &str) -> Result<Vec<FileEntry>> {
        let mut items = Vec::new();
        let mut marker = None;
        loop {
            let resp = self
                .inner
                .list_files(drive_id, parent_id, marker.as_deref())
                .await?;
            items.extend(resp.items);
            marker = resp.next_marker;
            if marker.is_none() || marker.as_deref() == Some("") {
                break;
            }
        }
        Ok(items)
    }

    pub async fn search_files(&self, drive_id: &str, conditions: &str) -> Result<Vec<FileEntry>> {
        let mut items = Vec::new();
        let mut marker = None;
        loop {
            let resp = self
                .inner
                .search_files(drive_id, conditions, marker.as_deref(), Some("name ASC"))
                .await?;
            items.extend(resp.items);
            marker = resp.next_marker;
            if marker.is_none() || marker.as_deref() == Some("") {
                break;
            }
        }
        Ok(items)
    }

    pub async fn list_starred_files(&self, drive_id: &str) -> Result<Vec<FileEntry>> {
        let mut items = Vec::new();
        let mut marker = None;
        loop {
            let resp = self
                .inner
                .list_starred_files(drive_id, marker.as_deref())
                .await?;
            items.extend(resp.items);
            marker = resp.next_marker;
            if marker.is_none() || marker.as_deref() == Some("") {
                break;
            }
        }
        Ok(items)
    }

    pub async fn get_file_by_id(&self, drive_id: &str, file_id: &str) -> Result<FileEntry> {
        self.inner.get_file_by_id(drive_id, file_id).await
    }

    pub async fn get_file_by_path(&self, drive_id: &str, file_path: &str) -> Result<FileEntry> {
        self.inner.get_file_by_path(drive_id, file_path).await
    }

    pub async fn batch_get_files(
        &self,
        drive_id: &str,
        file_ids: &[&str],
    ) -> Result<Vec<FileEntry>> {
        let mut items = Vec::new();
        for chunk in file_ids.chunks(100) {
            let resp = self.inner.batch_get_files(drive_id, chunk).await?;
            items.extend(resp.items);
        }
        Ok(items)
    }

    pub async fn get_download_url(&self, drive_id: &str, file_id: &str) -> Result<String> {
        Ok(self.inner.get_download_url(drive_id, file_id).await?.url)
    }

    pub async fn download_file_directly(
        &self,
        drive_id: &str,
        file_id: &str,
        target_dir: &str,
    ) -> Result<PathBuf> {
        self.inner
            .download_file_directly(drive_id, file_id, target_dir, None)
            .await
    }

    pub async fn download_file_continuously(
        &self,
        drive_id: &str,
        file_id: &str,
        target_dir: &str,
    ) -> Result<PathBuf> {
        self.inner
            .download_file_continuously(drive_id, file_id, target_dir, None)
            .await
    }

    pub async fn download_file_concurrency(
        &self,
        drive_id: &str,
        file_id: &str,
        target_dir: &str,
    ) -> Result<PathBuf> {
        self.inner
            .download_file_concurrency(drive_id, file_id, target_dir, None)
            .await
    }
}
