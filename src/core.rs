use crate::data::{
    AsyncTaskResponse, BatchGetFileDetailByIdRequest, CompleteUploadRequest, CopyFileRequest,
    CreateFileRequest, CreateFileResponse, DeleteFileRequest, DownloadFileRequest, FileEntry,
    FileSearchingRequest, FileSearchingResponse, FileType, FlushUploadUrlRequest,
    FlushUploadUrlResponse, GetAccessTokenResponse, GetAsyncTaskStateRequest,
    GetAsyncTaskStateResponse, GetDownloadUrlByIdRequest, GetDownloadUrlByIdResponse,
    GetDriveInfoRequest, GetDriveInfoResponse, GetFileDetailByIdRequest,
    GetFileDetailByPathRequest, GetFileListRequest, GetFileListResponse, GetFileStarredListRequest,
    GetSpaceInfoRequest, GetSpaceInfoResponse, GetUserInfoRequest, GetUserInfoResponse,
    IfNameExists, ListUploadedPartsRequest, ListUploadedPartsResponse, MoveFileRequest, OrderBy,
    PartInfo, RecycleFileRequest, Request, SortBy, UpdateFileRequest,
};
use crate::utils;
use crate::{auth, constants};

use reqwest::header::HeaderMap;
use std::io::{Read, Seek, SeekFrom};
use std::os::unix::fs::MetadataExt;
use std::sync::{Arc, Mutex, OnceLock};
use std::{error, fs, io::Write, result};

pub type Result<T> = result::Result<T, Box<dyn error::Error>>;
pub static TOKIO_RUNTIME: OnceLock<tokio::runtime::Runtime> = OnceLock::new();

pub struct ADriveCoreAPI {
    auth: auth::Auth,
}

impl Default for ADriveCoreAPI {
    fn default() -> Self {
        Self::new()
    }
}

impl ADriveCoreAPI {
    pub fn new() -> Self {
        Self {
            auth: auth::Auth {},
        }
    }

    pub async fn get_token(&self) -> Result<GetAccessTokenResponse> {
        self.auth.refresh_if_needed().await
    }
    pub async fn get_user_info(&self) -> Result<GetUserInfoResponse> {
        let token = self.auth.refresh_if_needed().await?;
        let resp = GetUserInfoRequest {}
            .dispatch(None, Some(&token.access_token))
            .await?;
        Ok(resp)
    }

    pub async fn get_drive_info(&self) -> Result<GetDriveInfoResponse> {
        let token = self.auth.refresh_if_needed().await?;
        let resp = GetDriveInfoRequest {}
            .dispatch(None, Some(&token.access_token))
            .await?;
        Ok(resp)
    }

    pub async fn get_space_info(&self) -> Result<GetSpaceInfoResponse> {
        let token = self.auth.refresh_if_needed().await?;
        let resp = GetSpaceInfoRequest {}
            .dispatch(None, Some(&token.access_token))
            .await?;
        Ok(resp)
    }

    pub async fn list_files(
        &self,
        drive_id: &str,
        parent_file_id: &str,
        marker: Option<&str>,
    ) -> Result<GetFileListResponse> {
        let token = self.auth.refresh_if_needed().await?;
        let resp = GetFileListRequest::new(
            drive_id,
            parent_file_id,
            marker,
            Some(OrderBy::NameEnhanced),
            Some(SortBy::Asc),
            None,
            None,
        )
        .dispatch(None, Some(&token.access_token))
        .await?;
        Ok(resp)
    }

    pub async fn search_files(
        &self,
        drive_id: &str,
        query: &str,
        marker: Option<&str>,
        order_by: Option<&str>,
    ) -> Result<FileSearchingResponse> {
        let token = self.auth.refresh_if_needed().await?;
        let resp = FileSearchingRequest::new(drive_id, Some(query), marker, order_by)
            .dispatch(None, Some(&token.access_token))
            .await?;
        Ok(resp)
    }

    pub async fn list_starred_files(
        &self,
        drive_id: &str,
        marker: Option<&str>,
    ) -> Result<GetFileListResponse> {
        let token = self.auth.refresh_if_needed().await?;
        let resp = GetFileStarredListRequest::new(drive_id, marker)
            .dispatch(None, Some(&token.access_token))
            .await?;
        Ok(resp)
    }

    pub async fn get_file_by_id(&self, drive_id: &str, file_id: &str) -> Result<FileEntry> {
        let token = self.auth.refresh_if_needed().await?;
        let resp = GetFileDetailByIdRequest::new(drive_id, file_id)
            .dispatch(None, Some(&token.access_token))
            .await?;
        Ok(resp)
    }

    pub async fn get_file_by_path(&self, drive_id: &str, file_path: &str) -> Result<FileEntry> {
        let token = self.auth.refresh_if_needed().await?;
        let resp = GetFileDetailByPathRequest::new(drive_id, file_path)
            .dispatch(None, Some(&token.access_token))
            .await?;
        Ok(resp)
    }

    pub async fn batch_get_files(
        &self,
        drive_id: &str,
        file_ids: &[&str],
    ) -> Result<GetFileListResponse> {
        if file_ids.len() > constants::MAX_BATCH_SIZE {
            return Err("the max batch size should not exceed 100".into());
        }
        let token = self.auth.refresh_if_needed().await?;
        BatchGetFileDetailByIdRequest::new(drive_id, file_ids)
            .dispatch(None, Some(&token.access_token))
            .await
    }

    pub async fn get_download_url(
        &self,
        drive_id: &str,
        file_id: &str,
    ) -> Result<GetDownloadUrlByIdResponse> {
        let token = self.auth.refresh_if_needed().await?;
        GetDownloadUrlByIdRequest::new(drive_id, file_id)
            .dispatch(None, Some(&token.access_token))
            .await
    }

    pub async fn download_file(
        &self,
        drive_id: &str,
        file_id: &str,
        file_handle: &mut fs::File,
        download_url: Option<&str>,
        start: Option<&str>,
        end: Option<&str>,
    ) -> Result<()> {
        let token = self.auth.refresh_if_needed().await?;
        let url = if let Some(url) = download_url {
            url.to_string()
        } else {
            self.get_download_url(drive_id, file_id).await?.url
        };
        let mut headers = HeaderMap::new();
        if start.is_some() || end.is_some() {
            headers.append(
                "Range",
                format!(
                    "bytes={}-{}",
                    start.unwrap_or_default(),
                    end.unwrap_or_default()
                )
                .parse()?,
            );
        }
        let bytes = DownloadFileRequest { url: &url }
            .get_original(Some(headers), Some(&token.access_token))
            .await?
            .bytes()
            .await?;
        Ok(file_handle.write_all(&bytes)?)
    }

    pub async fn download_file2(
        file_handle: Arc<Mutex<fs::File>>,
        download_url: String,
        token: String,
        start: Option<String>,
        end: Option<String>,
    ) {
        let mut headers = HeaderMap::new();
        if start.is_some() || end.is_some() {
            headers.append(
                "Range",
                format!(
                    "bytes={}-{}",
                    start.as_deref().unwrap_or_default(),
                    end.as_deref().unwrap_or_default()
                )
                .parse()
                .unwrap(),
            );
        }
        let bytes = DownloadFileRequest { url: &download_url }
            .get_original(Some(headers), Some(&token))
            .await
            .unwrap()
            .bytes()
            .await
            .unwrap();
        let mut file_handle = file_handle.lock().unwrap();
        let start = start.map_or(0, |v| v.parse::<u64>().unwrap());
        file_handle.seek(SeekFrom::Start(start)).unwrap();
        file_handle.write_all(&bytes).unwrap()
    }

    // 只能创建单层文件夹，dirname 不能是 a/b/c 这种形式
    pub async fn create_folder(
        &self,
        drive_id: &str,
        parent_file_id: &str,
        dir_name: &str,
    ) -> Result<CreateFileResponse> {
        let token = &self.auth.refresh_if_needed().await?;
        CreateFileRequest::new(
            drive_id,
            parent_file_id,
            dir_name,
            FileType::Folder,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .dispatch(None, Some(&token.access_token))
        .await
    }

    pub async fn create_multipart_upload(
        &self,
        drive_id: &str,
        parent_file_id: &str,
        file_name: &str,
        part_info_list: Option<Vec<PartInfo>>,
    ) -> Result<CreateFileResponse> {
        let token = &self.auth.refresh_if_needed().await?;
        CreateFileRequest::new(
            drive_id,
            parent_file_id,
            file_name,
            FileType::File,
            part_info_list,
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .dispatch(None, Some(&token.access_token))
        .await
    }

    pub async fn flush_upload_url(
        &self,
        drive_id: &str,
        file_id: &str,
        upload_id: &str,
        part_number_list: &[u16],
    ) -> Result<FlushUploadUrlResponse> {
        let token = &self.auth.refresh_if_needed().await?;
        FlushUploadUrlRequest::new(drive_id, file_id, upload_id, part_number_list)
            .dispatch(None, Some(&token.access_token))
            .await
    }

    pub async fn list_multipart_uploads(
        &self,
        drive_id: &str,
        file_id: &str,
        upload_id: &str,
        marker: Option<String>,
    ) -> Result<ListUploadedPartsResponse> {
        let token = &self.auth.refresh_if_needed().await?;
        ListUploadedPartsRequest::new(drive_id, file_id, upload_id, marker)
            .dispatch(None, Some(&token.access_token))
            .await
    }

    pub async fn complete_multipart_upload(
        &self,
        drive_id: &str,
        file_id: &str,
        upload_id: &str,
    ) -> Result<FileEntry> {
        let token = &self.auth.refresh_if_needed().await?;
        CompleteUploadRequest::new(drive_id, file_id, upload_id)
            .dispatch(None, Some(&token.access_token))
            .await
    }

    pub async fn upload_part(&self, part_info: &PartInfo, buffer: Vec<u8>) -> Result<()> {
        let token = &self.auth.refresh_if_needed().await?;
        part_info
            .put_original(None, Some(&token.access_token), buffer)
            .await?;
        Ok(())
    }

    pub const PART_SIZE: u64 = 64 * 1024 * 1024; // 64MB

    pub fn create_part_info_list(size: u64) -> Result<Vec<PartInfo>> {
        let count = (size + Self::PART_SIZE - 1) / Self::PART_SIZE;
        let parts = (1..=count)
            .map(|index| PartInfo {
                part_number: index as u16,
                part_size: None,
                upload_url: None,
            })
            .collect();
        Ok(parts)
    }

    pub async fn update_file(
        &self,
        drive_id: &str,
        file_id: &str,
        name: Option<&str>,
        if_name_exists: Option<IfNameExists>,
        starred: Option<bool>,
    ) -> Result<FileEntry> {
        let token = &self.auth.refresh_if_needed().await?;
        UpdateFileRequest::new(drive_id, file_id, name, if_name_exists, starred)
            .dispatch(None, Some(&token.access_token))
            .await
    }

    pub async fn move_file(
        &self,
        drive_id: &str,
        file_id: &str,
        target_parent_id: &str,
        rename: Option<&str>,
    ) -> Result<AsyncTaskResponse> {
        let token = &self.auth.refresh_if_needed().await?;
        MoveFileRequest::new(drive_id, file_id, target_parent_id, rename)
            .dispatch(None, Some(&token.access_token))
            .await
    }

    pub async fn copy_file(
        &self,
        drive_id: &str,
        file_id: &str,
        target_parent_id: &str,
    ) -> Result<AsyncTaskResponse> {
        let token = &self.auth.refresh_if_needed().await?;
        CopyFileRequest::new(drive_id, file_id, target_parent_id)
            .dispatch(None, Some(&token.access_token))
            .await
    }

    pub async fn recycle_file(&self, drive_id: &str, file_id: &str) -> Result<AsyncTaskResponse> {
        let token = &self.auth.refresh_if_needed().await?;
        RecycleFileRequest::new(drive_id, file_id)
            .dispatch(None, Some(&token.access_token))
            .await
    }

    pub async fn delete_file(&self, drive_id: &str, file_id: &str) -> Result<AsyncTaskResponse> {
        let token = &self.auth.refresh_if_needed().await?;
        DeleteFileRequest { drive_id, file_id }
            .dispatch(None, Some(&token.access_token))
            .await
    }

    pub async fn get_async_task_state(&self, task_id: &str) -> Result<GetAsyncTaskStateResponse> {
        let token = &self.auth.refresh_if_needed().await?;
        GetAsyncTaskStateRequest {
            async_task_id: task_id,
        }
        .dispatch(None, Some(&token.access_token))
        .await
    }

    pub async fn check_pre_hash(
        &self,
        drive_id: &str,
        parent_file_id: &str,
        file_name: &str,
        part_info_list: Vec<PartInfo>,
        pre_hash: &str,
        size: u64,
    ) -> Result<CreateFileResponse> {
        let token = &self.auth.refresh_if_needed().await?;
        CreateFileRequest::new(
            drive_id,
            parent_file_id,
            file_name,
            FileType::File,
            Some(part_info_list),
            Some(pre_hash),
            Some(size),
            None,
            None,
            None,
            None,
        )
        .dispatch(None, Some(&token.access_token))
        .await
    }

    pub async fn check_content_hash(
        &self,
        drive_id: &str,
        parent_file_id: &str,
        file_name: &str,
        part_info_list: Vec<PartInfo>,
        content_hash: &str,
        proof_code: &str,
        size: u64,
    ) -> Result<CreateFileResponse> {
        let token = &self.auth.refresh_if_needed().await?;
        CreateFileRequest::new(
            drive_id,
            parent_file_id,
            file_name,
            FileType::File,
            Some(part_info_list),
            None,
            Some(size),
            Some(proof_code),
            Some("v1"),
            Some(content_hash),
            Some("sha1"),
        )
        .dispatch(None, Some(&token.access_token))
        .await
    }

    /*

             checkPreHash
                /     \
    checkContentHash   uploadFile with part_info_list
        /   \
    return  uploadFile with part_info_list

    */

    pub async fn upload_file(
        &self,
        drive_id: &str,
        parent_file_id: &str,
        file_name: &str,
        file: &mut fs::File,
    ) -> Result<()> {
        let file_size = file.metadata()?.size();
        let part_info_list = Self::create_part_info_list(file_size)?;
        let pre_hash = utils::get_pre_hash(file)?;
        let resp = self
            .check_pre_hash(
                drive_id,
                parent_file_id,
                file_name,
                part_info_list.clone(),
                &pre_hash,
                file_size,
            )
            .await?;
        if resp.pre_hash_matched() {
            let content_hash = utils::get_content_hash(file)?;
            let token = self.auth.refresh_if_needed().await?;
            let proof_code = utils::get_proof_code(file, file_size, &token.access_token)?;
            let resp = self
                .check_content_hash(
                    drive_id,
                    parent_file_id,
                    file_name,
                    part_info_list,
                    &content_hash,
                    &proof_code,
                    file_size,
                )
                .await?;
            if resp.content_hash_matched() {
                Ok(())
            } else {
                self.multipart_upload_file(
                    drive_id,
                    parent_file_id,
                    file_name,
                    file_size,
                    file,
                    Some(resp),
                )
                .await
            }
        } else {
            self.multipart_upload_file(
                drive_id,
                parent_file_id,
                file_name,
                file_size,
                file,
                Some(resp),
            )
            .await
        }
    }

    pub async fn multipart_upload_file(
        &self,
        drive_id: &str,
        parent_file_id: &str,
        file_name: &str,
        file_size: u64,
        file: &mut fs::File,
        created_file: Option<CreateFileResponse>,
    ) -> Result<()> {
        let file_id;
        let upload_id;
        let part_info_list_with_upload_url;

        if created_file.is_none() {
            let part_info_list = Self::create_part_info_list(file_size)?;
            let resp = self
                .create_multipart_upload(drive_id, parent_file_id, file_name, Some(part_info_list))
                .await?;
            file_id = resp.file_id();
            upload_id = resp.upload_id();
            part_info_list_with_upload_url = resp.part_info_list();
        } else {
            let response = created_file.unwrap();
            file_id = response.file_id();
            upload_id = response.upload_id();
            part_info_list_with_upload_url = response.part_info_list();
        }

        for part_info in part_info_list_with_upload_url.iter() {
            let mut buffer = Vec::new();
            let pos = (part_info.part_number as u64 - 1) * ADriveCoreAPI::PART_SIZE;
            let _ = file.seek(SeekFrom::Start(pos));
            let _ = file.take(ADriveCoreAPI::PART_SIZE).read_to_end(&mut buffer);
            self.upload_part(part_info, buffer).await?;
        }

        let mut marker = None;
        let mut uploaded = Vec::new();
        loop {
            let resp = self
                .list_multipart_uploads(drive_id, &file_id, &upload_id, marker)
                .await?;
            uploaded.extend(resp.uploaded_parts);
            marker = Some(resp.next_part_number_marker);
            if marker.is_none() || marker.as_deref() == Some("") {
                break;
            }
        }
        assert!(uploaded.len() == part_info_list_with_upload_url.len());
        let _ = self
            .complete_multipart_upload(drive_id, &file_id, &upload_id)
            .await?;
        Ok(())
    }
}
