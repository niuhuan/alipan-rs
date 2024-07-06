use crate::common::*;
use crate::request::*;
use std::sync::Arc;

use crate::access_token_store::{AccessToken, BoxedAccessTokenStore};
use crate::types::*;
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

    pub async fn api_oauth_authorize(&self) -> OauthAuthorizeUrl {
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

    pub async fn api_oauth_access_token(&self) -> OauthAccessTokenRequest {
        OauthAccessTokenRequest {
            agent: self.agent.lock().await.clone(),
            api_host: self.api_host.lock().await.clone(),
            client_id: self.client_id.lock().await.clone(),
            client_secret: self.client_secret.lock().await.clone(),
            grant_type: GrantType::None,
            code: None,
            refresh_token: None,
            code_verifier: None,
        }
    }

    pub async fn api_oauth_users_info(&self) -> OauthUsersInfoRequest {
        let agent = self.agent.lock().await.clone();
        let api_host = self.api_host.lock().await.clone();
        OauthUsersInfoRequest {
            agent: agent.clone(),
            api_host: api_host.clone(),
            access_token: AccessTokenLoader {
                agent,
                api_host,
                client_id: self.client_id.lock().await.clone(),
                client_secret: self.client_secret.lock().await.clone(),
                access_token_store: self.access_token_store.lock().await.clone(),
            },
        }
    }

    pub async fn client_oauth_parse_code(&self, code: &str) -> Result<AccessToken> {
        let token = self
            .api_oauth_access_token()
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
            .api_oauth_access_token()
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
