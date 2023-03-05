use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum FileType {
    Folder,
    File,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "snake_case")]
pub enum IfNameExists {
    #[default]
    AutoRename,
    Refuse,
    Ignore,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PartInfo {
    pub part_number: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_upload_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
}

#[derive(Serialize, Debug)]
struct BaseCreationRequest<'a> {
    drive_id: &'a str,
    name: &'a str,
    #[serde(rename = "parent_file_id")]
    parent_id: &'a str,
    #[serde(rename = "check_name_mode")]
    #[serde(default)]
    if_name_exists: IfNameExists,
    r#type: FileType,
}

#[derive(Serialize, Debug)]
pub struct CreateDirRequest<'a> {
    #[serde(flatten)]
    _base: BaseCreationRequest<'a>,
}

impl<'a> CreateDirRequest<'a> {
    pub fn new(
        drive_id: &'a str,
        parent_id: &'a str,
        name: &'a str,
        if_name_exists: Option<IfNameExists>,
    ) -> Self {
        Self {
            _base: BaseCreationRequest {
                drive_id,
                parent_id,
                name,
                if_name_exists: if_name_exists.unwrap_or_default(),
                r#type: FileType::Folder,
            },
        }
    }
}

#[derive(Serialize, Debug)]
pub struct MatchPreHashRequest<'a> {
    size: u64,
    part_info_list: Vec<PartInfo>,
    create_scene: &'a str,
    device_name: &'a str,
    pre_hash: &'a str,

    #[serde(flatten)]
    _base: BaseCreationRequest<'a>,
}

impl<'a> MatchPreHashRequest<'a> {
    pub fn new(
        drive_id: &'a str,
        parent_id: &'a str,
        name: &'a str,
        if_name_exists: Option<IfNameExists>,
        size: u64,
        part_info_list: Vec<PartInfo>,
        create_scene: Option<&'a str>,
        device_name: Option<&'a str>,
        pre_hash: &'a str,
    ) -> Self {
        Self {
            size,
            part_info_list,
            create_scene: create_scene.unwrap_or("file_upload"),
            device_name: device_name.unwrap_or_default(),
            pre_hash,
            _base: BaseCreationRequest {
                drive_id,
                parent_id,
                name,
                if_name_exists: if_name_exists.unwrap_or_default(),
                r#type: FileType::File,
            },
        }
    }
}

#[derive(Serialize, Debug)]
pub struct CreateFileRequest<'a> {
    size: u64,
    part_info_list: Vec<PartInfo>,
    create_scene: &'a str,
    device_name: &'a str,
    content_hash: &'a str,
    content_hash_name: &'a str,
    proof_code: &'a str,
    proof_version: &'a str,

    #[serde(flatten)]
    _base: BaseCreationRequest<'a>,
}

impl<'a> CreateFileRequest<'a> {
    pub fn new(
        drive_id: &'a str,
        parent_id: &'a str,
        name: &'a str,
        if_name_exists: Option<IfNameExists>,
        size: u64,
        part_info_list: Vec<PartInfo>,
        create_scene: Option<&'a str>,
        device_name: Option<&'a str>,
        content_hash: &'a str,
        content_hash_name: Option<&'a str>,
        proof_code: &'a str,
        proof_version: Option<&'a str>,
    ) -> Self {
        Self {
            size,
            part_info_list,
            create_scene: create_scene.unwrap_or("file_upload"),
            device_name: device_name.unwrap_or_default(),
            content_hash,
            content_hash_name: content_hash_name.unwrap_or("sha1"),
            proof_code,
            proof_version: proof_version.unwrap_or("v1"),
            _base: BaseCreationRequest {
                drive_id,
                parent_id,
                name,
                if_name_exists: if_name_exists.unwrap_or_default(),
                r#type: FileType::File,
            },
        }
    }
}

#[derive(Serialize, Debug)]
pub struct CompleteFileRequest<'a> {
    pub drive_id: &'a str,
    pub upload_id: &'a str,
    pub file_id: &'a str,
}

#[derive(Deserialize, Debug)]
pub struct BaseCreationResponse {
    pub drive_id: String,
    pub file_id: String,
    pub domain_id: String,
    pub file_name: String,
    pub parent_file_id: String,
    pub encrypt_mode: String,
    pub r#type: FileType,
}

#[derive(Deserialize, Debug)]
pub struct CreateDirResponse {
    #[serde(flatten)]
    pub _base: BaseCreationResponse,
}

#[derive(Deserialize, Debug)]
pub struct MatchPreHashResponse {
    pub parent_file_id: String,
    pub file_name: String,
    pub pre_hash: String,
    pub code: String,
    pub message: String,
}

impl MatchPreHashResponse {
    fn matched(&self) -> bool {
        self.code.as_str() == "PreHashMatched" && self.message.as_str() == "Pre hash matched."
    }
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum CreateFileResponse {
    File {
        part_info_list: Vec<PartInfo>,
        upload_id: String,
        rapid_upload: bool,
        revision_id: String,
        location: String,
        #[serde(flatten)]
        _base: BaseCreationResponse,
    },
    RapidFile {
        upload_id: String,
        rapid_upload: bool,
        revision_id: String,
        location: String,
        #[serde(flatten)]
        _base: BaseCreationResponse,
    },
}

#[derive(Deserialize, Debug)]
pub struct CompleteFileResponse {
    category: String,
    content_hash: String,
    content_hash_name: String,
    content_type: String,
    crc64_hash: String,
    created_at: String,
    creator_id: String,
    creator_type: String,
    domain_id: String,
    drive_id: String,
    encrypt_mode: String,
    file_extension: String,
    file_id: String,
    hidden: bool,
    last_modifier_id: String,
    last_modifier_type: String,
    location: String,
    name: String,
    parent_file_id: String,
    revision_id: String,
    revision_version: i64,
    size: i64,
    starred: bool,
    status: String,
    r#type: String,
    updated_at: String,
    upload_id: String,
    user_meta: String,
    user_tags: UserTags,
}

#[derive(Deserialize, Debug)]
pub struct UserTags {
    channel: String,
    client: String,
    device_id: String,
    device_name: String,
}
