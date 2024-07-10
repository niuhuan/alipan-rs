use crate::{response, AccessTokenLoader, AdriveClient, LoadAccessToken, OptionParam};
use serde_derive::{Deserialize, Serialize};
use std::sync::Arc;

impl AdriveClient {
    pub async fn adrive_open_file_recyclebin_trash(&self) -> AdriveOpenFileRecyclebinTrashRequest {
        AdriveOpenFileRecyclebinTrashRequest {
            agent: self.clone_agent().await,
            api_host: self.clone_api_host().await,
            access_token: self.clone_access_token_loader().await,
            drive_id: None.into(),
            file_id: None.into(),
        }
    }
}

#[derive(Debug)]
pub struct AdriveOpenFileRecyclebinTrashRequest {
    pub agent: Arc<reqwest::Client>,
    pub api_host: Arc<String>,
    pub access_token: Arc<Box<dyn AccessTokenLoader>>,
    pub drive_id: OptionParam<String>,
    pub file_id: OptionParam<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdriveOpenFileRecyclebinTrashPost {
    pub drive_id: String,
    pub file_id: String,
}

impl AdriveOpenFileRecyclebinTrashRequest {
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
}

impl AdriveOpenFileRecyclebinTrashRequest {
    pub async fn request(&self) -> crate::Result<AdriveOpenFileRecyclebinTrash> {
        let url = format!("{}/adrive/v1.0/openFile/recyclebin/trash", self.api_host);
        let resp = self
            .agent
            .post(&url)
            .load_access_token(self.access_token.clone())
            .await?
            .json(&AdriveOpenFileRecyclebinTrashPost {
                drive_id: if let Some(drive_id) = self.drive_id.as_ref() {
                    drive_id.to_string()
                } else {
                    return Err(crate::Error::require_param_missing("drive_id"));
                },
                file_id: if let Some(file_id) = self.file_id.as_ref() {
                    file_id.to_string()
                } else {
                    return Err(crate::Error::require_param_missing("file_id"));
                },
            })
            .send()
            .await?;
        response(resp).await
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdriveOpenFileRecyclebinTrash {
    pub drive_id: String,
    pub file_id: String,
    pub async_task_id: Option<String>,
}
