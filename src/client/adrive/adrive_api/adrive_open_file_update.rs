use crate::{
    AccessTokenLoader, AdriveClient, AdriveOpenFileType, CheckNameMode, LoadAccessToken,
    OptionParam,
};
use serde_derive::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::Arc;

impl AdriveClient {
    pub async fn adrive_open_file_update(&self) -> AdriveOpenFileUpdateRequest {
        AdriveOpenFileUpdateRequest {
            agent: self.clone_agent().await,
            api_host: self.clone_api_host().await,
            access_token: self.clone_access_token_loader().await,
            drive_id: None.into(),
            file_id: None.into(),
            name: None.into(),
            check_name_mode: None.into(),
            starred: None.into(),
        }
    }
}

#[derive(Debug)]
pub struct AdriveOpenFileUpdateRequest {
    pub agent: Arc<reqwest::Client>,
    pub api_host: Arc<String>,
    pub access_token: Arc<Box<dyn AccessTokenLoader>>,
    pub drive_id: OptionParam<String>,
    pub file_id: OptionParam<String>,
    pub name: OptionParam<String>,
    pub check_name_mode: OptionParam<CheckNameMode>,
    pub starred: OptionParam<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Default)]
pub struct AdriveOpenFileUpdateRequestPost {
    pub drive_id: String,
    pub file_id: String,
    pub name: Option<String>,
    pub check_name_mode: Option<CheckNameMode>,
    pub starred: Option<bool>,
}

impl AdriveOpenFileUpdateRequest {
    pub fn agent(mut self, agent: impl Into<Arc<reqwest::Client>>) -> Self {
        self.agent = agent.into();
        self
    }

    pub fn api_host(mut self, api_host: impl Into<Arc<String>>) -> Self {
        self.api_host = api_host.into();
        self
    }

    pub fn access_token(
        mut self,
        access_token: impl Into<Arc<Box<dyn AccessTokenLoader>>>,
    ) -> Self {
        self.access_token = access_token.into();
        self
    }

    pub fn drive_id(mut self, drive_id: impl Into<OptionParam<String>>) -> Self {
        self.drive_id = drive_id.into();
        self
    }

    pub fn file_id(mut self, file_id: impl Into<OptionParam<String>>) -> Self {
        self.file_id = file_id.into();
        self
    }

    pub fn name(mut self, name: impl Into<OptionParam<String>>) -> Self {
        self.name = name.into();
        self
    }

    pub fn check_name_mode(
        mut self,
        check_name_mode: impl Into<OptionParam<CheckNameMode>>,
    ) -> Self {
        self.check_name_mode = check_name_mode.into();
        self
    }

    pub fn starred(mut self, starred: impl Into<OptionParam<bool>>) -> Self {
        self.starred = starred.into();
        self
    }
}

impl AdriveOpenFileUpdateRequest {
    pub async fn request(&self) -> crate::Result<AdriveOpenFileUpdate> {
        let resp = self
            .agent
            .post(format!("{}/adrive/v1.0/openFile/update", self.api_host.as_str()).as_str())
            .load_access_token(self.access_token.clone())
            .await?
            .json(&AdriveOpenFileUpdateRequestPost {
                drive_id: if let Some(ref v) = self.drive_id.deref() {
                    v.clone()
                } else {
                    return Err(crate::Error::require_param_missing("drive_id"));
                },
                file_id: if let Some(ref v) = self.file_id.deref() {
                    v.clone()
                } else {
                    return Err(crate::Error::require_param_missing("file_id"));
                },
                name: self.name.clone(),
                check_name_mode: self.check_name_mode.clone(),
                starred: self.starred.clone(),
            })
            .send()
            .await?;
        let resp = resp.json::<AdriveOpenFileUpdate>().await?;
        Ok(resp)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct AdriveOpenFileUpdate {
    pub drive_id: String,
    pub file_id: String,
    pub name: String,
    pub size: i64,
    pub file_extension: String,
    pub content_hash: String,
    pub category: String,
    pub r#type: AdriveOpenFileType,
    pub created_at: String,
    pub updated_at: String,
}
