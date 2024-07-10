use crate::{response, AccessTokenLoader, OptionParam};
use serde_derive::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::Arc;

#[derive(Debug)]
pub struct AdriveOpenFileCopyRequest {
    pub agent: Arc<reqwest::Client>,
    pub api_host: Arc<String>,
    pub access_token: Arc<Box<dyn AccessTokenLoader>>,
    pub drive_id: OptionParam<String>,
    pub file_id: OptionParam<String>,
    pub to_drive_id: OptionParam<String>,
    pub to_parent_file_id: OptionParam<String>,
    pub auto_rename: OptionParam<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdriveOpenFileCopyRequestPost {
    pub drive_id: String,
    pub file_id: String,
    pub to_drive_id: Option<String>,
    pub to_parent_file_id: String,
    pub auto_rename: Option<bool>,
}

impl AdriveOpenFileCopyRequest {
    pub fn agent(mut self, agent: Arc<reqwest::Client>) -> Self {
        self.agent = agent;
        self
    }

    pub fn api_host(mut self, api_host: impl Into<String>) -> Self {
        self.api_host = Arc::new(api_host.into());
        self
    }

    pub fn access_token(mut self, access_token: Arc<Box<dyn AccessTokenLoader>>) -> Self {
        self.access_token = access_token;
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

    pub fn to_drive_id(mut self, to_drive_id: impl Into<OptionParam<String>>) -> Self {
        self.to_drive_id = to_drive_id.into();
        self
    }

    pub fn to_parent_file_id(mut self, to_parent_file_id: impl Into<OptionParam<String>>) -> Self {
        self.to_parent_file_id = to_parent_file_id.into();
        self
    }

    pub fn auto_rename(mut self, auto_rename: impl Into<OptionParam<bool>>) -> Self {
        self.auto_rename = auto_rename.into();
        self
    }
}

impl AdriveOpenFileCopyRequest {
    pub async fn request(&self) -> crate::Result<AdriveOpenFileCopy> {
        let resp = self
            .agent
            .post(format!("{}/adrive/v1.0/openFile/copy", self.api_host.as_str()).as_str())
            .header(
                "Authorization",
                format!(
                    "Bearer {}",
                    self.access_token.get_access_token().await?.access_token
                )
                .as_str(),
            )
            .json(&AdriveOpenFileCopyRequestPost {
                drive_id: if let Some(drive_id) = self.drive_id.deref() {
                    drive_id.to_string()
                } else {
                    return Err(crate::Error::msg("drive_id"));
                },
                file_id: if let Some(file_id) = self.file_id.deref() {
                    file_id.to_string()
                } else {
                    return Err(crate::Error::msg("file_id"));
                },
                to_drive_id: self.to_drive_id.clone(),
                to_parent_file_id: if let Some(to_parent_file_id) = self.to_parent_file_id.deref() {
                    to_parent_file_id.to_string()
                } else {
                    return Err(crate::Error::msg("to_parent_file_id"));
                },
                auto_rename: self.auto_rename.clone(),
            })
            .send()
            .await?;
        response(resp).await
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdriveOpenFileCopy {
    pub drive_id: String,
    pub file_id: String,
    pub async_task_id: Option<String>,
}
