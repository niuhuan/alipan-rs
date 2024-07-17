use crate::response::null_to_default;
use crate::{
    response, AccessTokenLoader, AdriveClient, AdriveOpenFileType, Error, LoadAccessToken,
    OptionParam,
};
use chrono::Utc;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use std::ops::Deref;
use std::sync::Arc;

impl AdriveClient {
    pub async fn adrive_open_file_get(&self) -> AdriveOpenFileGetRequest {
        AdriveOpenFileGetRequest {
            agent: self.clone_agent().await,
            api_host: self.clone_api_host().await,
            access_token: self.clone_access_token_loader().await,
            drive_id: None.into(),
            file_id: None.into(),
            video_thumbnail_time: None.into(),
            video_thumbnail_width: None.into(),
            image_thumbnail_width: None.into(),
            fields: None.into(),
        }
    }
}

#[derive(Debug)]
pub struct AdriveOpenFileGetRequest {
    pub agent: Arc<reqwest::Client>,
    pub api_host: Arc<String>,
    pub access_token: Arc<Box<dyn AccessTokenLoader>>,
    pub drive_id: OptionParam<String>,
    pub file_id: OptionParam<String>,
    pub video_thumbnail_time: OptionParam<i64>,
    pub video_thumbnail_width: OptionParam<i64>,
    pub image_thumbnail_width: OptionParam<i64>,
    pub fields: OptionParam<String>,
}

impl AdriveOpenFileGetRequest {
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

    pub fn fields(mut self, fields: impl Into<OptionParam<String>>) -> Self {
        self.fields = fields.into();
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Eq, PartialEq)]
pub struct AdriveOpenFileGetRequestPost {
    pub drive_id: String,
    pub file_id: String,
    pub video_thumbnail_time: Option<i64>,
    pub video_thumbnail_width: Option<i64>,
    pub image_thumbnail_width: Option<i64>,
    pub fields: Option<String>,
}

impl AdriveOpenFileGetRequest {
    pub async fn request(&self) -> crate::Result<AdriveOpenFileGet> {
        let params = AdriveOpenFileGetRequestPost {
            drive_id: if let Some(drive_id) = self.drive_id.deref() {
                drive_id.clone()
            } else {
                return Err(Error::require_param_missing("drive_id"));
            },
            file_id: if let Some(file_id) = self.file_id.deref() {
                file_id.clone()
            } else {
                return Err(Error::require_param_missing("file_id"));
            },
            video_thumbnail_time: self.video_thumbnail_time.deref().clone(),
            video_thumbnail_width: self.video_thumbnail_width.deref().clone(),
            image_thumbnail_width: self.image_thumbnail_width.deref().clone(),
            fields: self.fields.deref().clone(),
        };
        let resp = self
            .agent
            .post(format!("{}/adrive/v1.0/openFile/get", self.api_host.as_str()).as_str())
            .load_access_token(self.access_token.clone())
            .await?
            .json(&params)
            .send()
            .await?;
        response(resp).await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct AdriveOpenFileGet {
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
    pub r#type: AdriveOpenFileType,
    pub thumbnail: Option<String>,
    pub url: Option<String>,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
    pub items: Option<Vec<Value>>,
    pub id_path: Option<String>,
    pub name_path: Option<String>,
}
