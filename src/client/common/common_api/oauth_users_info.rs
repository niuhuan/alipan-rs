use crate::{
    response, AccessTokenLoader, AdriveClient, LoadAccessToken, OAuthClient, OauthUsersInfo,
};
use std::sync::Arc;

impl OAuthClient {
    pub async fn oauth_users_info(&self) -> OauthUsersInfoRequest {
        OauthUsersInfoRequest {
            agent: self.clone_agent().await,
            api_host: self.clone_api_host().await,
            access_token: Arc::new(Box::new(
                crate::client::common::access_token_loader::UninitializedAccessTokenLoader {},
            )),
        }
    }
}

impl AdriveClient {
    pub async fn oauth_users_info(&self) -> OauthUsersInfoRequest {
        OauthUsersInfoRequest {
            agent: self.clone_agent().await,
            api_host: self.clone_api_host().await,
            access_token: self.clone_access_token_loader().await,
        }
    }
}

pub struct OauthUsersInfoRequest {
    pub agent: Arc<reqwest::Client>,
    pub api_host: Arc<String>,
    pub access_token: Arc<Box<dyn AccessTokenLoader>>,
}

impl OauthUsersInfoRequest {
    pub fn agent(mut self, agent: impl Into<Arc<reqwest::Client>>) -> Self {
        self.agent = agent.into();
        self
    }

    pub fn api_host(mut self, api_host: impl Into<Arc<String>>) -> Self {
        self.api_host = api_host.into();
        self
    }

    pub async fn request(&self) -> crate::Result<OauthUsersInfo> {
        let resp = self
            .agent
            .get(format!("{}/oauth/users/info", self.api_host.as_str()).as_str())
            .load_access_token(self.access_token.clone())
            .await?
            .send()
            .await?;
        response(resp).await
    }
}
