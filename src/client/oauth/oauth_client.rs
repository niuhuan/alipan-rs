use crate::define::DEFAULT_API_HOST;
use crate::oauth_access_token::OauthAccessTokenRequest;
use crate::{
    OauthAuthorizeUrl, OauthUsersInfoRequest, OauthUsersScopesRequest,
    UninitializedAccessTokenLoader,
};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct OAuthClient {
    pub api_host: Mutex<Arc<String>>,
    pub agent: Mutex<Arc<reqwest::Client>>,
    pub client_id: Mutex<Arc<String>>,
    pub client_secret: Mutex<Arc<String>>,
}

impl Default for OAuthClient {
    fn default() -> Self {
        Self {
            api_host: Mutex::new(Arc::new(DEFAULT_API_HOST.to_string())),
            agent: Mutex::new(Arc::new(reqwest::Client::new())),
            client_id: Mutex::new(Arc::new("".to_string())),
            client_secret: Mutex::new(Arc::new("".to_string())),
        }
    }
}

impl OAuthClient {
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
}

impl OAuthClient {
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
            access_token: Arc::new(Box::new(UninitializedAccessTokenLoader {})),
        }
    }

    pub async fn oauth_users_scopes(&self) -> OauthUsersScopesRequest {
        OauthUsersScopesRequest {
            agent: self.clone_agent().await,
            api_host: self.clone_api_host().await,
            access_token: Arc::new(Box::new(UninitializedAccessTokenLoader {})),
        }
    }

    async fn clone_agent(&self) -> Arc<reqwest::Client> {
        self.agent.lock().await.clone()
    }

    async fn clone_api_host(&self) -> Arc<String> {
        self.api_host.lock().await.clone()
    }
}
