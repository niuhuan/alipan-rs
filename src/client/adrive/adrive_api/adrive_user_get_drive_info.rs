use crate::{response, AccessTokenLoader, AdriveClient, AdriveUserGetDriveInfo};
use std::sync::Arc;

impl AdriveClient {
    pub async fn adrive_user_get_drive_info(&self) -> AdriveUserGetDriveInfoRequest {
        AdriveUserGetDriveInfoRequest {
            agent: self.clone_agent().await,
            api_host: self.clone_api_host().await,
            access_token: self.clone_access_token_loader().await,
        }
    }
}

pub struct AdriveUserGetDriveInfoRequest {
    pub agent: Arc<reqwest::Client>,
    pub api_host: Arc<String>,
    pub access_token: Arc<Box<dyn AccessTokenLoader>>,
}

impl AdriveUserGetDriveInfoRequest {
    pub fn agent(mut self, agent: Arc<reqwest::Client>) -> Self {
        self.agent = agent;
        self
    }

    pub fn api_host(mut self, api_host: impl Into<String>) -> Self {
        self.api_host = Arc::new(api_host.into());
        self
    }

    pub async fn request(&self) -> crate::Result<AdriveUserGetDriveInfo> {
        let token = self.access_token.get_access_token().await?;
        let resp = self
            .agent
            .post(format!("{}/adrive/v1.0/user/getDriveInfo", self.api_host.as_str()).as_str())
            .header("Authorization", format!("Bearer {}", token.access_token))
            .send()
            .await?;
        response(resp).await
    }
}
