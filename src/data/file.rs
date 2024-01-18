use super::Request;
use reqwest::Method;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Default, Debug)]
#[serde(rename_all = "snake_case")]
pub enum OrderBy {
    CreatedAt,
    #[default]
    UpdatedAt,
    Size,
    Name,
    NameEnhanced,
}

#[derive(Serialize, Default, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum SortBy {
    #[default]
    Desc,
    Asc,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "lowercase")]
pub enum FileType {
    Folder,
    File,
    #[default]
    #[serde(skip_deserializing)]
    All,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "snake_case")]
pub enum IfNameExists {
    #[default]
    AutoRename,
    Refuse,
    Ignore,
}

#[derive(Debug, Serialize, Default)]
pub struct GetFileListRequest<'a> {
    drive_id: &'a str,
    parent_file_id: &'a str,
    limit: Option<u32>, // 50..=100
    marker: Option<&'a str>,
    order_by: Option<OrderBy>,
    order_direction: Option<SortBy>,
    category: Option<&'a str>, // TODO
    r#type: Option<FileType>,
    video_thumbnail_time: Option<u32>,  // ms
    video_thumbnail_width: Option<u32>, // px
    image_thumbnail_width: Option<u32>, // px
    fields: Option<&'a str>,            // TODO *
}

impl<'a> GetFileListRequest<'a> {
    pub fn new(drive_id: &'a str, parent_file_id: &'a str) -> Self {
        Self {
            drive_id,
            parent_file_id,
            video_thumbnail_time: Some(120000), // ms
            video_thumbnail_width: Some(480),   // px
            image_thumbnail_width: Some(480),   // px
            ..Default::default()
        }
    }
}

impl Request for GetFileListRequest<'_> {
    const URI: &'static str = "/adrive/v1.0/openFile/list";
    const METHOD: reqwest::Method = Method::POST;
    type Response = GetFileListResponse;
}

#[derive(Debug, Deserialize)]
pub struct FileListingItem {
    pub drive_id: String,
    pub file_id: String,
    pub parent_file_id: String,
    pub name: String,
    pub size: Option<u64>,              // TODO folder don't have size
    pub file_extension: Option<String>, // TODO folder don't have file_extension
    pub content_hash: Option<String>,   // TODO folder don't have content_hash
    pub category: Option<String>,       // TODO folder don't have category
    pub r#type: FileType,
    pub thumbnail: Option<String>,
    pub url: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub play_cursor: Option<String>,
    pub video_media_metadata: Option<serde_json::Value>, // TODO
    pub video_preview_metadata: Option<String>,
    pub next_marker: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GetFileListResponse {
    pub items: Vec<FileListingItem>,
}

#[derive(Debug, Serialize, Default)]
pub struct FileSearchingRequest<'a> {
    drive_id: &'a str,
    limit: Option<u32>,
    marker: Option<&'a str>,
    /*
    查询语句，样例：
    固定目录搜索，只搜索一级 parent_file_id = '123'
    精确查询 name = '123'
    模糊匹配 name match "123"
    搜索指定后缀文件 file_extension = 'apk'
    范围查询 created_at < "2019-01-14T00:00:00"
    复合查询：
    type = 'folder' or name = '123'
    parent_file_id = 'root' and name = '123' and category = 'video'
    */
    query: Option<&'a str>,
    /*
    created_at ASC | DESC
    updated_at ASC | DESC
    name ASC | DESC
    size ASC | DESC
    */
    order_by: Option<OrderBy>,
    video_thumbnail_time: Option<u32>,
    video_thumbnail_width: Option<u32>,
    image_thumbnail_width: Option<u32>,
    return_total_count: Option<bool>,
}

impl<'a> FileSearchingRequest<'a> {
    pub fn new(drive_id: &'a str, query: Option<&'a str>) -> Self {
        Self {
            drive_id,
            query,
            video_thumbnail_time: Some(120000), // ms
            video_thumbnail_width: Some(480),   // px
            image_thumbnail_width: Some(480),   // px
            ..Default::default()
        }
    }
}

impl Request for FileSearchingRequest<'_> {
    const URI: &'static str = "/adrive/v1.0/openFile/search";
    const METHOD: reqwest::Method = Method::POST;
    type Response = FileSearchingResponse;
}

#[derive(Debug, Deserialize)]
pub struct FileSearchingItem {
    pub drive_id: String,
    pub file_id: String,
    pub parent_file_id: String,
    pub name: String,
    pub size: Option<u64>,              // TODO folder don't have size
    pub file_extension: Option<String>, // TODO folder don't have file_extension
    pub content_hash: Option<String>,   // TODO folder don't have content_hash
    pub category: Option<String>,       // TODO folder don't have category
    pub r#type: FileType,
    pub thumbnail: Option<String>,
    pub url: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub next_marker: Option<String>,
    pub total_count: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct FileSearchingResponse {
    pub items: Vec<FileSearchingItem>,
}

#[derive(Debug, Serialize, Default)]
pub struct GetFileStarredListRequest<'a> {
    drive_id: &'a str,
    limit: Option<u32>,
    marker: Option<&'a str>,
    r#type: Option<FileType>,
    order_by: Option<OrderBy>,
    order_direction: Option<SortBy>,
    video_thumbnail_time: Option<u32>,
    video_thumbnail_width: Option<u32>,
    image_thumbnail_width: Option<u32>,
}

impl<'a> GetFileStarredListRequest<'a> {
    pub fn new(drive_id: &'a str) -> Self {
        Self {
            drive_id,
            video_thumbnail_time: Some(120000), // ms
            video_thumbnail_width: Some(480),   // px
            image_thumbnail_width: Some(480),   // px
            ..Default::default()
        }
    }
}

impl Request for GetFileStarredListRequest<'_> {
    const URI: &'static str = "/adrive/v1.0/openFile/starredList";
    const METHOD: reqwest::Method = Method::POST;
    type Response = GetFileStarredListResponse;
}

#[derive(Debug, Deserialize)]
pub struct FileStarredItem {
    pub drive_id: String,
    pub file_id: String,
    pub parent_file_id: String,
    pub name: String,
    pub size: Option<u64>,              // TODO folder don't have size
    pub file_extension: Option<String>, // TODO folder don't have file_extension
    pub content_hash: Option<String>,   // TODO folder don't have content_hash
    pub category: Option<String>,       // TODO folder don't have category
    pub r#type: FileType,
    pub thumbnail: Option<String>,
    pub url: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub next_marker: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GetFileStarredListResponse {
    pub items: Vec<FileStarredItem>,
}

#[derive(Debug, Serialize, Default)]
pub struct GetFileDetailByIdRequest<'a> {
    drive_id: &'a str,
    file_id: &'a str,
    video_thumbnail_time: Option<u32>,
    video_thumbnail_width: Option<u32>,
    image_thumbnail_width: Option<u32>,
    fields: Option<&'a str>, // *
}

impl<'a> GetFileDetailByIdRequest<'a> {
    pub fn new(drive_id: &'a str, file_id: &'a str) -> Self {
        Self {
            drive_id,
            file_id,
            video_thumbnail_time: Some(120000), // ms
            video_thumbnail_width: Some(480),   // px
            image_thumbnail_width: Some(480),   // px
            ..Default::default()
        }
    }
}

impl Request for GetFileDetailByIdRequest<'_> {
    const URI: &'static str = "/adrive/v1.0/openFile/get";
    const METHOD: reqwest::Method = Method::POST;
    type Response = GetFileDetailResponse;
}

#[derive(Debug, Deserialize)]
pub struct GetFileDetailResponse {
    pub drive_id: String,
    pub file_id: String,
    pub parent_file_id: String,
    pub name: String,
    pub size: u64,
    pub file_extension: String,
    pub content_hash: String,
    pub category: String,
    pub r#type: FileType,
    pub thumbnail: String,
    pub url: String,
    pub created_at: String,
    pub updated_at: String,
    pub video_media_metadata: serde_json::Value,
}

#[derive(Debug, Serialize, Default)]
pub struct GetFileDetailByPathRequest<'a> {
    drive_id: &'a str,
    file_path: &'a str,
}

impl<'a> GetFileDetailByPathRequest<'a> {
    pub fn new(drive_id: &'a str, file_path: &'a str) -> Self {
        Self {
            drive_id,
            file_path,
        }
    }
}

impl Request for GetFileDetailByPathRequest<'_> {
    const URI: &'static str = "/adrive/v1.0/openFile/get_by_path";
    const METHOD: reqwest::Method = Method::POST;
    type Response = GetFileDetailResponse;
}

#[derive(Debug, Serialize, Default)]
pub struct BatchGetFileDetailByIdRequest<'a> {
    pub file_list: Vec<GetFileDetailByIdRequest<'a>>,
}

impl Request for BatchGetFileDetailByIdRequest<'_> {
    const URI: &'static str = "/adrive/v1.0/openFile/batch/get";
    const METHOD: reqwest::Method = Method::POST;
    type Response = BatchGetFileDetailByIdResponse;
}

#[derive(Debug, Deserialize)]
pub struct BatchGetFileDetailByIdResponse {
    pub items: Vec<GetFileDetailResponse>,
}

#[derive(Debug, Serialize, Default)]
pub struct GetDownloadUrlByIdRequest<'a> {
    drive_id: &'a str,
    file_id: &'a str,
    expire_sec: Option<u32>,
}

impl<'a> GetDownloadUrlByIdRequest<'a> {
    pub fn new(drive_id: &'a str, file_id: &'a str) -> Self {
        Self {
            drive_id,
            file_id,
            expire_sec: Some(900), //s
        }
    }
}

impl Request for GetDownloadUrlByIdRequest<'_> {
    const URI: &'static str = "/adrive/v1.0/openFile/getDownloadUrl";
    const METHOD: reqwest::Method = Method::POST;
    type Response = GetDownloadUrlByIdResponse;
}

#[derive(Debug, Deserialize)]
pub struct GetDownloadUrlByIdResponse {
    pub url: String,
    pub expiration: String,
    pub method: String,
}

#[derive(Debug, Serialize)]
pub struct DownloadFileRequest<'a> {
    #[serde(skip_serializing)]
    pub url: &'a str,
}

impl Request for DownloadFileRequest<'_> {
    const URI: &'static str = "";
    const METHOD: reqwest::Method = Method::GET;
    type Response = ();

    fn path_join(&self) -> crate::Result<reqwest::Url> {
        Ok(reqwest::Url::parse(self.url)?)
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PartInfo {
    pub part_number: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub part_size: Option<u32>,
}

impl Request for PartInfo {
    const URI: &'static str = "";
    const METHOD: reqwest::Method = Method::PUT;
    type Response = ();

    fn path_join(&self) -> crate::Result<url::Url> {
        Ok(reqwest::Url::parse(self.upload_url.as_ref().unwrap())?)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StreamsInfo {
    pub content_hash: Option<String>,
    pub content_hash_name: Option<String>,
    pub proof_version: Option<String>,
    pub proof_code: Option<String>,
    pub content_md5: Option<String>,
    pub pre_hash: Option<String>,
    pub size: Option<u64>,
    pub part_info_list: Option<Vec<PartInfo>>,
}

#[derive(Debug, Serialize, Default)]
pub struct GetUploadUrlRequest<'a> {
    pub drive_id: &'a str,
    pub parent_file_id: &'a str,
    pub name: &'a str,
    pub r#type: FileType,
    pub check_name_mode: IfNameExists,
    pub streams_info: Option<StreamsInfo>,
    pub pre_hash: Option<&'a str>,
    pub size: Option<u64>,
    pub content_hash: Option<&'a str>,
    pub content_hash_name: Option<&'a str>,
    pub proof_code: Option<&'a str>,
    pub local_created_at: Option<&'a str>,
    pub local_modified_at: Option<&'a str>,
    pub part_info_list: Option<Vec<PartInfo>>,
}

impl<'a> GetUploadUrlRequest<'a> {
    pub fn new(
        drive_id: &'a str,
        parent_file_id: &'a str,
        name: &'a str,
        r#type: FileType,
        part_info_list: Option<Vec<PartInfo>>,
    ) -> Self {
        Self {
            drive_id,
            parent_file_id,
            name,
            r#type,
            part_info_list,
            ..Default::default()
        }
    }
}

impl Request for GetUploadUrlRequest<'_> {
    const URI: &'static str = "/adrive/v1.0/openFile/create";
    const METHOD: reqwest::Method = Method::POST;
    type Response = GetUploadUrlResponse;
}

#[derive(Debug, Deserialize)]
pub struct GetUploadUrlResponse {
    pub drive_id: String,
    pub file_id: String,
    pub status: Option<String>,
    pub parent_file_id: String,
    pub upload_id: Option<String>,
    pub file_name: String,
    pub available: Option<bool>,
    pub exist: Option<bool>,
    pub rapid_upload: Option<bool>,
    pub part_info_list: Option<Vec<PartInfo>>,
}

#[derive(Debug, Serialize, Default)]
pub struct FlushUploadUrlRequest<'a> {
    pub drive_id: &'a str,
    pub file_id: &'a str,
    pub upload_id: &'a str,
    pub part_info_list: Vec<PartInfo>,
}

impl<'a> FlushUploadUrlRequest<'a> {
    pub fn new(
        drive_id: &'a str,
        file_id: &'a str,
        upload_id: &'a str,
        part_number_list: &[u16],
    ) -> Self {
        Self {
            drive_id,
            file_id,
            upload_id,
            part_info_list: part_number_list
                .iter()
                .map(|part_number| PartInfo {
                    part_number: *part_number,
                    ..Default::default()
                })
                .collect(),
        }
    }
}

impl Request for FlushUploadUrlRequest<'_> {
    const URI: &'static str = "/adrive/v1.0/openFile/getUploadUrl";
    const METHOD: reqwest::Method = Method::POST;
    type Response = FlushUploadUrlResponse;
}

#[derive(Debug, Deserialize)]
pub struct FlushUploadUrlResponse {
    pub drive_id: String,
    pub file_id: String,
    pub upload_id: String,
    pub create_at: String,
    pub part_info_list: Vec<PartInfo>,
}

#[derive(Debug, Serialize, Default)]
pub struct ListUploadedPartsRequest<'a> {
    pub drive_id: &'a str,
    pub file_id: &'a str,
    pub upload_id: &'a str,
    pub part_number_marker: Option<u16>,
}

impl<'a> ListUploadedPartsRequest<'a> {
    pub fn new(drive_id: &'a str, file_id: &'a str, upload_id: &'a str) -> Self {
        Self {
            drive_id,
            file_id,
            upload_id,
            ..Default::default()
        }
    }
}

impl Request for ListUploadedPartsRequest<'_> {
    const URI: &'static str = "/adrive/v1.0/openFile/listUploadedParts";
    const METHOD: reqwest::Method = Method::POST;
    type Response = serde_json::Value;
}

#[derive(Debug, Deserialize)]
pub struct UploadedParts {
    pub etag: String,
    pub part_number: u16,
    pub part_size: u32,
}

#[derive(Debug, Deserialize)]
pub struct ListUploadedPartsResponse {
    pub file_id: String,
    pub upload_id: String,
    #[serde(rename = "parallelUpload")]
    pub parallel_upload: bool,
    pub uploaded_parts: Vec<UploadedParts>,
    pub next_part_number_marker: String,
}

#[derive(Debug, Serialize, Default)]
pub struct CompleteUploadRequest<'a> {
    pub drive_id: &'a str,
    pub file_id: &'a str,
    pub upload_id: &'a str,
}

impl<'a> CompleteUploadRequest<'a> {
    pub fn new(drive_id: &'a str, file_id: &'a str, upload_id: &'a str) -> Self {
        Self {
            drive_id,
            file_id,
            upload_id,
        }
    }
}

impl Request for CompleteUploadRequest<'_> {
    const URI: &'static str = "/adrive/v1.0/openFile/complete";
    const METHOD: reqwest::Method = Method::POST;
    type Response = CompleteUploadResponse;
}

#[derive(Debug, Deserialize)]
pub struct CompleteUploadResponse {
    pub drive_id: String,
    pub file_id: String,
    pub name: String,
    pub size: u64,
    pub file_extension: String,
    pub content_hash: String,
    pub category: String,
    pub r#type: FileType,
    pub thumbnail: Option<String>,
    pub url: Option<String>,
    pub download_url: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Default)]
pub struct UpdateFileRequest<'a> {
    pub drive_id: &'a str,
    pub file_id: &'a str,
    pub name: Option<&'a str>,
    pub check_name_mode: Option<IfNameExists>,
    pub starred: Option<bool>,
}

impl<'a> UpdateFileRequest<'a> {
    pub fn new(
        drive_id: &'a str,
        file_id: &'a str,
        name: Option<&'a str>,
        check_name_mode: Option<IfNameExists>,
        starred: Option<bool>,
    ) -> Self {
        Self {
            drive_id,
            file_id,
            name,
            check_name_mode,
            starred,
        }
    }
}

impl Request for UpdateFileRequest<'_> {
    const URI: &'static str = "/adrive/v1.0/openFile/update";
    const METHOD: reqwest::Method = Method::POST;
    type Response = UpdateFileResponse;
}

#[derive(Debug, Deserialize)]
pub struct UpdateFileResponse {
    pub drive_id: String,
    pub file_id: String,
    pub name: String,
    pub size: u64,
    pub file_extension: String,
    pub content_hash: String,
    pub category: String,
    pub r#type: FileType,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Default)]
pub struct MoveFileRequest<'a> {
    pub drive_id: &'a str,
    pub file_id: &'a str,
    pub to_parent_file_id: &'a str,
    pub check_name_mode: Option<IfNameExists>,
    pub new_name: Option<&'a str>,
}

impl<'a> MoveFileRequest<'a> {
    pub fn new(drive_id: &'a str, file_id: &'a str, to_parent_file_id: &'a str) -> Self {
        Self {
            drive_id,
            file_id,
            to_parent_file_id,
            check_name_mode: Some(IfNameExists::AutoRename),
            ..Default::default()
        }
    }
}

impl Request for MoveFileRequest<'_> {
    const URI: &'static str = "/adrive/v1.0/openFile/move";
    const METHOD: reqwest::Method = Method::POST;
    type Response = MoveFileResponse;
}

#[derive(Debug, Deserialize)]
pub struct MoveFileResponse {
    pub drive_id: String,
    pub file_id: String,
    pub async_task_id: Option<String>,
    pub exit: Option<bool>,
}

#[derive(Debug, Serialize, Default)]
pub struct CopyFileRequest<'a> {
    pub drive_id: &'a str,
    pub file_id: &'a str,
    pub to_drive_id: Option<&'a str>,
    pub to_parent_file_id: &'a str,
    pub auto_rename: Option<bool>,
}

impl<'a> CopyFileRequest<'a> {
    pub fn new(drive_id: &'a str, file_id: &'a str, to_parent_file_id: &'a str) -> Self {
        Self {
            drive_id,
            file_id,
            to_parent_file_id,
            to_drive_id: Some(drive_id),
            auto_rename: Some(true),
        }
    }
}

impl Request for CopyFileRequest<'_> {
    const URI: &'static str = "/adrive/v1.0/openFile/copy";
    const METHOD: reqwest::Method = Method::POST;
    type Response = CopyFileResponse;
}

#[derive(Debug, Deserialize)]
pub struct CopyFileResponse {
    pub drive_id: String,
    pub file_id: String,
    pub async_task_id: Option<String>,
}

#[derive(Debug, Serialize, Default)]
pub struct MoveFileToRecycleBinRequest<'a> {
    pub drive_id: &'a str,
    pub file_id: &'a str,
}

impl<'a> MoveFileToRecycleBinRequest<'a> {
    pub fn new(drive_id: &'a str, file_id: &'a str) -> Self {
        Self { drive_id, file_id }
    }
}

impl Request for MoveFileToRecycleBinRequest<'_> {
    const URI: &'static str = "/adrive/v1.0/openFile/recyclebin/trash";
    const METHOD: reqwest::Method = Method::POST;
    type Response = MoveFileToRecycleBinResponse;
}

#[derive(Debug, Deserialize)]
pub struct MoveFileToRecycleBinResponse {
    pub drive_id: String,
    pub file_id: String,
    pub async_task_id: Option<String>,
}

#[derive(Debug, Serialize, Default)]
pub struct DeleteFileRequest<'a> {
    pub drive_id: &'a str,
    pub file_id: &'a str,
}

impl Request for DeleteFileRequest<'_> {
    const URI: &'static str = "/adrive/v1.0/openFile/delete";
    const METHOD: reqwest::Method = Method::POST;
    type Response = DeleteFileResponse;
}

#[derive(Debug, Deserialize)]
pub struct DeleteFileResponse {
    pub drive_id: Option<String>,
    pub file_id: String,
    pub async_task_id: Option<String>,
}

#[derive(Debug, Serialize, Default)]
pub struct GetAsyncTaskStateRequest<'a> {
    pub async_task_id: &'a str,
}

impl Request for GetAsyncTaskStateRequest<'_> {
    const URI: &'static str = "/adrive/v1.0/openFile/async_task/get";
    const METHOD: reqwest::Method = Method::POST;
    type Response = GetAsyncTaskStateResponse;
}

#[derive(Debug, Deserialize)]
pub enum AsyncTaskState {
    Succeed,
    Running,
    Failed,
}

impl From<String> for AsyncTaskState {
    fn from(s: String) -> Self {
        match s.as_str() {
            "Succeed" => AsyncTaskState::Succeed,
            "Running" => AsyncTaskState::Running,
            "Failed" => AsyncTaskState::Failed,
            _ => panic!("invalid async task state"),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct GetAsyncTaskStateResponse {
    pub state: AsyncTaskState,
    pub async_task_id: String,
}
