use crate::request::response;
use crate::{AccessTokenLoader, OauthUsersScopes};
use std::sync::Arc;

pub struct OauthUsersScopesRequest {
    pub agent: Arc<reqwest::Client>,
    pub api_host: Arc<String>,
    pub access_token: AccessTokenLoader,
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
        let token = self.access_token.load_access_token().await?;
        let resp = self
            .agent
            .get(format!("{}/oauth/users/scopes", self.api_host.as_str()).as_str())
            .header("Authorization", format!("Bearer {}", token.access_token))
            .send()
            .await?;
        response(resp).await
    }
}
