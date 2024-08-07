use crate::{response, AccessTokenLoader, AdriveClient, LoadAccessToken, OptionParam};
use serde_derive::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::Arc;

impl AdriveClient {
    pub async fn adrive_open_file_get_download_url(&self) -> AdriveOpenFileGetDownloadUrlRequest {
        AdriveOpenFileGetDownloadUrlRequest {
            agent: self.clone_agent().await,
            api_host: self.clone_api_host().await,
            access_token: self.clone_access_token_loader().await,
            drive_id: None.into(),
            file_id: None.into(),
            expire_sec: None.into(),
        }
    }
}

#[derive(Debug)]
pub struct AdriveOpenFileGetDownloadUrlRequest {
    pub agent: Arc<reqwest::Client>,
    pub api_host: Arc<String>,
    pub access_token: Arc<Box<dyn AccessTokenLoader>>,
    pub drive_id: OptionParam<String>,
    pub file_id: OptionParam<String>,
    pub expire_sec: OptionParam<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Eq, PartialEq)]
pub struct AdriveOpenFileGetDownloadUrlRequestPost {
    pub drive_id: String,
    pub file_id: String,
    pub expire_sec: Option<i64>,
}

impl AdriveOpenFileGetDownloadUrlRequest {
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

    pub fn expire_sec(mut self, expire_sec: impl Into<OptionParam<i64>>) -> Self {
        self.expire_sec = expire_sec.into();
        self
    }
}

impl AdriveOpenFileGetDownloadUrlRequest {
    pub async fn request(&self) -> crate::Result<AdriveOpenFileGetDownloadUrl> {
        let resp = self
            .agent
            .post(&format!(
                "{}/adrive/v1.0/openFile/getDownloadUrl",
                self.api_host
            ))
            .load_access_token(self.access_token.clone())
            .await?
            .json(&AdriveOpenFileGetDownloadUrlRequestPost {
                drive_id: if let Some(drive_id) = self.drive_id.deref() {
                    drive_id.clone()
                } else {
                    return Err(crate::Error::require_param_missing("drive_id"));
                },
                file_id: if let Some(file_id) = self.file_id.deref() {
                    file_id.clone()
                } else {
                    return Err(crate::Error::require_param_missing("file_id"));
                },
                expire_sec: self.expire_sec.to_owned(),
            })
            .send()
            .await?;
        response(resp).await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Eq, PartialEq)]
pub struct AdriveOpenFileGetDownloadUrl {
    pub url: String,
    pub expiration: String,
    pub method: String,
    pub description: Option<String>,
}
