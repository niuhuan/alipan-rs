use crate::request::response;
use crate::{AccessTokenLoader, AdriveOpenFileList, Error};
use std::collections::HashMap;
use std::sync::Arc;

pub struct AdriveOpenFileListRequest {
    pub agent: Arc<reqwest::Client>,
    pub api_host: Arc<String>,
    pub access_token: AccessTokenLoader,
    pub drive_id: String,
    pub limit: Option<i64>,
    pub marker: Option<String>,
    pub order_by: Option<String>,
    pub order_direction: Option<String>,
    pub parent_file_id: String,
    pub category: Option<String>,
    pub file_type: Option<String>,
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

    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn marker(mut self, marker: impl Into<String>) -> Self {
        self.marker = Some(marker.into());
        self
    }

    pub fn order_by(mut self, order_by: impl Into<String>) -> Self {
        self.order_by = Some(order_by.into());
        self
    }

    pub fn order_direction(mut self, order_direction: impl Into<String>) -> Self {
        self.order_direction = Some(order_direction.into());
        self
    }

    pub fn parent_file_id(mut self, parent_file_id: impl Into<String>) -> Self {
        self.parent_file_id = parent_file_id.into();
        self
    }

    pub fn category(mut self, category: impl Into<String>) -> Self {
        self.category = Some(category.into());
        self
    }

    pub fn file_type(mut self, file_type: impl Into<String>) -> Self {
        self.file_type = Some(file_type.into());
        self
    }

    pub fn video_thumbnail_time(mut self, video_thumbnail_time: i64) -> Self {
        self.video_thumbnail_time = Some(video_thumbnail_time);
        self
    }

    pub fn video_thumbnail_width(mut self, video_thumbnail_width: i64) -> Self {
        self.video_thumbnail_width = Some(video_thumbnail_width);
        self
    }

    pub fn image_thumbnail_width(mut self, image_thumbnail_width: i64) -> Self {
        self.image_thumbnail_width = Some(image_thumbnail_width);
        self
    }

    pub fn fields(mut self, fields: impl Into<String>) -> Self {
        self.fields = Some(fields.into());
        self
    }

    pub async fn request(&self) -> crate::Result<AdriveOpenFileList> {
        if self.drive_id.is_empty() {
            return Err(Error::msg("drive_id is required"));
        }
        if self.parent_file_id.is_empty() {
            return Err(Error::msg("parent_file_id is required"));
        }
        let mut form = HashMap::<&str, String>::new();
        form.insert("drive_id", self.drive_id.clone());
        if let Some(limit) = self.limit {
            form.insert("limit", limit.to_string());
        }
        if let Some(marker) = &self.marker {
            form.insert("marker", marker.clone());
        }
        if let Some(order_by) = &self.order_by {
            form.insert("order_by", order_by.clone());
        }
        if let Some(order_direction) = &self.order_direction {
            form.insert("order_direction", order_direction.clone());
        }
        form.insert("parent_file_id", self.parent_file_id.clone());
        if let Some(category) = &self.category {
            form.insert("category", category.clone());
        }
        if let Some(file_type) = &self.file_type {
            form.insert("type", file_type.clone());
        }
        if let Some(video_thumbnail_time) = self.video_thumbnail_time {
            form.insert("video_thumbnail_time", video_thumbnail_time.to_string());
        }
        if let Some(video_thumbnail_width) = self.video_thumbnail_width {
            form.insert("video_thumbnail_width", video_thumbnail_width.to_string());
        }
        if let Some(image_thumbnail_width) = self.image_thumbnail_width {
            form.insert("image_thumbnail_width", image_thumbnail_width.to_string());
        }
        if let Some(fields) = &self.fields {
            form.insert("fields", fields.clone());
        }
        let token = self.access_token.load_access_token().await?;
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
