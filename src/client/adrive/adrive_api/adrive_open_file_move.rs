use crate::{
    response, AccessTokenLoader, AdriveClient, CheckNameMode, LoadAccessToken, OptionParam,
};
use serde_derive::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::Arc;

impl AdriveClient {
    pub async fn adrive_open_file_move(&self) -> AdriveOpenFileMoveRequest {
        AdriveOpenFileMoveRequest {
            agent: self.clone_agent().await,
            api_host: self.clone_api_host().await,
            access_token: self.clone_access_token_loader().await,
            drive_id: None.into(),
            file_id: None.into(),
            to_parent_file_id: None.into(),
            check_name_mode: None.into(),
            new_name: None.into(),
        }
    }
}

#[derive(Debug)]
pub struct AdriveOpenFileMoveRequest {
    pub agent: Arc<reqwest::Client>,
    pub api_host: Arc<String>,
    pub access_token: Arc<Box<dyn AccessTokenLoader>>,
    pub drive_id: OptionParam<String>,
    pub file_id: OptionParam<String>,
    pub to_parent_file_id: OptionParam<String>,
    pub check_name_mode: OptionParam<CheckNameMode>,
    pub new_name: OptionParam<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Default)]
pub struct AdriveOpenFileMoveRequestPost {
    pub drive_id: String,
    pub file_id: String,
    pub to_parent_file_id: String,
    pub check_name_mode: Option<CheckNameMode>,
    pub new_name: Option<String>,
}

impl AdriveOpenFileMoveRequest {
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

    pub fn to_parent_file_id(mut self, to_parent_file_id: impl Into<OptionParam<String>>) -> Self {
        self.to_parent_file_id = to_parent_file_id.into();
        self
    }

    pub fn check_name_mode(
        mut self,
        check_name_mode: impl Into<OptionParam<CheckNameMode>>,
    ) -> Self {
        self.check_name_mode = check_name_mode.into();
        self
    }

    pub fn new_name(mut self, new_name: impl Into<OptionParam<String>>) -> Self {
        self.new_name = new_name.into();
        self
    }
}

impl AdriveOpenFileMoveRequest {
    pub async fn request(&self) -> crate::Result<AdriveOpenFileMove> {
        let resp = self
            .agent
            .post(format!("{}/adrive/v1.0/openFile/move", self.api_host.as_str()).as_str())
            .load_access_token(self.access_token.clone())
            .await?
            .json(&AdriveOpenFileMoveRequestPost {
                drive_id: if let Some(drive_id) = self.drive_id.deref() {
                    drive_id.deref().to_string()
                } else {
                    return Err(crate::Error::require_param_missing("drive_id"));
                },
                file_id: if let Some(file_id) = self.file_id.deref() {
                    file_id.deref().to_string()
                } else {
                    return Err(crate::Error::require_param_missing("file_id"));
                },
                to_parent_file_id: if let Some(to_parent_file_id) = self.to_parent_file_id.deref() {
                    to_parent_file_id.deref().to_string()
                } else {
                    return Err(crate::Error::require_param_missing("to_parent_file_id"));
                },
                check_name_mode: self.check_name_mode.clone(),
                new_name: self.new_name.clone(),
            })
            .send()
            .await?;
        response(resp).await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Default)]
pub struct AdriveOpenFileMove {
    pub drive_id: String,
    pub file_id: String,
    pub async_task_id: Option<String>,
    pub exist: bool,
}
