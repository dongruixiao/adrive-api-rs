use axum::http::Error;
use base64::prelude::*;
use reqwest::header::HeaderMap;
use sha1_smol::Sha1;

use crate::data::{
    AsyncTaskResponse, BatchGetFileDetailByIdRequest, CompleteUploadRequest, CopyFileRequest,
    CreateFileRequest, CreateFileResponse, DeleteFileRequest, DownloadFileRequest, FileEntry,
    FileSearchingRequest, FileSearchingResponse, FlushUploadUrlRequest, FlushUploadUrlResponse,
    GetAsyncTaskStateRequest, GetAsyncTaskStateResponse, GetDownloadUrlByIdRequest,
    GetDownloadUrlByIdResponse, GetDriveInfoRequest, GetDriveInfoResponse,
    GetFileDetailByIdRequest, GetFileDetailByPathRequest, GetFileListRequest, GetFileListResponse,
    GetFileStarredListRequest, GetSpaceInfoRequest, GetSpaceInfoResponse, GetUserInfoRequest,
    GetUserInfoResponse, IfNameExists, ListUploadedPartsRequest, ListUploadedPartsResponse,
    MoveFileRequest, OrderBy, PartInfo, RecycleFileRequest, Request, SortBy, UpdateFileRequest,
};

use crate::auth;
use crate::data::FileType;
use async_recursion::async_recursion;
use std::cmp;
use std::io::{Read, Seek, SeekFrom};
use std::os::unix::fs::MetadataExt;
use std::sync::{Arc, Mutex, OnceLock};
use std::{error, fs, io::Write, path::PathBuf, result};

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
        if file_ids.len() > 100 {
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

    const CONCURRENCY: usize = 10;

    fn runtime() -> &'static tokio::runtime::Runtime {
        TOKIO_RUNTIME.get_or_init(|| {
            tokio::runtime::Builder::new_multi_thread()
                .worker_threads(Self::CONCURRENCY)
                .enable_all()
                .build()
                .unwrap()
        })
    }

    fn ensure_dirs(dir: &str) -> Result<PathBuf> {
        fs::create_dir_all(dir)?;
        let path = fs::canonicalize(dir)?;
        Ok(path)
    }

    pub async fn download_file_directly(
        &self,
        drive_id: &str,
        file_id: &str,
        target_dir: &str,
        file_name: Option<&str>,
    ) -> Result<PathBuf> {
        let token = self.auth.refresh_if_needed().await?;
        let url = self.get_download_url(drive_id, file_id).await?.url;
        let dst_path = if let Some(file_name) = file_name {
            Self::ensure_dirs(target_dir)?.join(file_name)
        } else {
            let file_name = self.get_file_by_id(drive_id, file_id).await?.name;
            Self::ensure_dirs(target_dir)?.join(file_name)
        };
        let bytes = DownloadFileRequest { url: &url }
            .get_original(None, Some(&token.access_token))
            .await?
            .bytes()
            .await?;
        fs::File::create(&dst_path)?.write_all(&bytes)?;
        Ok(dst_path)
    }

    pub async fn download_file_continuously(
        &self,
        drive_id: &str,
        file_id: &str,
        target_dir: &str,
        file_name: Option<&str>,
    ) -> Result<PathBuf> {
        let token = self.auth.refresh_if_needed().await?;
        let url = self.get_download_url(drive_id, file_id).await?.url;
        let detail = self.get_file_by_id(drive_id, file_id).await?;
        let dst_path = Self::ensure_dirs(target_dir)?.join(file_name.unwrap_or(&detail.name));

        if dst_path.exists() {
            let mut file = fs::OpenOptions::new()
                .write(true)
                .append(true)
                .open(&dst_path)?;
            let mut headers = HeaderMap::new();
            headers.append(
                "Range",
                format!("bytes={}-", file.metadata()?.len()).parse()?,
            );

            let mut resp = DownloadFileRequest { url: &url }
                .get_original(Some(headers), Some(&token.access_token))
                .await?;
            while let Some(chunk) = resp.chunk().await? {
                file.write_all(&chunk)?;
            }
        } else {
            let mut file = fs::File::create(&dst_path)?;
            let mut resp = DownloadFileRequest { url: &url }
                .get_original(None, Some(&token.access_token))
                .await?;
            while let Some(chunk) = resp.chunk().await? {
                file.write_all(&chunk)?;
            }
        }
        Ok(dst_path)
    }

    pub async fn download_file_concurrency(
        &self,
        drive_id: &str,
        file_id: &str,
        target_dir: &str,
        file_name: Option<&str>,
    ) -> Result<PathBuf> {
        let token = self.auth.refresh_if_needed().await?;
        let url = self.get_download_url(drive_id, file_id).await?.url;
        let detail = self.get_file_by_id(drive_id, file_id).await?;
        let dst_path = Self::ensure_dirs(target_dir)?.join(file_name.unwrap_or(&detail.name));

        let file = Arc::new(Mutex::new(fs::File::create(&dst_path)?));
        let mut offset = 0_u64;
        let chunk = 100 * 1024 * 1024_u64;
        let mut futures = Vec::new();
        loop {
            let mut headers = HeaderMap::new();
            if offset + chunk - 1 > detail.size.unwrap() {
                headers.append("Range", format!("bytes={}-", offset).parse()?);
            } else {
                headers.append(
                    "Range",
                    format!("bytes={}-{}", offset, offset + chunk - 1).parse()?,
                );
            };
            let url = url.clone();
            let token = token.access_token.clone();
            let file = Arc::clone(&file);
            let future = Self::runtime().spawn(async move {
                println!("{:#?}", headers);
                let bytes = DownloadFileRequest { url: &url }
                    .get_original(Some(headers), Some(&token))
                    .await
                    .unwrap()
                    .bytes()
                    .await
                    .unwrap();
                let mut file = file.lock().unwrap();
                file.seek(SeekFrom::Start(offset)).unwrap();
                file.write(&bytes).unwrap()
            });
            futures.push(future);
            offset += chunk;
            if offset > detail.size.unwrap() {
                break;
            }
        }
        for future in futures {
            println!("over");
            future.await?;
        }
        Ok(dst_path)
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

    pub async fn create_file_upload(
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

    pub async fn list_uploaded_parts(
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

    pub async fn complete_file_upload(
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

    fn get_pre_hash(file: &mut fs::File) -> Result<String> {
        // TODO 1024?
        let mut buffer = vec![0u8; 1024];
        let count = file.read(&mut buffer)?;
        let data = &buffer[..count];
        let mut hasher = Sha1::new();
        hasher.update(data);
        Ok(hasher.hexdigest().to_uppercase())
    }

    pub async fn check_pre_hash(
        &self,
        drive_id: &str,
        parent_file_id: &str,
        file_name: &str,
        pre_hash: &str,
        size: u64,
    ) -> Result<bool> {
        let token = &self.auth.refresh_if_needed().await?;
        let resp = CreateFileRequest::new(
            drive_id,
            parent_file_id,
            file_name,
            FileType::File,
            None,
            Some(pre_hash),
            Some(size),
            None,
            None,
            None,
            None,
        )
        .dispatch(None, Some(&token.access_token))
        .await?;

        match resp {
            CreateFileResponse::FileCreated {
                drive_id,
                parent_file_id,
                file_name,
                ..
            } => {
                // file.seek(SeekFrom::Start(0))?;
                // return self
                //     .upload_file_with_check(
                //         &drive_id,
                //         &parent_file_id,
                //         &file_name,
                //         file,
                //         true,
                //         true,
                //     )
                //     .await;
                Ok(false)
            }
            CreateFileResponse::PreHashMatched { code } => {
                if code == "PreHashMatched".to_string() {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
        }
    }

    fn get_content_hash(file: &mut fs::File) -> Result<String> {
        let mut hasher = Sha1::new();
        let mut buffer = vec![0u8; 10 * 1024];
        loop {
            let count = file.read(&mut buffer)?;
            if count == 0 {
                break;
            }
            let data = &buffer[..count];
            hasher.update(data);
        }
        Ok(hasher.hexdigest().to_uppercase())
    }

    fn get_proof_code(file: &mut fs::File, size: u64, token: &str) -> Result<String> {
        if size <= 0 {
            return Ok(String::from(""));
        }
        let digest = md5::compute(token);
        let hex = format!("{:x}", digest);
        let uint = u64::from_str_radix(&hex[..16], 16)?;

        let start = uint % size;
        let end = cmp::min(start + 8, size);

        let mut buf = vec![0u8; (end - start) as usize];
        file.seek(SeekFrom::Start(start))?;

        file.read_exact(&mut buf)?;
        Ok(BASE64_STANDARD.encode(&buf))
    }

    pub async fn check_content_hash(
        &self,
        drive_id: &str,
        parent_file_id: &str,
        file_name: &str,
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
            None,
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

    #[async_recursion]
    pub async fn upload_file_with_check(
        &self,
        drive_id: &str,
        parent_file_id: &str,
        file_name: &str,
        file: &mut fs::File,
        pre_hash_checked: bool,
        content_hash_checked: bool,
    ) -> Result<()> {
        if !pre_hash_checked {
            let pre_hash = Self::get_pre_hash(file)?;
            let resp = self
                .check_pre_hash(
                    drive_id,
                    parent_file_id,
                    file_name,
                    &pre_hash,
                    file.metadata()?.size(),
                )
                .await?;
            match resp {
                CreateFileResponse::FileCreated {
                    drive_id,
                    parent_file_id,
                    file_name,
                    ..
                } => {
                    file.seek(SeekFrom::Start(0))?;
                    return self
                        .upload_file_with_check(
                            &drive_id,
                            &parent_file_id,
                            &file_name,
                            file,
                            true,
                            true,
                        )
                        .await;
                }
                CreateFileResponse::PreHashMatched { code } => {
                    return self
                        .upload_file_with_check(
                            drive_id,
                            parent_file_id,
                            file_name,
                            file,
                            true,
                            false,
                        )
                        .await
                }
            }
        }
        if !content_hash_checked {
            let content_hash = Self::get_content_hash(file)?;
            let size = file.metadata()?.size() as u64;
            let token = &self.auth.refresh_if_needed().await?;
            let proof_code = Self::get_proof_code(file, size, &token.access_token)?;
            let resp = self
                .check_content_hash(
                    drive_id,
                    parent_file_id,
                    file_name,
                    &content_hash,
                    &proof_code,
                    size,
                )
                .await?;
            match resp {
                CreateFileResponse::FileCreated { rapid_upload, .. } => {
                    if rapid_upload == Some(true) {
                        return Ok(());
                    } else {
                        return self
                            .upload_file_with_check(
                                drive_id,
                                parent_file_id,
                                file_name,
                                file,
                                true,
                                true,
                            )
                            .await;
                    }
                }
                _ => return Err("".into()),
            }
        }
        let resp = self
            .multipart_upload_file(drive_id, parent_file_id, file_name, file)
            .await?;
        Ok(resp)
    }

    pub async fn multipart_upload_file(
        &self,
        drive_id: &str,
        parent_file_id: &str,
        file_name: &str,
        file: &mut fs::File,
    ) -> Result<()> {
        let file_size = file.metadata()?.size();
        let part_info_list = Self::create_part_info_list(file_size)?;
        let resp = self
            .create_file_upload(drive_id, parent_file_id, file_name, Some(part_info_list))
            .await?;

        match resp {
            CreateFileResponse::FileCreated {
                file_id,
                upload_id,
                part_info_list,
                ..
            } => {
                let part_info_list = part_info_list.unwrap();
                for part_info in part_info_list.iter() {
                    let mut buffer = Vec::new();
                    let pos = (part_info.part_number as u64 - 1) * ADriveCoreAPI::PART_SIZE;
                    let _ = file.seek(SeekFrom::Start(pos));
                    let _ = file.take(ADriveCoreAPI::PART_SIZE).read_to_end(&mut buffer);
                    self.upload_part(part_info, buffer).await?;
                }

                let mut marker = None;
                let mut uploaded = Vec::new();
                loop {
                    let upload_id = upload_id.as_ref().unwrap();
                    let resp = self
                        .list_uploaded_parts(drive_id, &file_id, &upload_id, marker)
                        .await?;
                    uploaded.extend(resp.uploaded_parts);
                    marker = Some(resp.next_part_number_marker);
                    if marker.is_none() || marker.as_deref() == Some("") {
                        break;
                    }
                }
                assert!(uploaded.len() == part_info_list.len());
                Ok(())
            }
            _ => Err("".into()),
        }
    }
}
