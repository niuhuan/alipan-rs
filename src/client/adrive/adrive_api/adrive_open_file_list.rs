use crate::{
    response, AccessTokenLoader, AdriveOpenFileList, AdriveOpenFileType, Error, OptionParam,
};
use serde_derive::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::Arc;

pub struct AdriveOpenFileListRequest {
    pub agent: Arc<reqwest::Client>,
    pub api_host: Arc<String>,
    pub access_token: Arc<Box<dyn AccessTokenLoader>>,
    pub drive_id: String,
    pub limit: OptionParam<i64>,
    pub marker: OptionParam<String>,
    pub order_by: OptionParam<String>,
    pub order_direction: OptionParam<String>,
    pub parent_file_id: String,
    pub category: OptionParam<String>,
    pub r#type: OptionParam<AdriveOpenFileType>,
    pub video_thumbnail_time: OptionParam<i64>,
    pub video_thumbnail_width: OptionParam<i64>,
    pub image_thumbnail_width: OptionParam<i64>,
    pub fields: OptionParam<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdriveOpenFileListRequestPost {
    pub drive_id: String,
    pub limit: Option<i64>,
    pub marker: Option<String>,
    pub order_by: Option<String>,
    pub order_direction: Option<String>,
    pub parent_file_id: String,
    pub category: Option<String>,
    pub r#type: Option<AdriveOpenFileType>,
    pub video_thumbnail_time: Option<i64>,
    pub video_thumbnail_width: Option<i64>,
    pub image_thumbnail_width: Option<i64>,
    pub fields: Option<String>,
}

impl AdriveOpenFileListRequest {
    pub fn agent(mut self, agent: Arc<reqwest::Client>) -> Self {
        self.agent = agent;
        self
    }

    pub fn api_host(mut self, api_host: impl Into<String>) -> Self {
        self.api_host = Arc::new(api_host.into());
        self
    }

    pub fn drive_id(mut self, drive_id: impl Into<String>) -> Self {
        self.drive_id = drive_id.into();
        self
    }

    pub fn limit(mut self, limit: impl Into<OptionParam<i64>>) -> Self {
        self.limit = limit.into();
        self
    }

    pub fn marker(mut self, marker: impl Into<OptionParam<String>>) -> Self {
        self.marker = marker.into();
        self
    }

    pub fn order_by(mut self, order_by: impl Into<OptionParam<String>>) -> Self {
        self.order_by = order_by.into();
        self
    }

    pub fn order_direction(mut self, order_direction: impl Into<OptionParam<String>>) -> Self {
        self.order_direction = order_direction.into();
        self
    }

    pub fn parent_file_id(mut self, parent_file_id: impl Into<String>) -> Self {
        self.parent_file_id = parent_file_id.into();
        self
    }

    pub fn category(mut self, category: impl Into<OptionParam<String>>) -> Self {
        self.category = category.into();
        self
    }

    pub fn r#type(mut self, r#type: impl Into<OptionParam<AdriveOpenFileType>>) -> Self {
        self.r#type = r#type.into();
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

    pub async fn request(&self) -> crate::Result<AdriveOpenFileList> {
        if self.drive_id.is_empty() {
            return Err(Error::msg("drive_id is required"));
        }
        if self.parent_file_id.is_empty() {
            return Err(Error::msg("parent_file_id is required"));
        }
        let form = AdriveOpenFileListRequestPost {
            drive_id: self.drive_id.clone(),
            limit: self.limit.deref().clone(),
            marker: self.marker.deref().clone(),
            order_by: self.order_by.deref().clone(),
            order_direction: self.order_direction.deref().clone(),
            parent_file_id: self.parent_file_id.clone(),
            category: self.category.deref().clone(),
            r#type: self.r#type.deref().clone(),
            video_thumbnail_time: self.video_thumbnail_time.deref().clone(),
            video_thumbnail_width: self.video_thumbnail_width.deref().clone(),
            image_thumbnail_width: self.image_thumbnail_width.deref().clone(),
            fields: self.fields.deref().clone(),
        };
        let token = self.access_token.get_access_token().await?;
        let url = url::Url::parse(
            format!("{}/adrive/v1.0/openFile/list", self.api_host.as_str()).as_str(),
        )?;
        let resp = self
            .agent
            .post(url)
            .header("Authorization", format!("Bearer {}", token.access_token))
            .json(&form)
            .send()
            .await?;
        response(resp).await
    }
}
