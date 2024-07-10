use crate::{response, AccessTokenLoader, AdriveClient, OAuthClient, OauthUsersScopes};
use std::sync::Arc;

impl OAuthClient {
    pub async fn oauth_users_scopes(&self) -> OauthUsersScopesRequest {
        OauthUsersScopesRequest {
            agent: self.clone_agent().await,
            api_host: self.clone_api_host().await,
            access_token: Arc::new(Box::new(
                crate::client::common::access_token_loader::UninitializedAccessTokenLoader {},
            )),
        }
    }
}

impl AdriveClient {
    pub async fn oauth_users_scopes(&self) -> OauthUsersScopesRequest {
        OauthUsersScopesRequest {
            agent: self.clone_agent().await,
            api_host: self.clone_api_host().await,
            access_token: self.clone_access_token_loader().await,
        }
    }
}

pub struct OauthUsersScopesRequest {
    pub agent: Arc<reqwest::Client>,
    pub api_host: Arc<String>,
    pub access_token: Arc<Box<dyn AccessTokenLoader>>,
}

impl OauthUsersScopesRequest {
    pub fn agent(mut self, agent: Arc<reqwest::Client>) -> Self {
        self.agent = agent;
        self
    }

    pub fn api_host(mut self, api_host: impl Into<String>) -> Self {
        self.api_host = Arc::new(api_host.into());
        self
    }

    pub async fn request(&self) -> crate::Result<OauthUsersScopes> {
        let token = self.access_token.get_access_token().await?;
        let resp = self
            .agent
            .get(format!("{}/oauth/users/scopes", self.api_host.as_str()).as_str())
            .header("Authorization", format!("Bearer {}", token.access_token))
            .send()
            .await?;
        response(resp).await
    }
}
