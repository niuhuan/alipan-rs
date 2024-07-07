use crate::{AccessTokenLoader, AdriveOpenFileType, CheckNameMode};
use chrono::Local;
use std::sync::Arc;

pub struct AdriveOpenFileCreateRequest {
    pub agent: Arc<reqwest::Client>,
    pub api_host: Arc<String>,
    pub access_token: AccessTokenLoader,
    pub drive_id: String,
    pub parent_file_id: String,
    pub name: String,
    pub r#type: AdriveOpenFileType,
    pub check_name_mode: CheckNameMode,
    pub part_info_list: Option<Vec<AdriveOpenFilePartInfo>>,
    pub streams_info: Option<Vec<AdriveOpenFileStreamInfo>>,
    pub pre_hash: Option<String>,
    pub size: i64,
    pub content_hash: Option<String>,
    pub content_hash_name: Option<String>,
    pub proof_code: Option<String>,
    pub proof_version: Option<String>,
    pub local_created_at: Option<chrono::DateTime<Local>>,
    pub local_modified_at: Option<chrono::DateTime<Local>>,
}

pub struct AdriveOpenFilePartInfo {
    pub part_number: i64,
}

pub struct AdriveOpenFileStreamInfo {
    pub content_hash: String,
    pub content_hash_name: String,
    pub proof_version: String,
    pub proof_code: String,
    pub content_md5: String,
    pub pre_hash: String,
    pub size: i64,
    pub part_info_list: Vec<AdriveOpenFilePartInfo>,
}
