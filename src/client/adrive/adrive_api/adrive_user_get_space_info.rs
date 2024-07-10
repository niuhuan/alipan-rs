use crate::{response, AccessTokenLoader, AdriveClient, AdriveUserGetSpaceInfo};
use std::sync::Arc;

impl AdriveClient {
    pub async fn adrive_user_get_space_info(&self) -> AdriveUserGetSpaceInfoRequest {
        AdriveUserGetSpaceInfoRequest {
            agent: self.clone_agent().await,
            api_host: self.clone_api_host().await,
            access_token: self.clone_access_token_loader().await,
        }
    }
}

pub struct AdriveUserGetSpaceInfoRequest {
    pub agent: Arc<reqwest::Client>,
    pub api_host: Arc<String>,
    pub access_token: Arc<Box<dyn AccessTokenLoader>>,
}

impl AdriveUserGetSpaceInfoRequest {
    pub fn agent(mut self, agent: impl Into<Arc<reqwest::Client>>) -> Self {
        self.agent = agent.into();
        self
    }

    pub fn api_host(mut self, api_host: impl Into<Arc<String>>) -> Self {
        self.api_host = api_host.into();
        self
    }

    pub async fn request(&self) -> crate::Result<AdriveUserGetSpaceInfo> {
        let token = self.access_token.get_access_token().await?;
        let resp = self
            .agent
            .post(format!("{}/adrive/v1.0/user/getSpaceInfo", self.api_host.as_str()).as_str())
            .header("Authorization", format!("Bearer {}", token.access_token))
            .send()
            .await?;
        response(resp).await
    }
}
