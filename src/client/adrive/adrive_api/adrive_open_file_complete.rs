use crate::{response, AccessTokenLoader, AdriveClient, LoadAccessToken, OptionParam};
use serde_derive::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::Arc;

impl AdriveClient {
    pub async fn adrive_open_file_complete(&self) -> AdriveOpenFileCompleteRequest {
        AdriveOpenFileCompleteRequest {
            agent: self.clone_agent().await,
            api_host: self.clone_api_host().await,
            access_token: self.clone_access_token_loader().await,
            drive_id: None.into(),
            file_id: None.into(),
            upload_id: None.into(),
        }
    }
}

#[derive(Debug)]
pub struct AdriveOpenFileCompleteRequest {
    pub agent: Arc<reqwest::Client>,
    pub api_host: Arc<String>,
    pub access_token: Arc<Box<dyn AccessTokenLoader>>,
    pub drive_id: OptionParam<String>,
    pub file_id: OptionParam<String>,
    pub upload_id: OptionParam<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdriveOpenFileCompleteRequestPost {
    pub drive_id: String,
    pub file_id: String,
    pub upload_id: String,
}

impl AdriveOpenFileCompleteRequest {
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

    pub fn upload_id(mut self, upload_id: impl Into<OptionParam<String>>) -> Self {
        self.upload_id = upload_id.into();
        self
    }
}

impl AdriveOpenFileCompleteRequest {
    pub async fn request(&self) -> crate::Result<AdriveOpenFileComplete> {
        let resp = self
            .agent
            .post(format!("{}/adrive/v1.0/openFile/complete", self.api_host.as_str()).as_str())
            .load_access_token(self.access_token.clone())
            .await?
            .json(&AdriveOpenFileCompleteRequestPost {
                drive_id: if let Some(drive_id) = self.drive_id.deref() {
                    drive_id.clone().into()
                } else {
                    return Err(crate::Error::require_param_missing("drive_id"));
                },
                file_id: if let Some(file_id) = self.file_id.deref() {
                    file_id.clone().into()
                } else {
                    return Err(crate::Error::require_param_missing("file_id"));
                },
                upload_id: if let Some(upload_id) = self.upload_id.deref() {
                    upload_id.clone().into()
                } else {
                    return Err(crate::Error::require_param_missing("upload_id"));
                },
            })
            .send()
            .await?;
        response(resp).await
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdriveOpenFileComplete {
    pub drive_id: String,
    pub file_id: String,
    pub name: String,
    pub size: i64,
    pub file_extension: String,
    pub content_hash: String,
    pub category: String,
    pub r#type: String,
    pub thumbnail: Option<String>,
    pub url: Option<String>,
    pub download_url: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}
