use std::sync::Arc;

use crate::adrive::adrive_api::{
    AdriveOpenFileCreateRequest, AdriveOpenFileListRequest, AdriveUserGetDriveInfoRequest,
    AdriveUserGetSpaceInfoRequest,
};
use crate::adrive_api::{
    AdriveOpenFileCompleteRequest, AdriveOpenFileGetUploadUrlRequest,
    AdriveOpenFileListUploadedPartsRequest,
};
use crate::client::common::access_token_loader::BoxedAccessTokenLoader;
use crate::define::DEFAULT_API_HOST;
use crate::{OauthUsersInfoRequest, OauthUsersScopesRequest};
use tokio::sync::Mutex;

#[derive(Debug)]
pub struct AdriveClient {
    pub api_host: Mutex<Arc<String>>,
    pub agent: Mutex<Arc<reqwest::Client>>,
    pub client_id: Mutex<Arc<String>>,
    pub access_token_loader: Mutex<Arc<BoxedAccessTokenLoader>>,
}

impl Default for AdriveClient {
    fn default() -> Self {
        Self {
            api_host: Mutex::new(Arc::new(DEFAULT_API_HOST.to_string())),
            agent: Mutex::new(Arc::new(reqwest::Client::new())),
            client_id: Mutex::new(Arc::new("".to_string())),
            access_token_loader: Mutex::new(Arc::new(Box::new(
                crate::access_token_loader::UninitializedAccessTokenLoader {},
            ))),
        }
    }
}

impl AdriveClient {
    pub async fn set_client_id(self, client_id: impl Into<String>) -> Self {
        *self.client_id.lock().await = Arc::new(client_id.into());
        self
    }

    pub async fn set_api_host(self, api_host: impl Into<String>) -> Self {
        *self.api_host.lock().await = Arc::new(api_host.into());
        self
    }

    pub async fn set_agent(self, agent: reqwest::Client) -> Self {
        *self.agent.lock().await = Arc::new(agent);
        self
    }

    pub async fn set_access_token_loader(
        self,
        access_token_loader: BoxedAccessTokenLoader,
    ) -> Self {
        *self.access_token_loader.lock().await = Arc::new(access_token_loader);
        self
    }

    pub async fn oauth_users_info(&self) -> OauthUsersInfoRequest {
        OauthUsersInfoRequest {
            agent: self.clone_agent().await,
            api_host: self.clone_api_host().await,
            access_token: self.clone_access_token_loader().await,
        }
    }

    pub async fn oauth_users_scopes(&self) -> OauthUsersScopesRequest {
        OauthUsersScopesRequest {
            agent: self.clone_agent().await,
            api_host: self.clone_api_host().await,
            access_token: self.clone_access_token_loader().await,
        }
    }

    pub async fn adrive_user_get_drive_info(&self) -> AdriveUserGetDriveInfoRequest {
        AdriveUserGetDriveInfoRequest {
            agent: self.clone_agent().await,
            api_host: self.clone_api_host().await,
            access_token: self.clone_access_token_loader().await,
        }
    }

    pub async fn adrive_user_get_space_info(&self) -> AdriveUserGetSpaceInfoRequest {
        AdriveUserGetSpaceInfoRequest {
            agent: self.clone_agent().await,
            api_host: self.clone_api_host().await,
            access_token: self.clone_access_token_loader().await,
        }
    }

    pub async fn adrive_open_file_list(&self) -> AdriveOpenFileListRequest {
        AdriveOpenFileListRequest {
            agent: self.clone_agent().await,
            api_host: self.clone_api_host().await,
            access_token: self.clone_access_token_loader().await,
            drive_id: "".to_string(),
            limit: None.into(),
            marker: None.into(),
            order_by: None.into(),
            order_direction: None.into(),
            parent_file_id: "root".to_string(),
            category: None.into(),
            r#type: None.into(),
            video_thumbnail_time: None.into(),
            video_thumbnail_width: None.into(),
            image_thumbnail_width: None.into(),
            fields: None.into(),
        }
    }

    pub async fn adrive_open_file_create(&self) -> AdriveOpenFileCreateRequest {
        AdriveOpenFileCreateRequest {
            agent: self.clone_agent().await,
            api_host: self.clone_api_host().await,
            access_token: self.clone_access_token_loader().await,
            drive_id: None.into(),
            parent_file_id: None.into(),
            name: None.into(),
            r#type: None.into(),
            check_name_mode: None.into(),
            part_info_list: None.into(),
            streams_info: None.into(),
            pre_hash: None.into(),
            size: None.into(),
            content_hash: None.into(),
            content_hash_name: None.into(),
            proof_code: None.into(),
            proof_version: None.into(),
            local_created_at: None.into(),
            local_modified_at: None.into(),
        }
    }

    pub async fn adrive_open_file_get_upload_url(&self) -> AdriveOpenFileGetUploadUrlRequest {
        AdriveOpenFileGetUploadUrlRequest {
            agent: self.clone_agent().await,
            api_host: self.clone_api_host().await,
            access_token: self.clone_access_token_loader().await,
            drive_id: None.into(),
            file_id: None.into(),
            upload_id: None.into(),
            part_info_list: None.into(),
        }
    }

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

    // /adrive/v1.0/openFile/complete

    pub async fn adrive_open_file_complete(&self) -> AdriveOpenFileCompleteRequest {
        AdriveOpenFileCompleteRequest {
            agent: self.clone_agent().await,
            api_host: self.clone_api_host().await,
            access_token: self.clone_access_token_loader().await,
            drive_id: None.into(),
            file_id: None.into(),
            upload_id: None.into(),
        }
    }

    async fn clone_agent(&self) -> Arc<reqwest::Client> {
        self.agent.lock().await.clone()
    }

    async fn clone_api_host(&self) -> Arc<String> {
        self.api_host.lock().await.clone()
    }

    async fn clone_access_token_loader(&self) -> Arc<BoxedAccessTokenLoader> {
        self.access_token_loader.lock().await.clone()
    }
}
