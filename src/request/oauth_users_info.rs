use crate::request::response;
use crate::{AccessTokenLoader, OauthUsersInfo};
use std::sync::Arc;

pub struct OauthUsersInfoRequest {
    pub agent: Arc<reqwest::Client>,
    pub api_host: Arc<String>,
    pub access_token: AccessTokenLoader,
}

impl OauthUsersInfoRequest {
    pub fn agent(mut self, agent: Arc<reqwest::Client>) -> Self {
        self.agent = agent;
        self
    }

    pub fn api_host(mut self, api_host: impl Into<String>) -> Self {
        self.api_host = Arc::new(api_host.into());
        self
    }

    pub async fn request(&self) -> crate::Result<OauthUsersInfo> {
        let token = self.access_token.load_access_token().await?;
        let resp = self
            .agent
            .get(format!("{}/oauth/users/info", self.api_host.as_str()).as_str())
            .header("Authorization", format!("Bearer {}", token.access_token))
            .send()
            .await?;
        response(resp).await
    }
}
