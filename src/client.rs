use crate::common::*;
use crate::request::*;
use std::sync::Arc;

use crate::access_token_store::{AccessToken, BoxedAccessTokenStore};
use crate::result::*;
use tokio::sync::Mutex;

const DEFAULT_API_HOST: &str = "https://openapi.alipan.com";

#[derive(Debug)]
pub struct Client {
    pub api_host: Mutex<Arc<String>>,
    pub agent: Mutex<Arc<reqwest::Client>>,
    pub client_id: Mutex<Arc<String>>,
    pub client_secret: Mutex<Arc<String>>,
    pub access_token_store: Mutex<Arc<BoxedAccessTokenStore>>,
}

impl Default for Client {
    fn default() -> Self {
        Self {
            api_host: Mutex::new(Arc::new(DEFAULT_API_HOST.to_string())),
            agent: Mutex::new(Arc::new(reqwest::Client::new())),
            client_id: Mutex::new(Arc::new("".to_string())),
            client_secret: Mutex::new(Arc::new("".to_string())),
            access_token_store: Mutex::new(Arc::new(Box::new(
                crate::access_token_store::InMemoryAccessTokenStore::new(),
            ))),
        }
    }
}

impl Client {
    pub async fn set_client_id(self, client_id: impl Into<String>) -> Self {
        *self.client_id.lock().await = Arc::new(client_id.into());
        self
    }

    pub async fn set_client_secret(self, client_secret: impl Into<String>) -> Self {
        *self.client_secret.lock().await = Arc::new(client_secret.into());
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

    pub async fn set_access_token_store(self, access_token_store: BoxedAccessTokenStore) -> Self {
        *self.access_token_store.lock().await = Arc::new(access_token_store);
        self
    }

    pub async fn oauth_authorize(&self) -> OauthAuthorizeUrl {
        OauthAuthorizeUrl {
            api_host: self.api_host.lock().await.clone(),
            client_id: self.client_id.lock().await.clone(),
            redirect_uri: "".to_string(),
            scope: "".to_string(),
            response_type: "code".to_string(),
            state: None,
            relogin: None,
            drive: None,
        }
    }

    pub async fn oauth_access_token(&self) -> OauthAccessTokenRequest {
        OauthAccessTokenRequest {
            agent: self.agent.lock().await.clone(),
            api_host: self.api_host.lock().await.clone(),
            client_id: self.client_id.lock().await.clone(),
            client_secret: self.client_secret.lock().await.clone(),
            grant_type: None.into(),
            code: None.into(),
            refresh_token: None.into(),
            code_verifier: None.into(),
        }
    }

    pub async fn oauth_users_info(&self) -> OauthUsersInfoRequest {
        OauthUsersInfoRequest {
            agent: self.clone_agent().await,
            api_host: self.clone_api_host().await,
            access_token: self.access_token_loader().await,
        }
    }

    pub async fn oauth_users_scopes(&self) -> OauthUsersScopesRequest {
        OauthUsersScopesRequest {
            agent: self.clone_agent().await,
            api_host: self.clone_api_host().await,
            access_token: self.access_token_loader().await,
        }
    }

    pub async fn adrive_user_get_drive_info(&self) -> AdriveUserGetDriveInfoRequest {
        AdriveUserGetDriveInfoRequest {
            agent: self.clone_agent().await,
            api_host: self.clone_api_host().await,
            access_token: self.access_token_loader().await,
        }
    }

    pub async fn adrive_user_get_space_info(&self) -> AdriveUserGetSpaceInfoRequest {
        AdriveUserGetSpaceInfoRequest {
            agent: self.clone_agent().await,
            api_host: self.clone_api_host().await,
            access_token: self.access_token_loader().await,
        }
    }

    pub async fn adrive_open_file_list(&self) -> AdriveOpenFileListRequest {
        AdriveOpenFileListRequest {
            agent: self.clone_agent().await,
            api_host: self.clone_api_host().await,
            access_token: self.access_token_loader().await,
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
            access_token: self.access_token_loader().await,
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

    async fn access_token_loader(&self) -> AccessTokenLoader {
        AccessTokenLoader {
            agent: self.agent.lock().await.clone(),
            api_host: self.api_host.lock().await.clone(),
            client_id: self.client_id.lock().await.clone(),
            client_secret: self.client_secret.lock().await.clone(),
            access_token_store: self.access_token_store.lock().await.clone(),
        }
    }

    pub async fn clone_agent(&self) -> Arc<reqwest::Client> {
        self.agent.lock().await.clone()
    }

    async fn clone_api_host(&self) -> Arc<String> {
        self.api_host.lock().await.clone()
    }

    pub async fn client_oauth_parse_code(&self, code: &str) -> Result<AccessToken> {
        let token = self
            .oauth_access_token()
            .await
            .code(code)
            .grant_type(GrantType::AuthorizationCode)
            .request()
            .await?;
        let access_token = AccessToken::wrap_oauth_token(token);
        let token_store = self.access_token_store.lock().await;
        token_store.set_access_token(access_token.clone()).await?;
        Ok(access_token)
    }

    pub async fn client_oauth_parse_refresh_token(
        &self,
        refresh_token: &str,
    ) -> Result<AccessToken> {
        let token = self
            .oauth_access_token()
            .await
            .refresh_token(refresh_token)
            .grant_type(GrantType::RefreshToken)
            .request()
            .await?;
        let access_token = AccessToken::wrap_oauth_token(token);
        let token_store = self.access_token_store.lock().await;
        token_store.set_access_token(access_token.clone()).await?;
        Ok(access_token)
    }
}
