use crate::data::{
    BatchGetFileDetailByIdRequest, BatchGetFileDetailByIdResponse, CompleteUploadRequest,
    CompleteUploadResponse, CopyFileRequest, CopyFileResponse, DeleteFileRequest,
    DeleteFileResponse, DownloadFileRequest, FileSearchingRequest, FileSearchingResponse,
    FlushUploadUrlRequest, FlushUploadUrlResponse, GetAsyncTaskStateRequest,
    GetAsyncTaskStateResponse, GetDownloadUrlByIdRequest, GetDownloadUrlByIdResponse,
    GetDriveInfoRequest, GetDriveInfoResponse, GetFileDetailByIdRequest,
    GetFileDetailByPathRequest, GetFileDetailResponse, GetFileListRequest, GetFileListResponse,
    GetFileStarredListRequest, GetFileStarredListResponse, GetSpaceInfoRequest,
    GetSpaceInfoResponse, GetUploadUrlRequest, GetUploadUrlResponse, GetUserInfoRequest,
    GetUserInfoResponse, ListUploadedPartsRequest, MoveFileRequest, MoveFileResponse,
    MoveFileToRecycleBinRequest, MoveFileToRecycleBinResponse, PartInfo, Request,
    UpdateFileRequest, UpdateFileResponse,
};

use crate::auth;
use crate::data::FileType;
use std::io::{Read, Seek, SeekFrom};
use std::sync::{Arc, Mutex, OnceLock};
use std::{error, fs, io::Write, path::PathBuf, result};

pub type Result<T> = result::Result<T, Box<dyn error::Error>>;
pub static TOKIO_RUNTIME: OnceLock<tokio::runtime::Runtime> = OnceLock::new();

pub struct ADriveCoreAPI {
    auth: auth::Auth,
}

impl ADriveCoreAPI {
    pub fn new() -> Self {
        Self {
            auth: auth::Auth {},
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
        let token = self.auth.refresh_if_needed().await?;
        let resp = GetSpaceInfoRequest {}
            .dispatch(None, Some(&token.access_token))
            .await?;
        Ok(resp)
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
        let token = self.auth.refresh_if_needed().await?;
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
        let file = Arc::new(Mutex::new(fs::File::create(dst_path)?));
        let detail = GetFileDetailByIdRequest::new(drive_id, file_id)
            .dispatch(None, Some(&token.access_token))
            .await?;
        let mut from = 0;
        let offset = 10 * 1024 * 1024 - 1; // 10MB
        let mut tasks = Vec::new();
        loop {
            let to = from + offset;
            let url = url.clone();
            let token = token.access_token.clone();
            let file_clone = Arc::clone(&file);
            if from <= detail.size {
                let to = if to > detail.size { detail.size } else { to };
                let task = Self::runtime().spawn(async move {
                    let _ = Self::write_chunk(
                        &url,
                        &token,
                        file_clone,
                        Some(&from.to_string()),
                        Some(&to.to_string()),
                        detail.size,
                    )
                    .await;
                });
                tasks.push(task);
                from = to + 1;
            } else {
                break;
            }
        }
        for task in tasks {
            task.await?;
        }

        Ok(())
    }

    fn runtime() -> &'static tokio::runtime::Runtime {
        TOKIO_RUNTIME.get_or_init(|| {
            tokio::runtime::Builder::new_multi_thread()
                .worker_threads(1)
                .enable_all()
                .build()
                .unwrap()
        })
    }

    async fn write_chunk(
        url: &str,
        token: &str,
        file: Arc<Mutex<fs::File>>,
        from: Option<&str>,
        to: Option<&str>,
        size: u64,
    ) -> Result<usize> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "Range",
            format!(
                "bytes={}-{}",
                from.unwrap(),
                to.and_then(|v| {
                    if v.parse::<u64>().unwrap() == size {
                        Some("")
                    } else {
                        to
                    }
                })
                .unwrap()
            )
            .parse()?,
        );
        let bytes = DownloadFileRequest { url: &url }
            .get_original(Some(headers), Some(&token))
            .await
            .unwrap()
            .bytes()
            .await?;
        let mut file_guard = file.lock().unwrap();
        file_guard.seek(SeekFrom::Start(from.unwrap().parse::<u64>()?))?;
        let count = file_guard.write(&bytes)?;
        Ok(count)
    }

    // 只能创建单层文件夹
    pub async fn create_dir(
        &self,
        drive_id: &str,
        parent_file_id: &str,
        dir_name: &str,
    ) -> Result<GetUploadUrlResponse> {
        let token = &self.auth.refresh_if_needed().await?;
        GetUploadUrlRequest::new(drive_id, parent_file_id, dir_name, FileType::Folder, None)
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
            return GetUploadUrlRequest::new(
                drive_id,
                parent_file_id,
                file_name,
                FileType::File,
                None,
            )
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
    ) -> Result<serde_json::Value> {
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

    async fn upload_part(
        part_info: &PartInfo,
        src_path: &str,
        token: &str,
        part_size: Option<u64>,
    ) -> Result<()> {
        let mut file = fs::File::open(src_path)?;
        let mut buffer = Vec::new();
        if part_size.is_none() {
            let _ = file.read_to_end(&mut buffer);
        } else {
            let pos = (part_info.part_number as u64 - 1) * part_size.unwrap();
            let _ = file.seek(SeekFrom::Start(pos));
            let _ = file.take(part_size.unwrap()).read_to_end(&mut buffer);
        }
        println!("{:#?}", part_info);
        let _ = PartInfo::reqwest_client()
            .put(part_info.upload_url.as_ref().unwrap())
            .body(buffer)
            .bearer_auth(token)
            .send()
            .await?;
        println!("upload part success");
        Ok(())
    }

    pub async fn upload_file(
        &self,
        drive_id: &str,
        parent_file_id: &str,
        src_path: &str,
    ) -> Result<()> {
        let token = &self.auth.refresh_if_needed().await?;
        let src_path = PathBuf::from(src_path);
        if src_path.is_dir() {
            return Err("src_path is a directory".into());
        }
        let name = src_path.file_name().unwrap().to_str().unwrap();
        let resp = GetUploadUrlRequest::new(drive_id, parent_file_id, name, FileType::File, None)
            .dispatch(None, Some(&token.access_token))
            .await?;

        let access_token = token.access_token.clone();
        for part_info in resp.part_info_list.unwrap().iter() {
            Self::upload_part(part_info, src_path.to_str().unwrap(), &access_token, None).await?;
        }

        let _resp = CompleteUploadRequest::new(drive_id, &resp.file_id, &resp.upload_id.unwrap())
            .dispatch(None, Some(&access_token))
            .await?;
        Ok(())
    }

    fn get_part_info_list(size: u64) -> Vec<PartInfo> {
        const DEFAULT_PART_SIZE: u64 = 64 * 1024 * 1024; // 64MB
        let count = (size + DEFAULT_PART_SIZE - 1) / DEFAULT_PART_SIZE;
        let part_info_list = (1..=count)
            .map(|index| PartInfo {
                part_number: index as u16,
                part_size: None,
                upload_url: None,
            })
            .collect();
        part_info_list
    }

    pub async fn multiparts_upload_file(
        &self,
        drive_id: &str,
        parent_file_id: &str,
        src_path: &str,
    ) -> Result<CompleteUploadResponse> {
        let token = &self.auth.refresh_if_needed().await?;
        let src_path = PathBuf::from(src_path);
        if src_path.is_dir() {
            return Err("src_path is a directory".into());
        }
        let name = src_path.file_name().unwrap().to_str().unwrap();
        let size = fs::File::open(&src_path)?.metadata()?.len();
        let part_info_list = Self::get_part_info_list(size);
        let resp = GetUploadUrlRequest::new(
            drive_id,
            parent_file_id,
            name,
            FileType::File,
            Some(part_info_list),
        )
        .dispatch(None, Some(&token.access_token))
        .await?;

        println!("{:#?}", resp);

        let src_path = src_path.to_str().unwrap().to_string();
        // let handles: Vec<JoinHandle<()>> = resp
        //     .part_info_list
        //     .unwrap()
        //     .into_iter()
        //     .map(|part_info| {
        //         println!("{:#?}", part_info.part_number);
        //         let src_path = src_path.clone();
        //         let token = token.access_token.clone();
        //         Self::runtime().spawn_blocking(|| async move {
        //             Self::upload_part(&part_info, &src_path, &token, Some(64 * 1024 * 1024))
        //                 .await
        //                 .unwrap()
        //         })
        //     })
        //     // })
        //     .collect();
        // for handle in handles {
        //     handle.await?;
        // }

        // 因服务端使用流式计算 SHA1 值，单个文件的分片需要串行上传，不支持多个分片并行上传
        let mut futures = Vec::new();
        for part_info in resp.part_info_list.unwrap().into_iter() {
            println!("{:#?}", part_info.part_number);

            let src_path = src_path.clone();
            let token = token.access_token.clone();
            let task = Self::runtime().spawn(async move {
                Self::upload_part(&part_info, &src_path, &token, Some(64 * 1024 * 1024))
                    .await
                    .unwrap();
            });
            futures.push(task);
        }
        println!("futures len: {}", futures.len());
        for future in futures {
            future.await?;
        }
        let upload_id = resp.upload_id.unwrap();
        let r = ListUploadedPartsRequest::new(drive_id, &resp.file_id, &upload_id)
            .dispatch(None, Some(&token.access_token))
            .await?;
        println!("{:#?}", r);
        let resp = CompleteUploadRequest::new(drive_id, &resp.file_id, &upload_id)
            .dispatch(None, Some(&token.access_token))
            .await?;
        Ok(resp)
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
