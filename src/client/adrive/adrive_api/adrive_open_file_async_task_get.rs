use crate::{
    response, AccessTokenLoader, AdriveAsyncTaskState, AdriveClient, LoadAccessToken, OptionParam,
};
use serde_derive::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::Arc;

impl AdriveClient {
    pub async fn adrive_open_file_async_task_get(&self) -> AdriveOpenFileAsyncTaskGetRequest {
        AdriveOpenFileAsyncTaskGetRequest {
            agent: self.clone_agent().await,
            api_host: self.clone_api_host().await,
            access_token: self.clone_access_token_loader().await,
            async_task_id: None.into(),
        }
    }
}

#[derive(Debug)]
pub struct AdriveOpenFileAsyncTaskGetRequest {
    pub agent: Arc<reqwest::Client>,
    pub api_host: Arc<String>,
    pub access_token: Arc<Box<dyn AccessTokenLoader>>,
    pub async_task_id: OptionParam<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdriveOpenFileAsyncTaskGetPost {
    pub async_task_id: String,
}

impl AdriveOpenFileAsyncTaskGetRequest {
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

    pub fn async_task_id(mut self, async_task_id: impl Into<OptionParam<String>>) -> Self {
        self.async_task_id = async_task_id.into();
        self
    }
}

impl AdriveOpenFileAsyncTaskGetRequest {
    pub async fn request(&self) -> crate::Result<AdriveOpenFileAsyncTaskGet> {
        let url = format!(
            "{}/adrive/v1.0/openFile/async_task/get",
            self.api_host.deref().as_str()
        );
        let post = AdriveOpenFileAsyncTaskGetPost {
            async_task_id: if let Some(v) = self.async_task_id.as_ref() {
                v.clone()
            } else {
                return Err(crate::Error::require_param_missing("async_task_id"));
            },
        };
        let resp = self
            .agent
            .post(&url)
            .load_access_token(self.access_token.clone())
            .await?
            .json(&post)
            .send()
            .await?;
        response(resp).await
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdriveOpenFileAsyncTaskGet {
    pub state: AdriveAsyncTaskState,
    pub async_task_id: String,
}
