use crate::client::common::access_token_loader::AccessTokenLoader;
use crate::{
    response, AdriveOpenFileCreate, AdriveOpenFileCreatePost, AdriveOpenFilePartInfo,
    AdriveOpenFileStreamInfo, AdriveOpenFileType, CheckNameMode,
};
use chrono::Local;
use std::sync::Arc;

pub struct AdriveOpenFileCreateRequest {
    pub agent: Arc<reqwest::Client>,
    pub api_host: Arc<String>,
    pub access_token: Arc<Box<dyn AccessTokenLoader>>,
    pub drive_id: String,
    pub parent_file_id: String,
    pub name: String,
    pub r#type: AdriveOpenFileType,
    pub check_name_mode: CheckNameMode,
    pub part_info_list: Option<Vec<AdriveOpenFilePartInfo>>,
    pub streams_info: Option<Vec<AdriveOpenFileStreamInfo>>,
    pub pre_hash: Option<String>,
    pub size: Option<i64>,
    pub content_hash: Option<String>,
    pub content_hash_name: Option<String>,
    pub proof_code: Option<String>,
    pub proof_version: Option<String>,
    pub local_created_at: Option<chrono::DateTime<Local>>,
    pub local_modified_at: Option<chrono::DateTime<Local>>,
}

impl AdriveOpenFileCreateRequest {
    pub async fn request(&self) -> crate::Result<AdriveOpenFileCreate> {
        let params = AdriveOpenFileCreatePost {
            drive_id: self.drive_id.clone(),
            parent_file_id: self.parent_file_id.clone(),
            name: self.name.clone(),
            r#type: self.r#type.clone(),
            check_name_mode: self.check_name_mode.clone(),
            part_info_list: self.part_info_list.clone(),
            streams_info: self.streams_info.clone(),
            pre_hash: self.pre_hash.clone(),
            size: self.size.clone(),
            content_hash: self.content_hash.clone(),
            content_hash_name: self.content_hash_name.clone(),
            proof_code: self.proof_code.clone(),
            proof_version: self.proof_version.clone(),
            local_created_at: self.local_created_at.clone(),
            local_modified_at: self.local_modified_at.clone(),
        };
        let token = self.access_token.get_access_token().await?;
        let resp = self
            .agent
            .post(format!("{}/adrive/v1.0/openFile/create", self.api_host.as_str()).as_str())
            .header("Authorization", format!("Bearer {}", token.access_token))
            .json(&params)
            .send()
            .await?;
        response(resp).await
    }
}

impl AdriveOpenFileCreateRequest {
    pub fn agent(mut self, agent: Arc<reqwest::Client>) -> Self {
        self.agent = agent;
        self
    }

    pub fn api_host(mut self, api_host: impl Into<String>) -> Self {
        self.api_host = Arc::new(api_host.into());
        self
    }

    pub fn drive_id(mut self, drive_id: impl Into<String>) -> Self {
        self.drive_id = drive_id.into();
        self
    }

    pub fn parent_file_id(mut self, parent_file_id: impl Into<String>) -> Self {
        self.parent_file_id = parent_file_id.into();
        self
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    pub fn r#type(mut self, r#type: AdriveOpenFileType) -> Self {
        self.r#type = r#type;
        self
    }

    pub fn check_name_mode(mut self, check_name_mode: CheckNameMode) -> Self {
        self.check_name_mode = check_name_mode;
        self
    }

    pub fn part_info_list(mut self, part_info_list: Vec<AdriveOpenFilePartInfo>) -> Self {
        self.part_info_list = Some(part_info_list);
        self
    }

    pub fn streams_info(mut self, streams_info: Vec<AdriveOpenFileStreamInfo>) -> Self {
        self.streams_info = Some(streams_info);
        self
    }

    pub fn pre_hash(mut self, pre_hash: impl Into<String>) -> Self {
        self.pre_hash = Some(pre_hash.into());
        self
    }

    pub fn size(mut self, size: i64) -> Self {
        self.size = Some(size);
        self
    }

    pub fn content_hash(mut self, content_hash: impl Into<String>) -> Self {
        self.content_hash = Some(content_hash.into());
        self
    }

    pub fn content_hash_name(mut self, content_hash_name: impl Into<String>) -> Self {
        self.content_hash_name = Some(content_hash_name.into());
        self
    }

    pub fn proof_code(mut self, proof_code: impl Into<String>) -> Self {
        self.proof_code = Some(proof_code.into());
        self
    }

    pub fn proof_version(mut self, proof_version: impl Into<String>) -> Self {
        self.proof_version = Some(proof_version.into());
        self
    }

    pub fn local_created_at(mut self, local_created_at: chrono::DateTime<Local>) -> Self {
        self.local_created_at = Some(local_created_at);
        self
    }

    pub fn local_modified_at(mut self, local_modified_at: chrono::DateTime<Local>) -> Self {
        self.local_modified_at = Some(local_modified_at);
        self
    }
}
