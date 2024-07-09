use crate::common::*;
use std::sync::Arc;

use crate::adrive::adrive_api::{
    AdriveOpenFileCreateRequest, AdriveOpenFileListRequest, AdriveUserGetDriveInfoRequest,
    AdriveUserGetSpaceInfoRequest,
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

    pub async fn set_access_token_store(self, access_token_store: BoxedAccessTokenLoader) -> Self {
        *self.access_token_loader.lock().await = Arc::new(access_token_store);
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

    // /adrive/v1.0/openFile/create

    pub async fn adrive_open_file_create(&self) -> AdriveOpenFileCreateRequest {
        AdriveOpenFileCreateRequest {
            agent: self.clone_agent().await,
            api_host: self.clone_api_host().await,
            access_token: self.clone_access_token_loader().await,
            drive_id: "".to_string(),
            parent_file_id: "root".to_string(),
            name: "".to_string(),
            r#type: AdriveOpenFileType::File,
            check_name_mode: CheckNameMode::None,
            part_info_list: None,
            streams_info: None,
            pre_hash: None,
            size: None,
            content_hash: None,
            content_hash_name: None,
            proof_code: None,
            proof_version: None,
            local_created_at: None,
            local_modified_at: None,
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