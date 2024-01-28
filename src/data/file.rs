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
    Desc,
    #[default]
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
pub struct ListFilesRequest<'a> {
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

impl<'a> ListFilesRequest<'a> {
    pub fn new(
        drive_id: &'a str,
        parent_file_id: &'a str,
        marker: Option<&'a str>,
        order_by: Option<OrderBy>,
        order_direction: Option<SortBy>,
        category: Option<&'a str>,
        r#type: Option<FileType>,
    ) -> Self {
        Self {
            drive_id,
            parent_file_id,
            marker,
            order_by,
            order_direction,
            category,
            r#type,
            ..Default::default()
        }
    }
}

impl Request for ListFilesRequest<'_> {
    const URI: &'static str = "/adrive/v1.0/openFile/list";
    const METHOD: reqwest::Method = Method::POST;
    type Response = ListFilesResponse;
}
#[derive(Debug, Deserialize)]
pub struct VideoMediaMetadata {
    pub width: u32,
    pub height: u32,
    pub duration: Option<String>,
    pub video_media_video_stream: Vec<serde_json::Value>,
    pub video_media_audio_stream: Vec<serde_json::Value>,
    pub time: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct FileEntry {
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
    pub download_url: Option<String>, // TODO complete file needed
    pub created_at: String,
    pub updated_at: String,
    pub play_cursor: Option<String>,
    pub video_media_metadata: Option<VideoMediaMetadata>,
    pub video_preview_metadata: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
pub struct ListFilesResponse {
    pub items: Vec<FileEntry>,
    #[serde(default)]
    pub next_marker: Option<String>,
    #[serde(default)]
    pub total_count: Option<u32>,
}

#[derive(Debug, Serialize, Default)]
pub struct SearchFilesRequest<'a> {
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
    order_by: Option<&'a str>,
    video_thumbnail_time: Option<u32>,
    video_thumbnail_width: Option<u32>,
    image_thumbnail_width: Option<u32>,
    return_total_count: Option<bool>,
}

impl<'a> SearchFilesRequest<'a> {
    pub fn new(
        drive_id: &'a str,
        query: Option<&'a str>,
        marker: Option<&'a str>,
        order_by: Option<&'a str>,
    ) -> Self {
        Self {
            drive_id,
            query,
            marker,
            order_by,
            return_total_count: Some(true),
            ..Default::default()
        }
    }
}

impl Request for SearchFilesRequest<'_> {
    const URI: &'static str = "/adrive/v1.0/openFile/search";
    const METHOD: reqwest::Method = Method::POST;
    type Response = ListFilesResponse;
}

#[derive(Debug, Serialize, Default)]
pub struct ListStarredFilesRequest<'a> {
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

impl<'a> ListStarredFilesRequest<'a> {
    pub fn new(drive_id: &'a str, marker: Option<&'a str>) -> Self {
        Self {
            drive_id,
            marker,
            order_by: Some(OrderBy::NameEnhanced),
            order_direction: Some(SortBy::Asc),
            ..Default::default()
        }
    }
}

impl Request for ListStarredFilesRequest<'_> {
    const URI: &'static str = "/adrive/v1.0/openFile/starredList";
    const METHOD: reqwest::Method = Method::POST;
    type Response = ListFilesResponse;
}

#[derive(Debug, Serialize, Default)]
pub struct GetFileByIdRequest<'a> {
    drive_id: &'a str,
    file_id: &'a str,
    video_thumbnail_time: Option<u32>,
    video_thumbnail_width: Option<u32>,
    image_thumbnail_width: Option<u32>,
    fields: Option<&'a str>, // *
}

impl<'a> GetFileByIdRequest<'a> {
    pub fn new(drive_id: &'a str, file_id: &'a str) -> Self {
        Self {
            drive_id,
            file_id,
            ..Default::default()
        }
    }
}

impl Request for GetFileByIdRequest<'_> {
    const URI: &'static str = "/adrive/v1.0/openFile/get";
    const METHOD: reqwest::Method = Method::POST;
    type Response = FileEntry;
}

#[derive(Debug, Serialize, Default)]
pub struct GetFileByPathRequest<'a> {
    drive_id: &'a str,
    file_path: &'a str,
}

impl<'a> GetFileByPathRequest<'a> {
    pub fn new(drive_id: &'a str, file_path: &'a str) -> Self {
        Self {
            drive_id,
            file_path,
        }
    }
}

impl Request for GetFileByPathRequest<'_> {
    const URI: &'static str = "/adrive/v1.0/openFile/get_by_path";
    const METHOD: reqwest::Method = Method::POST;
    type Response = FileEntry;
}

#[derive(Debug, Serialize, Default)]
pub struct BatchGetFilesRequest<'a> {
    file_list: Vec<GetFileByIdRequest<'a>>,
    video_thumbnail_time: Option<u32>,
    video_thumbnail_width: Option<u32>,
    image_thumbnail_width: Option<u32>,
}

impl<'a> BatchGetFilesRequest<'a> {
    pub fn new(drive_id: &'a str, file_ids: &[&'a str]) -> Self {
        let file_list = file_ids
            .iter()
            .map(|file_id| GetFileByIdRequest::new(drive_id, file_id))
            .collect();
        Self {
            file_list,
            ..Default::default()
        }
    }
}
impl Request for BatchGetFilesRequest<'_> {
    const URI: &'static str = "/adrive/v1.0/openFile/batch/get";
    const METHOD: reqwest::Method = Method::POST;
    type Response = ListFilesResponse;
}

#[derive(Debug, Serialize, Default)]
pub struct GetDownloadUrlRequest<'a> {
    drive_id: &'a str,
    file_id: &'a str,
    expire_sec: Option<u32>, // default 900s
}

impl<'a> GetDownloadUrlRequest<'a> {
    pub fn new(drive_id: &'a str, file_id: &'a str) -> Self {
        Self {
            drive_id,
            file_id,
            expire_sec: Some(900), //s
        }
    }
}

impl Request for GetDownloadUrlRequest<'_> {
    const URI: &'static str = "/adrive/v1.0/openFile/getDownloadUrl";
    const METHOD: reqwest::Method = Method::POST;
    type Response = GetDownloadUrlResponse;
}

#[derive(Debug, Deserialize)]
pub struct GetDownloadUrlResponse {
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

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
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

    fn path_join(&self) -> crate::Result<reqwest::Url> {
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
pub struct CreateFileRequest<'a> {
    drive_id: &'a str,
    parent_file_id: &'a str,
    name: &'a str,
    r#type: FileType,
    check_name_mode: IfNameExists,
    streams_info: Option<StreamsInfo>,
    pre_hash: Option<&'a str>,
    size: Option<u64>,
    content_hash: Option<&'a str>,
    content_hash_name: Option<&'a str>,
    proof_code: Option<&'a str>,
    proof_version: Option<&'a str>,
    local_created_at: Option<&'a str>,
    local_modified_at: Option<&'a str>,
    part_info_list: Option<Vec<PartInfo>>,
}

impl<'a> CreateFileRequest<'a> {
    pub fn new(
        drive_id: &'a str,
        parent_file_id: &'a str,
        name: &'a str,
        r#type: FileType,
        part_info_list: Option<Vec<PartInfo>>,
        pre_hash: Option<&'a str>,          // content_hash needed
        size: Option<u64>,                  // content_hash needed
        proof_code: Option<&'a str>,        // content_hash needed
        proof_version: Option<&'a str>,     // content_hash needed, default v1
        content_hash: Option<&'a str>,      // content_hash needed
        content_hash_name: Option<&'a str>, // content_hash needed, default sha1
    ) -> Self {
        Self {
            drive_id,
            parent_file_id,
            name,
            r#type,
            part_info_list,
            pre_hash,
            size,
            proof_code,
            proof_version,
            content_hash,
            content_hash_name,
            ..Default::default()
        }
    }
}

impl Request for CreateFileRequest<'_> {
    const URI: &'static str = "/adrive/v1.0/openFile/create";
    const METHOD: reqwest::Method = Method::POST;
    type Response = CreateFileResponse;
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum CreateFileResponse {
    FileCreated {
        drive_id: String,
        file_id: String,
        status: Option<String>,
        parent_file_id: String,
        upload_id: Option<String>,
        file_name: String,
        available: Option<bool>,
        exist: Option<bool>,
        rapid_upload: Option<bool>,
        part_info_list: Option<Vec<PartInfo>>,
    },
    PreHashMatched {
        code: String,
    },
}

impl CreateFileResponse {
    pub fn pre_hash_matched(&self) -> bool {
        match self {
            CreateFileResponse::PreHashMatched { code } => code == "PreHashMatched",
            _ => false,
        }
    }

    pub fn content_hash_matched(&self) -> bool {
        match self {
            CreateFileResponse::FileCreated { rapid_upload, .. } => rapid_upload.unwrap_or(false),
            _ => false,
        }
    }

    pub fn upload_id(&self) -> String {
        match self {
            CreateFileResponse::FileCreated { upload_id, .. } => upload_id.clone().unwrap(),
            _ => panic!("upload_id not found"),
        }
    }

    pub fn file_id(&self) -> String {
        match self {
            CreateFileResponse::FileCreated { file_id, .. } => file_id.clone(),
            _ => panic!("file_id not found"),
        }
    }

    pub fn part_info_list(&self) -> Vec<PartInfo> {
        match self {
            CreateFileResponse::FileCreated { part_info_list, .. } => {
                part_info_list.clone().unwrap()
            }
            _ => panic!("part_info_list not found"),
        }
    }
}

#[derive(Debug, Serialize, Default)]
pub struct FlushUploadUrlRequest<'a> {
    drive_id: &'a str,
    file_id: &'a str,
    upload_id: &'a str,
    part_info_list: Vec<PartInfo>,
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
    drive_id: &'a str,
    file_id: &'a str,
    upload_id: &'a str,
    part_number_marker: Option<String>,
}

impl<'a> ListUploadedPartsRequest<'a> {
    pub fn new(
        drive_id: &'a str,
        file_id: &'a str,
        upload_id: &'a str,
        part_number_marker: Option<String>,
    ) -> Self {
        Self {
            drive_id,
            file_id,
            upload_id,
            part_number_marker,
        }
    }
}

impl Request for ListUploadedPartsRequest<'_> {
    const URI: &'static str = "/adrive/v1.0/openFile/listUploadedParts";
    const METHOD: reqwest::Method = Method::POST;
    type Response = ListUploadedPartsResponse;
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
    drive_id: &'a str,
    file_id: &'a str,
    upload_id: &'a str,
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
    type Response = FileEntry;
}

#[derive(Debug, Serialize, Default)]
pub struct UpdateFileRequest<'a> {
    drive_id: &'a str,
    file_id: &'a str,
    name: Option<&'a str>,
    check_name_mode: Option<IfNameExists>,
    starred: Option<bool>,
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
    type Response = FileEntry;
}

#[derive(Debug, Serialize, Default)]
pub struct MoveFileRequest<'a> {
    drive_id: &'a str,
    file_id: &'a str,
    to_parent_file_id: &'a str,
    check_name_mode: Option<IfNameExists>,
    new_name: Option<&'a str>,
}

impl<'a> MoveFileRequest<'a> {
    pub fn new(
        drive_id: &'a str,
        file_id: &'a str,
        to_parent_file_id: &'a str,
        rename: Option<&'a str>,
    ) -> Self {
        Self {
            drive_id,
            file_id,
            to_parent_file_id,
            check_name_mode: Some(IfNameExists::AutoRename),
            new_name: rename,
        }
    }
}

impl Request for MoveFileRequest<'_> {
    const URI: &'static str = "/adrive/v1.0/openFile/move";
    const METHOD: reqwest::Method = Method::POST;
    type Response = AsyncTaskResponse;
}

#[derive(Debug, Deserialize)]
pub struct AsyncTaskResponse {
    pub drive_id: String,
    pub file_id: String,
    pub async_task_id: Option<String>,
    pub exit: Option<bool>,
}

#[derive(Debug, Serialize, Default)]
pub struct CopyFileRequest<'a> {
    drive_id: &'a str,
    file_id: &'a str,
    to_drive_id: Option<&'a str>,
    to_parent_file_id: &'a str,
    auto_rename: Option<bool>,
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
    type Response = AsyncTaskResponse;
}

#[derive(Debug, Serialize, Default)]
pub struct RecycleFileRequest<'a> {
    drive_id: &'a str,
    file_id: &'a str,
}

impl<'a> RecycleFileRequest<'a> {
    pub fn new(drive_id: &'a str, file_id: &'a str) -> Self {
        Self { drive_id, file_id }
    }
}

impl Request for RecycleFileRequest<'_> {
    const URI: &'static str = "/adrive/v1.0/openFile/recyclebin/trash";
    const METHOD: reqwest::Method = Method::POST;
    type Response = AsyncTaskResponse;
}

#[derive(Debug, Serialize, Default)]
pub struct DeleteFileRequest<'a> {
    pub drive_id: &'a str,
    pub file_id: &'a str,
}

impl Request for DeleteFileRequest<'_> {
    const URI: &'static str = "/adrive/v1.0/openFile/delete";
    const METHOD: reqwest::Method = Method::POST;
    type Response = AsyncTaskResponse;
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
