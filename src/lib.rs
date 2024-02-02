#![allow(dead_code)]
#![allow(clippy::too_many_arguments)]

mod auth;
mod constants;
mod core;
mod data;
mod self_hosting;
mod utils;

use anyhow::anyhow;
pub use auth::Auth;
pub use core::{ADriveCoreAPI, Result};
use data::{
    FileEntry, GetDriveInfoResponse as DriveInfo, GetSpaceInfoResponse as SpaceInfo,
    GetUserInfoResponse as UserInfo, IfNameExists,
};
pub use self_hosting::app as self_hosting_app;
use std::{
    fs,
    path::PathBuf,
    sync::{Arc, Mutex, OnceLock},
};

static TOKIO_RUNTIME: OnceLock<tokio::runtime::Runtime> = OnceLock::new();

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
        for chunk in file_ids.chunks(constants::MAX_BATCH_SIZE) {
            let resp = self.inner.batch_get_files(drive_id, chunk).await?;
            items.extend(resp.items);
        }
        Ok(items)
    }

    pub async fn get_download_url(&self, drive_id: &str, file_id: &str) -> Result<String> {
        Ok(self.inner.get_download_url(drive_id, file_id).await?.url)
    }

    pub async fn download_file(
        &self,
        drive_id: &str,
        file_id: &str,
        target_dir: &str,
        rename_as: Option<&str>,
    ) -> Result<()> {
        let target_dir = utils::ensure_dirs(target_dir)?;
        let detail = self.get_file_by_id(drive_id, file_id).await?;
        let dst_path = target_dir.join(rename_as.unwrap_or(&detail.name));
        let download_url = self.get_download_url(drive_id, file_id).await?;
        let mut file = fs::OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(&dst_path)?;
        let mut start = file.metadata().map_or(0, |m| m.len());
        let file_size = detail.size.unwrap();
        loop {
            if start >= file_size {
                break;
            }
            let end = start + constants::CHUNK_SIZE - 1;
            let end = if end >= file_size {
                None
            } else {
                Some(end.to_string())
            };
            self.inner
                .download_file(
                    drive_id,
                    file_id,
                    &mut file,
                    Some(&download_url),
                    Some(&start.to_string()),
                    end.as_deref(),
                )
                .await?;
            start += constants::CHUNK_SIZE;
        }
        Ok(())
    }

    fn runtime() -> &'static tokio::runtime::Runtime {
        TOKIO_RUNTIME.get_or_init(|| {
            tokio::runtime::Builder::new_multi_thread()
                .worker_threads(constants::MAX_CONCURRENCY)
                .enable_all()
                .build()
                .unwrap()
        })
    }

    pub async fn concurrent_download_file(
        &self,
        drive_id: &str,
        file_id: &str,
        target_dir: &str,
        rename_as: Option<&str>,
    ) -> Result<()> {
        let target_dir = utils::ensure_dirs(target_dir)?;
        let detail = self.get_file_by_id(drive_id, file_id).await?;
        let dst_path = target_dir.join(rename_as.unwrap_or(&detail.name));
        let download_url = self.get_download_url(drive_id, file_id).await?;

        let file_handle = Arc::new(Mutex::new(fs::File::create(&dst_path)?));
        let file_size = detail.size.unwrap();
        let mut start = 0_u64;
        let mut futures = Vec::new();
        let token = self.inner.get_token().await?.refresh_token;
        loop {
            if start >= file_size {
                break;
            }
            let end = start + constants::CHUNK_SIZE - 1;
            let end = if end >= file_size {
                None
            } else {
                Some(end.to_string())
            };
            let download_url = download_url.clone();
            let token = token.clone();
            let file_handle = Arc::clone(&file_handle);
            let future = Self::runtime().spawn(ADriveCoreAPI::download_file2(
                file_handle,
                download_url,
                token,
                Some(start.to_string()),
                end,
            ));
            futures.push(future);
            start += constants::CHUNK_SIZE;
        }
        for future in futures {
            future.await?;
        }
        Ok(())
    }

    pub async fn create_folder(
        &self,
        drive_id: &str,
        parent_id: &str,
        name: &str,
    ) -> Result<String> {
        let resp = self.inner.create_folder(drive_id, parent_id, name).await?;
        Ok(resp.file_id())
    }

    pub async fn upload_file(
        &self,
        drive_id: &str,
        parent_id: &str,
        file_path: &str,
    ) -> Result<()> {
        let file_path = PathBuf::from(file_path);
        if file_path.is_dir() {
            return Err(anyhow!("file_path is a directory"));
        }

        let file_name = file_path.file_name().unwrap().to_str().unwrap();
        let mut file = fs::File::open(&file_path)?;
        self.inner
            .upload_file(drive_id, parent_id, file_name, &mut file)
            .await
    }

    pub async fn star_file(&self, drive_id: &str, file_id: &str) -> Result<FileEntry> {
        self.inner
            .update_file(drive_id, file_id, None, None, Some(true))
            .await
    }

    pub async fn unstar_file(&self, drive_id: &str, file_id: &str) -> Result<FileEntry> {
        self.inner
            .update_file(drive_id, file_id, None, None, Some(false))
            .await
    }

    pub async fn rename_file(
        &self,
        drive_id: &str,
        file_id: &str,
        rename_as: &str,
    ) -> Result<FileEntry> {
        self.inner
            .update_file(
                drive_id,
                file_id,
                Some(rename_as),
                Some(IfNameExists::AutoRename),
                None,
            )
            .await
    }

    pub async fn move_file(
        &self,
        drive_id: &str,
        file_id: &str,
        target_parent_id: &str,
        rename_as: Option<&str>,
    ) -> Result<()> {
        self.inner
            .move_file(drive_id, file_id, target_parent_id, rename_as)
            .await?;
        Ok(())
    }

    pub async fn copy_file(
        &self,
        drive_id: &str,
        file_id: &str,
        target_parent_id: &str,
    ) -> Result<String> {
        let resp = self
            .inner
            .copy_file(drive_id, file_id, target_parent_id)
            .await?;
        Ok(resp.file_id)
    }

    pub async fn recycle_file(&self, drive_id: &str, file_id: &str) -> Result<()> {
        self.inner.recycle_file(drive_id, file_id).await?;
        Ok(())
    }

    pub async fn delete_file(&self, drive_id: &str, file_id: &str) -> Result<()> {
        self.inner.delete_file(drive_id, file_id).await?;
        Ok(())
    }
}
