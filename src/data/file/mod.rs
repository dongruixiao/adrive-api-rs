use serde::{Deserialize, Serialize};

pub mod create;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum FileType {
    Folder,
    File,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "snake_case")]
pub enum IfNameExists {
    #[default]
    AutoRename,
    Refuse,
    Ignore,
}

#[derive(Serialize, Default, Debug)]
#[serde(rename_all = "snake_case")]
pub enum OrderBy {
    CreatedAt,
    #[default]
    UpdatedAt,
    Size,
    Name,
}

#[derive(Serialize, Default, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum SortBy {
    #[default]
    Desc,
    Asc,
}
