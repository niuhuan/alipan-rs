// /adrive/v1.0/openFile/get_by_path

use crate::adrive_api::AdriveOpenFileGet;
use crate::{response, AccessTokenLoader, AdriveClient, Error, OptionParam};
use serde_derive::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::Arc;

impl AdriveClient {
    pub async fn adrive_open_file_get_by_path(&self) -> AdriveOpenFileGetByPathRequest {
        AdriveOpenFileGetByPathRequest {
            agent: self.clone_agent().await,
            api_host: self.clone_api_host().await,
            access_token: self.clone_access_token_loader().await,
            drive_id: None.into(),
            file_path: None.into(),
        }
    }
}

#[derive(Debug)]
pub struct AdriveOpenFileGetByPathRequest {
    pub agent: Arc<reqwest::Client>,
    pub api_host: Arc<String>,
    pub access_token: Arc<Box<dyn AccessTokenLoader>>,
    pub drive_id: OptionParam<String>,
    pub file_path: OptionParam<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdriveOpenFileGetByPathRequestPost {
    pub drive_id: String,
    pub file_path: String,
}

impl AdriveOpenFileGetByPathRequest {
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

    pub fn file_path(mut self, file_path: impl Into<OptionParam<String>>) -> Self {
        self.file_path = file_path.into();
        self
    }
}

impl AdriveOpenFileGetByPathRequest {
    pub async fn request(&self) -> crate::Result<AdriveOpenFileGet> {
        let token = self.access_token.get_access_token().await?;
        let resp = self
            .agent
            .post(
                format!(
                    "{}/adrive/v1.0/openFile/get_by_path",
                    self.api_host.as_str()
                )
                .as_str(),
            )
            .header("Authorization", format!("Bearer {}", token.access_token))
            .json(&AdriveOpenFileGetByPathRequestPost {
                drive_id: if let Some(drive_id) = &self.drive_id.deref() {
                    drive_id.to_owned()
                } else {
                    return Err(Error::msg("drive_id is required"));
                },
                file_path: if let Some(file_path) = &self.file_path.deref() {
                    file_path.to_owned()
                } else {
                    return Err(Error::msg("file_path is required"));
                },
            })
            .send()
            .await?;
        response(resp).await
    }
}