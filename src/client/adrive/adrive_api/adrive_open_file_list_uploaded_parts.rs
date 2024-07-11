use crate::{response, AdriveClient, BoxedAccessTokenLoader, LoadAccessToken, OptionParam};
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use std::ops::Deref;
use std::sync::Arc;

impl AdriveClient {
    pub async fn adrive_open_file_list_uploaded_parts(
        &self,
    ) -> AdriveOpenFileListUploadedPartsRequest {
        AdriveOpenFileListUploadedPartsRequest {
            agent: self.clone_agent().await,
            api_host: self.clone_api_host().await,
            access_token: self.clone_access_token_loader().await,
            drive_id: None.into(),
            file_id: None.into(),
            upload_id: None.into(),
            part_number_marker: None.into(),
        }
    }
}

pub struct AdriveOpenFileListUploadedPartsRequest {
    pub agent: Arc<reqwest::Client>,
    pub api_host: Arc<String>,
    pub access_token: Arc<BoxedAccessTokenLoader>,
    pub drive_id: OptionParam<String>,
    pub file_id: OptionParam<String>,
    pub upload_id: OptionParam<String>,
    pub part_number_marker: OptionParam<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Eq, PartialEq)]
pub struct AdriveOpenFileListUploadedPartsRequestPost {
    pub drive_id: Option<String>,
    pub file_id: Option<String>,
    pub upload_id: Option<String>,
    pub part_number_marker: Option<String>,
}

impl AdriveOpenFileListUploadedPartsRequest {
    pub fn agent(mut self, agent: impl Into<Arc<reqwest::Client>>) -> Self {
        self.agent = agent.into();
        self
    }

    pub fn api_host(mut self, api_host: impl Into<Arc<String>>) -> Self {
        self.api_host = api_host.into();
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
}

impl AdriveOpenFileListUploadedPartsRequest {
    pub async fn request(&self) -> crate::Result<AdriveOpenFileListUploadedParts> {
        let resp = self
            .agent
            .post(
                format!(
                    "{}/adrive/v1.0/openFile/listUploadedParts",
                    self.api_host.as_str()
                )
                .as_str(),
            )
            .load_access_token(self.access_token.clone())
            .await?
            .json(&AdriveOpenFileListUploadedPartsRequestPost {
                drive_id: if let Some(v) = &self.drive_id.deref() {
                    Some(v.clone())
                } else {
                    return Err(crate::Error::require_param_missing("drive_id"));
                },
                file_id: if let Some(v) = &self.file_id.deref() {
                    Some(v.clone())
                } else {
                    return Err(crate::Error::require_param_missing("file_id"));
                },
                upload_id: self.upload_id.clone().into(),
                part_number_marker: self.part_number_marker.clone(),
            })
            .send()
            .await?;
        response(resp).await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Eq, PartialEq)]
pub struct AdriveOpenFileListUploadedParts {
    #[serde(default)]
    pub drive_id: String,
    pub upload_id: String,
    #[serde(rename = "parallelUpload")]
    pub parallel_upload: bool,
    pub uploaded_parts: Vec<Value>,
    pub next_part_number_marker: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Eq, PartialEq)]
pub struct UploadedParts {
    pub content_type: String,
    pub etag: Option<String>,
    pub part_number: i64,
    pub part_size: i64,
    pub upload_form_info: Option<Value>,
    pub upload_url: String,
}
