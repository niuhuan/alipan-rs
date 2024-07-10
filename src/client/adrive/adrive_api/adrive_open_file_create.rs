use crate::client::common::access_token_loader::AccessTokenLoader;
use crate::{
    null_to_default, response, AdriveClient, AdriveOpenFileCreatePost, AdriveOpenFilePartInfo,
    AdriveOpenFileStreamInfo, AdriveOpenFileType, CheckNameMode, OptionParam,
};
use chrono::Local;
use serde_derive::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::Arc;

impl AdriveClient {
    pub async fn adrive_open_file_create(&self) -> AdriveOpenFileCreateRequest {
        AdriveOpenFileCreateRequest {
            agent: self.clone_agent().await,
            api_host: self.clone_api_host().await,
            access_token: self.clone_access_token_loader().await,
            drive_id: None.into(),
            parent_file_id: None.into(),
            name: None.into(),
            r#type: None.into(),
            check_name_mode: None.into(),
            part_info_list: None.into(),
            streams_info: None.into(),
            pre_hash: None.into(),
            size: None.into(),
            content_hash: None.into(),
            content_hash_name: None.into(),
            proof_code: None.into(),
            proof_version: None.into(),
            local_created_at: None.into(),
            local_modified_at: None.into(),
        }
    }
}

pub struct AdriveOpenFileCreateRequest {
    pub agent: Arc<reqwest::Client>,
    pub api_host: Arc<String>,
    pub access_token: Arc<Box<dyn AccessTokenLoader>>,
    pub drive_id: OptionParam<String>,
    pub parent_file_id: OptionParam<String>,
    pub name: OptionParam<String>,
    pub r#type: OptionParam<AdriveOpenFileType>,
    pub check_name_mode: OptionParam<CheckNameMode>,
    pub part_info_list: OptionParam<Vec<AdriveOpenFilePartInfo>>,
    pub streams_info: OptionParam<Vec<AdriveOpenFileStreamInfo>>,
    pub pre_hash: OptionParam<String>,
    pub size: OptionParam<i64>,
    pub content_hash: OptionParam<String>,
    pub content_hash_name: OptionParam<String>,
    pub proof_code: OptionParam<String>,
    pub proof_version: OptionParam<String>,
    pub local_created_at: OptionParam<chrono::DateTime<Local>>,
    pub local_modified_at: OptionParam<chrono::DateTime<Local>>,
}

impl AdriveOpenFileCreateRequest {
    pub async fn request(&self) -> crate::Result<AdriveOpenFileCreate> {
        let params = AdriveOpenFileCreatePost {
            drive_id: if let Some(drive_id) = self.drive_id.deref() {
                drive_id.clone()
            } else {
                return Err(crate::Error::msg("drive_id is required"));
            },
            parent_file_id: if let Some(parent_file_id) = self.parent_file_id.deref() {
                parent_file_id.clone()
            } else {
                return Err(crate::Error::msg("parent_file_id is required"));
            },
            name: if let Some(name) = self.name.deref() {
                name.clone()
            } else {
                return Err(crate::Error::msg("name is required"));
            },
            r#type: if let Some(r#type) = self.r#type.deref() {
                r#type.clone()
            } else {
                return Err(crate::Error::msg("r#type is required"));
            },
            check_name_mode: if let Some(check_name_mode) = self.check_name_mode.deref() {
                check_name_mode.clone()
            } else {
                return Err(crate::Error::msg("check_name_mode is required"));
            },
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

    pub fn drive_id(mut self, drive_id: impl Into<OptionParam<String>>) -> Self {
        self.drive_id = drive_id.into();
        self
    }

    pub fn parent_file_id(mut self, parent_file_id: impl Into<OptionParam<String>>) -> Self {
        self.parent_file_id = parent_file_id.into();
        self
    }

    pub fn name(mut self, name: impl Into<OptionParam<String>>) -> Self {
        self.name = name.into();
        self
    }

    pub fn r#type(mut self, r#type: impl Into<OptionParam<AdriveOpenFileType>>) -> Self {
        self.r#type = r#type.into();
        self
    }

    pub fn check_name_mode(
        mut self,
        check_name_mode: impl Into<OptionParam<CheckNameMode>>,
    ) -> Self {
        self.check_name_mode = check_name_mode.into();
        self
    }

    pub fn part_info_list(
        mut self,
        part_info_list: impl Into<OptionParam<Vec<AdriveOpenFilePartInfo>>>,
    ) -> Self {
        self.part_info_list = part_info_list.into();
        self
    }

    pub fn streams_info(
        mut self,
        streams_info: impl Into<OptionParam<Vec<AdriveOpenFileStreamInfo>>>,
    ) -> Self {
        self.streams_info = streams_info.into();
        self
    }

    pub fn pre_hash(mut self, pre_hash: impl Into<OptionParam<String>>) -> Self {
        self.pre_hash = pre_hash.into();
        self
    }

    pub fn size(mut self, size: impl Into<OptionParam<i64>>) -> Self {
        self.size = size.into();
        self
    }

    pub fn content_hash(mut self, content_hash: impl Into<OptionParam<String>>) -> Self {
        self.content_hash = content_hash.into();
        self
    }

    pub fn content_hash_name(mut self, content_hash_name: impl Into<OptionParam<String>>) -> Self {
        self.content_hash_name = content_hash_name.into();
        self
    }

    pub fn proof_code(mut self, proof_code: impl Into<OptionParam<String>>) -> Self {
        self.proof_code = proof_code.into();
        self
    }

    pub fn proof_version(mut self, proof_version: impl Into<OptionParam<String>>) -> Self {
        self.proof_version = proof_version.into();
        self
    }

    pub fn local_created_at(
        mut self,
        local_created_at: impl Into<OptionParam<chrono::DateTime<Local>>>,
    ) -> Self {
        self.local_created_at = local_created_at.into();
        self
    }

    pub fn local_modified_at(
        mut self,
        local_modified_at: impl Into<OptionParam<chrono::DateTime<Local>>>,
    ) -> Self {
        self.local_modified_at = local_modified_at.into();
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdriveOpenFileCreate {
    pub drive_id: String,
    pub file_id: String,
    pub status: Option<String>,
    pub parent_file_id: String,
    pub upload_id: Option<String>,
    pub file_name: String,
    #[serde(deserialize_with = "null_to_default")]
    pub available: bool,
    #[serde(deserialize_with = "null_to_default")]
    pub exist: bool,
    #[serde(deserialize_with = "null_to_default")]
    pub rapid_upload: bool,
    #[serde(deserialize_with = "null_to_default")]
    pub part_info_list: Vec<AdriveOpenFilePartInfo>,
}
