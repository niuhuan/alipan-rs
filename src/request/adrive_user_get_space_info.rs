use crate::request::response;
use crate::{AccessTokenLoader, AdriveUserGetSpaceInfo};
use std::sync::Arc;

pub struct AdriveUserGetSpaceInfoRequest {
    pub agent: Arc<reqwest::Client>,
    pub api_host: Arc<String>,
    pub access_token: AccessTokenLoader,
}

impl AdriveUserGetSpaceInfoRequest {
    pub fn agent(mut self, agent: Arc<reqwest::Client>) -> Self {
        self.agent = agent;
        self
    }

    pub fn api_host(mut self, api_host: impl Into<String>) -> Self {
        self.api_host = Arc::new(api_host.into());
        self
    }

    pub async fn request(&self) -> crate::Result<AdriveUserGetSpaceInfo> {
        let token = self.access_token.load_access_token().await?;
        let resp = self
            .agent
            .post(format!("{}/adrive/v1.0/user/getSpaceInfo", self.api_host.as_str()).as_str())
            .header("Authorization", format!("Bearer {}", token.access_token))
            .send()
            .await?;
        response(resp).await
    }
}
