use crate::Result;
use crate::{response, AdriveOpenFilePartInfo, BoxedAccessTokenLoader, OptionParam};
use chrono::Local;
use serde_derive::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug)]
pub struct AdriveOpenFileGetUploadUrlRequest {
    pub agent: Arc<reqwest::Client>,
    pub api_host: Arc<String>,
    pub access_token: Arc<BoxedAccessTokenLoader>,
    pub drive_id: OptionParam<String>,
    pub file_id: OptionParam<String>,
    pub upload_id: OptionParam<String>,
    pub part_info_list: OptionParam<Vec<AdriveOpenFilePartInfo>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdriveOpenFileGetUploadUrlRequestPost {
    pub drive_id: Option<String>,
    pub file_id: Option<String>,
    pub upload_id: Option<String>,
    pub part_info_list: Option<Vec<AdriveOpenFilePartInfo>>,
}

impl AdriveOpenFileGetUploadUrlRequest {
    pub fn agent(mut self, agent: Arc<reqwest::Client>) -> Self {
        self.agent = agent;
        self
    }

    pub fn api_host(mut self, api_host: impl Into<String>) -> Self {
        self.api_host = Arc::new(api_host.into());
        self
    }

    pub fn access_token(mut self, access_token: Arc<BoxedAccessTokenLoader>) -> Self {
        self.access_token = access_token;
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

    pub fn part_info_list(
        mut self,
        part_info_list: impl Into<OptionParam<Vec<AdriveOpenFilePartInfo>>>,
    ) -> Self {
        self.part_info_list = part_info_list.into();
        self
    }
}

impl AdriveOpenFileGetUploadUrlRequest {
    pub async fn request(&self) -> Result<AdriveOpenFileGetUploadUrl> {
        let url = format!("{}/adrive/v1.0/openFile/getUploadUrl", self.api_host);
        let client = self.agent.clone();
        let access_token = self.access_token.clone();
        let mut request = client.post(&url).header(
            "Authorization",
            access_token.get_access_token().await?.access_token,
        );
        let post = AdriveOpenFileGetUploadUrlRequestPost {
            drive_id: self.drive_id.clone().into(),
            file_id: self.file_id.clone().into(),
            upload_id: self.upload_id.clone().into(),
            part_info_list: self.part_info_list.clone().into(),
        };
        request = request.json(&post);
        let rsp = request.send().await?;
        response(rsp).await
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdriveOpenFileGetUploadUrl {
    pub drive_id: String,
    pub file_id: String,
    pub upload_id: String,
    pub created_at: Option<chrono::DateTime<Local>>,
    pub part_info_list: Vec<AdriveOpenFilePartInfoUpload>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdriveOpenFilePartInfoUpload {
    pub part_number: i64,
    pub part_size: i64,
    pub upload_url: String,
}
