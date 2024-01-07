pub mod auth;
pub mod constants;
pub mod data_structures;

use data_structures::{
    BatchGetFileDetailByIdRequest, BatchGetFileDetailByIdResponse, CompleteUploadRequest,
    CompleteUploadResponse, CopyFileRequest, CopyFileResponse, DeleteFileRequest,
    DeleteFileResponse, DownloadFileRequest, FileSearchingRequest, FileSearchingResponse,
    FlushUploadUrlRequest, FlushUploadUrlResponse, GetAsyncTaskStateRequest,
    GetAsyncTaskStateResponse, GetDownloadUrlByIdRequest, GetDownloadUrlByIdResponse,
    GetDriveInfoRequest, GetDriveInfoResponse, GetFileDetailByIdRequest,
    GetFileDetailByPathRequest, GetFileDetailResponse, GetFileListRequest, GetFileListResponse,
    GetFileStarredListRequest, GetFileStarredListResponse, GetSpaceInfoRequest,
    GetSpaceInfoResponse, GetUploadUrlRequest, GetUploadUrlResponse, GetUserInfoRequest,
    GetUserInfoResponse, ListUploadedPartsRequest, ListUploadedPartsResponse, MoveFileRequest,
    MoveFileResponse, MoveFileToRecycleBinRequest, MoveFileToRecycleBinResponse, Request,
    UpdateFileRequest, UpdateFileResponse,
};

use crate::data_structures::FileType;
use std::sync::OnceLock;
use std::{error, fs, io::Write, path::PathBuf, result};

type Result<T> = result::Result<T, Box<dyn error::Error>>;

pub struct ADriveAPI<'a> {
    auth: auth::Auth<'a>,
}

impl Default for ADriveAPI<'_> {
    fn default() -> Self {
        Self::new()
    }
}

pub static TOKIO_RUNTIME: OnceLock<tokio::runtime::Runtime> = OnceLock::new();

impl ADriveAPI<'_> {
    pub fn new() -> Self {
        Self {
            auth: auth::Auth::new(),
        }
    }

    pub async fn user_info(&self) -> Result<GetUserInfoResponse> {
        let token = self.auth.refresh_if_needed().await?;
        let resp = GetUserInfoRequest {}
            .dispatch(None, Some(&token.access_token))
            .await?;
        Ok(resp)
    }

    pub async fn drive_info(&self) -> Result<GetDriveInfoResponse> {
        let token = self.auth.refresh_if_needed().await?;
        let resp = GetDriveInfoRequest {}
            .dispatch(None, Some(&token.access_token))
            .await?;
        Ok(resp)
    }

    pub async fn space_info(&self) -> Result<GetSpaceInfoResponse> {
        let token: data_structures::GetAccessTokenResponse = self.auth.refresh_if_needed().await?;
        let resp = GetSpaceInfoRequest {}
            .dispatch(None, Some(&token.access_token))
            .await?;
        Ok(resp)
    }

    pub async fn get_default_drive_id(&self) -> Result<String> {
        Ok(self.drive_info().await?.default_drive_id)
    }

    pub async fn get_resource_drive_id(&self) -> Result<String> {
        Ok(self.drive_info().await?.resource_drive_id.unwrap())
    }

    pub async fn get_backup_drive_id(&self) -> Result<String> {
        Ok(self.drive_info().await?.backup_drive_id.unwrap())
    }

    pub async fn get_file_list(
        &self,
        drive_id: &str,
        parent_file_id: &str,
    ) -> Result<GetFileListResponse> {
        let token = self.auth.refresh_if_needed().await?;
        let resp = GetFileListRequest::new(drive_id, parent_file_id)
            .dispatch(None, Some(&token.access_token))
            .await?;
        Ok(resp)
    }

    pub async fn search_for_file(
        &mut self,
        drive_id: &str,
        query: &str,
    ) -> Result<FileSearchingResponse> {
        let token = self.auth.refresh_if_needed().await?;
        let resp = FileSearchingRequest::new(drive_id, Some(query))
            .dispatch(None, Some(&token.access_token))
            .await?;
        Ok(resp)
    }

    pub async fn get_starred_file_list(
        &self,
        drive_id: &str,
    ) -> Result<GetFileStarredListResponse> {
        let token = self.auth.refresh_if_needed().await?;
        let resp = GetFileStarredListRequest::new(drive_id)
            .dispatch(None, Some(&token.access_token))
            .await?;
        Ok(resp)
    }

    pub async fn get_file_detail_by_id(
        &self,
        drive_id: &str,
        file_id: &str,
    ) -> Result<GetFileDetailResponse> {
        let token = self.auth.refresh_if_needed().await?;
        GetFileDetailByIdRequest::new(drive_id, file_id)
            .dispatch(None, Some(&token.access_token))
            .await
    }

    pub async fn get_file_detail_by_path(
        &self,
        drive_id: &str,
        path: &str,
    ) -> Result<GetFileDetailResponse> {
        let token = self.auth.refresh_if_needed().await?;
        GetFileDetailByPathRequest::new(drive_id, path)
            .dispatch(None, Some(&token.access_token))
            .await
    }

    pub async fn batch_file_detail_by_id(
        &self,
        drive_ids: &[&str],
        file_ids: &[&str],
    ) -> Result<BatchGetFileDetailByIdResponse> {
        let token = self.auth.refresh_if_needed().await?;
        let mut file_list = Vec::new();
        let zipper = drive_ids.iter().zip(file_ids.iter());
        for (drive_id, file_id) in zipper {
            file_list.push(GetFileDetailByIdRequest::new(drive_id, file_id));
        }
        BatchGetFileDetailByIdRequest { file_list }
            .dispatch(None, Some(&token.access_token))
            .await
    }

    pub async fn get_download_url_by_file_id(
        &self,
        drive_id: &str,
        file_id: &str,
    ) -> Result<GetDownloadUrlByIdResponse> {
        let token = self.auth.refresh_if_needed().await?;
        GetDownloadUrlByIdRequest::new(drive_id, file_id)
            .dispatch(None, Some(&token.access_token))
            .await
    }

    pub async fn download_small_file(
        &self,
        drive_id: &str,
        file_id: &str,
        dst_path: &str,
    ) -> Result<()> {
        let token = &self.auth.refresh_if_needed().await?;
        let url = self
            .get_download_url_by_file_id(drive_id, file_id)
            .await?
            .url;
        let bytes = DownloadFileRequest { url: &url }
            .get_original(None, Some(&token.access_token))
            .await?
            .bytes()
            .await?;
        let dst_path = PathBuf::from(dst_path);
        if dst_path.is_dir() {
            return Err("dst_path is a directory".into());
        }
        if dst_path.parent().is_none() {
            return Err("dst_path has no parent".into());
        } else {
            fs::create_dir_all(dst_path.parent().unwrap())?;
        }
        let mut file = fs::File::create(dst_path)?;
        let _ = file.write_all(&bytes);
        Ok(())
    }

    pub async fn download_big_file(
        &self,
        drive_id: &str,
        file_id: &str,
        dst_path: &str,
    ) -> Result<()> {
        let token = &self.auth.refresh_if_needed().await?;
        let url = self
            .get_download_url_by_file_id(drive_id, file_id)
            .await?
            .url;
        let _stream = DownloadFileRequest { url: &url }
            .get_original(None, Some(&token.access_token))
            .await?
            .bytes_stream();
        let dst_path = PathBuf::from(dst_path);
        if dst_path.is_dir() {
            return Err("dst_path is a directory".into());
        }
        if dst_path.parent().is_none() {
            return Err("dst_path has no parent".into());
        } else {
            fs::create_dir_all(dst_path.parent().unwrap())?;
        }
        let mut _file = fs::File::create(dst_path)?;
        let from = "100";
        let to = "200";
        let url = self
            .get_download_url_by_file_id(drive_id, file_id)
            .await?
            .url
            .to_owned();
        // let task = Self::runtime().spawn(async move {
        //     self.write_chunk(&url, &mut _file, Some(from), Some(to))
        //         .await;
        // });
        Ok(())
    }

    fn runtime() -> &'static tokio::runtime::Runtime {
        TOKIO_RUNTIME.get_or_init(|| {
            tokio::runtime::Builder::new_multi_thread()
                .worker_threads(10)
                .enable_all()
                .build()
                .unwrap()
        })
    }

    pub async fn write_chunk(
        &self,
        url: &str,
        file: &mut fs::File,
        from: Option<&str>,
        to: Option<&str>,
    ) -> Result<usize> {
        let token = &self.auth.refresh_if_needed().await?;
        let mut headers = reqwest::header::HeaderMap::new();
        if from.is_some() || to.is_some() {
            headers.insert(
                "Range",
                format!("bytes={}-{}", from.unwrap_or("0"), to.unwrap_or("")).parse()?,
            );
        }
        let bytes = DownloadFileRequest { url: &url }
            .get_original(Some(headers), Some(&token.access_token))
            .await
            .unwrap()
            .bytes()
            .await?;
        Ok(file.write(&bytes)?)
    }

    // 只能创建单层文件夹
    pub async fn create_dir(
        &self,
        drive_id: &str,
        parent_file_id: &str,
        dir_name: &str,
    ) -> Result<GetUploadUrlResponse> {
        let token = &self.auth.refresh_if_needed().await?;
        GetUploadUrlRequest::new(drive_id, parent_file_id, dir_name, FileType::Folder)
            .dispatch(None, Some(&token.access_token))
            .await
    }

    // src path 只能是单层文件夹路径或者单层文件路径
    pub async fn get_upload_url(
        &self,
        drive_id: &str,
        parent_file_id: &str,
        file_name: &str,
    ) -> Result<GetUploadUrlResponse> {
        let metadata = fs::metadata(file_name)?;
        if metadata.is_dir() {
            self.create_dir(drive_id, parent_file_id, file_name).await
        } else if metadata.is_file() {
            let token = &self.auth.refresh_if_needed().await?;
            return GetUploadUrlRequest::new(drive_id, parent_file_id, file_name, FileType::File)
                .dispatch(None, Some(&token.access_token))
                .await;
        } else {
            return Err("src_path is not a file or a directory".into());
        }
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
    ) -> Result<ListUploadedPartsResponse> {
        let token = &self.auth.refresh_if_needed().await?;
        ListUploadedPartsRequest::new(drive_id, file_id, upload_id)
            .dispatch(None, Some(&token.access_token))
            .await
    }

    pub async fn complete_upload(
        &self,
        drive_id: &str,
        file_id: &str,
        upload_id: &str,
    ) -> Result<CompleteUploadResponse> {
        let token = &self.auth.refresh_if_needed().await?;
        CompleteUploadRequest::new(drive_id, file_id, upload_id)
            .dispatch(None, Some(&token.access_token))
            .await
    }

    pub async fn starred_file(&self, drive_id: &str, file_id: &str) -> Result<UpdateFileResponse> {
        let token = &self.auth.refresh_if_needed().await?;
        UpdateFileRequest::new(drive_id, file_id, None, None, Some(true))
            .dispatch(None, Some(&token.access_token))
            .await
    }

    pub async fn unstarred_file(
        &self,
        drive_id: &str,
        file_id: &str,
    ) -> Result<UpdateFileResponse> {
        let token = &self.auth.refresh_if_needed().await?;
        UpdateFileRequest::new(drive_id, file_id, None, None, Some(false))
            .dispatch(None, Some(&token.access_token))
            .await
    }

    pub async fn rename_file(
        &self,
        drive_id: &str,
        file_id: &str,
        new_name: &str,
    ) -> Result<UpdateFileResponse> {
        let token = &self.auth.refresh_if_needed().await?;
        UpdateFileRequest::new(drive_id, file_id, Some(new_name), None, None)
            .dispatch(None, Some(&token.access_token))
            .await
    }

    pub async fn move_file(
        &self,
        drive_id: &str,
        file_id: &str,
        dst_parent_id: &str,
    ) -> Result<MoveFileResponse> {
        let token = &self.auth.refresh_if_needed().await?;
        MoveFileRequest::new(drive_id, file_id, dst_parent_id)
            .dispatch(None, Some(&token.access_token))
            .await
    }

    pub async fn copy_file(
        &self,
        drive_id: &str,
        file_id: &str,
        dst_parent_id: &str,
    ) -> Result<CopyFileResponse> {
        let token = &self.auth.refresh_if_needed().await?;
        CopyFileRequest::new(drive_id, file_id, dst_parent_id)
            .dispatch(None, Some(&token.access_token))
            .await
    }

    pub async fn put_in_recylebin(
        &self,
        drive_id: &str,
        file_id: &str,
    ) -> Result<MoveFileToRecycleBinResponse> {
        let token = &self.auth.refresh_if_needed().await?;
        MoveFileToRecycleBinRequest::new(drive_id, file_id)
            .dispatch(None, Some(&token.access_token))
            .await
    }

    pub async fn delete_file(&self, drive_id: &str, file_id: &str) -> Result<DeleteFileResponse> {
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
}
