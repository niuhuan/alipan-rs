use crate::response::null_to_default;
use crate::{response, AccessTokenLoader, AdriveClient, LoadAccessToken, OptionParam};
use serde_derive::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::Arc;

impl AdriveClient {
    pub async fn adrive_open_file_batch_get(&self) -> AdriveOpenFileBatchGetRequest {
        AdriveOpenFileBatchGetRequest {
            agent: self.clone_agent().await,
            api_host: self.clone_api_host().await,
            access_token: self.clone_access_token_loader().await,
            file_list: vec![],
            video_thumbnail_time: None.into(),
            video_thumbnail_width: None.into(),
            image_thumbnail_width: None.into(),
        }
    }
}

#[derive(Debug)]
pub struct AdriveOpenFileBatchGetRequest {
    pub agent: Arc<reqwest::Client>,
    pub api_host: Arc<String>,
    pub access_token: Arc<Box<dyn AccessTokenLoader>>,
    pub file_list: Vec<AdriveOpenFileBatchGetRequestFileList>,
    pub video_thumbnail_time: OptionParam<i64>,
    pub video_thumbnail_width: OptionParam<i64>,
    pub image_thumbnail_width: OptionParam<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdriveOpenFileBatchGetRequestPost {
    pub file_list: Vec<AdriveOpenFileBatchGetRequestFileList>,
    pub video_thumbnail_time: Option<i64>,
    pub video_thumbnail_width: Option<i64>,
    pub image_thumbnail_width: Option<i64>,
}

impl AdriveOpenFileBatchGetRequest {
    pub fn agent(mut self, agent: Arc<reqwest::Client>) -> Self {
        self.agent = agent;
        self
    }

    pub fn api_host(mut self, api_host: impl Into<String>) -> Self {
        self.api_host = Arc::new(api_host.into());
        self
    }

    pub fn file_list(mut self, file_list: Vec<AdriveOpenFileBatchGetRequestFileList>) -> Self {
        self.file_list = file_list;
        self
    }

    pub fn video_thumbnail_time(
        mut self,
        video_thumbnail_time: impl Into<OptionParam<i64>>,
    ) -> Self {
        self.video_thumbnail_time = video_thumbnail_time.into();
        self
    }

    pub fn video_thumbnail_width(
        mut self,
        video_thumbnail_width: impl Into<OptionParam<i64>>,
    ) -> Self {
        self.video_thumbnail_width = video_thumbnail_width.into();
        self
    }

    pub fn image_thumbnail_width(
        mut self,
        image_thumbnail_width: impl Into<OptionParam<i64>>,
    ) -> Self {
        self.image_thumbnail_width = image_thumbnail_width.into();
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdriveOpenFileBatchGetRequestFileList {
    pub drive_id: String,
    pub file_id: String,
}

impl AdriveOpenFileBatchGetRequestFileList {
    pub fn drive_id(mut self, drive_id: impl Into<String>) -> Self {
        self.drive_id = drive_id.into();
        self
    }

    pub fn file_id(mut self, file_id: impl Into<String>) -> Self {
        self.file_id = file_id.into();
        self
    }
}

impl AdriveOpenFileBatchGetRequest {
    pub async fn request(&self) -> crate::Result<AdriveOpenFileBatchGetResponse> {
        let url = format!("{}/adrive/v1.0/openFile/batch/get", self.api_host.deref());
        let body = AdriveOpenFileBatchGetRequestPost {
            file_list: self.file_list.clone(),
            video_thumbnail_time: self.video_thumbnail_time.deref().clone(),
            video_thumbnail_width: self.video_thumbnail_width.deref().clone(),
            image_thumbnail_width: self.image_thumbnail_width.deref().clone(),
        };
        let resp = self
            .agent
            .post(&url)
            .load_access_token(self.access_token.clone())
            .await?
            .json(&body)
            .send()
            .await?;
        response(resp).await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdriveOpenFileBatchGetResponse {
    pub items: Vec<AdriveOpenFileBatchGetResponseItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdriveOpenFileBatchGetResponseItem {
    pub drive_id: String,
    pub file_id: String,
    pub parent_file_id: String,
    pub name: String,
    #[serde(deserialize_with = "null_to_default")]
    pub size: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub file_extension: String,
    #[serde(deserialize_with = "null_to_default")]
    pub content_hash: String,
    #[serde(deserialize_with = "null_to_default")]
    pub category: String,
    pub r#type: String,
    pub thumbnail: Option<String>,
    pub url: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}
