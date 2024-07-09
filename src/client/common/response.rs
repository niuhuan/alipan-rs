use crate::{AdriveOpenFilePartInfo, AdriveOpenFileType};
use serde::Deserialize;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OauthUsersInfo {
    pub id: String,
    pub name: String,
    pub avatar: String,
    pub phone: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OauthUsersScopes {
    pub id: String,
    pub scopes: Vec<OauthUsersScope>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OauthUsersScope {
    pub scope: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdriveUserGetDriveInfo {
    pub user_id: String,
    pub name: String,
    pub avatar: String,
    pub default_drive_id: String,
    pub resource_drive_id: Option<String>,
    pub backup_drive_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdriveUserGetSpaceInfo {
    pub personal_space_info: AdriveUserSpaceInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdriveUserSpaceInfo {
    pub used_size: i64,
    pub total_size: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdriveOpenFileList {
    pub items: Vec<AdriveOpenFile>,
    #[serde(deserialize_with = "blank_to_null")]
    pub next_marker: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdriveOpenFile {
    pub drive_id: String,
    pub file_id: String,
    pub parent_file_id: String,
    pub name: String,
    #[serde(deserialize_with = "null_to_default")]
    pub size: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub file_extension: String,
    #[serde(deserialize_with = "null_to_default")]
    pub content_hash: String,
    #[serde(deserialize_with = "null_to_default")]
    pub category: String,
    pub r#type: AdriveOpenFileType,
    pub thumbnail: Option<String>,
    pub url: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub play_cursor: Option<String>,
    pub video_media_metadata: Option<Value>,
    pub video_preview_metadata: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdriveOpenFileCreate {
    pub drive_id: String,
    pub file_id: String,
    pub status: String,
    pub parent_file_id: String,
    pub upload_id: Option<String>,
    pub file_name: String,
    pub available: bool,
    pub exist: bool,
    pub rapid_upload: bool,
    pub part_info_list: Option<Vec<AdriveOpenFilePartInfo>>,
}

fn null_to_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: serde::Deserializer<'de>,
    T: Default + serde::Deserialize<'de>,
{
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
}

fn blank_to_null<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let opt = Option::deserialize(deserializer)?;
    Ok(opt
        .map(|s: String| if s.is_empty() { None } else { Some(s) })
        .flatten())
}
