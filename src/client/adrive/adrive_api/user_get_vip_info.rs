use crate::{AccessTokenLoader, AdriveClient, LoadAccessToken};
use serde_derive::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::Arc;

impl AdriveClient {
    pub async fn user_get_vip_info(&self) -> UserGetVipInfoRequest {
        UserGetVipInfoRequest {
            agent: self.clone_agent().await,
            api_host: self.clone_api_host().await,
            access_token: self.clone_access_token_loader().await,
        }
    }
}

#[derive(Debug)]
pub struct UserGetVipInfoRequest {
    pub agent: Arc<reqwest::Client>,
    pub api_host: Arc<String>,
    pub access_token: Arc<Box<dyn AccessTokenLoader>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserGetVipInfoPost {}

impl UserGetVipInfoRequest {
    pub fn agent(mut self, agent: impl Into<Arc<reqwest::Client>>) -> Self {
        self.agent = agent.into();
        self
    }

    pub fn api_host(mut self, api_host: impl Into<Arc<String>>) -> Self {
        self.api_host = api_host.into();
        self
    }

    pub fn access_token(
        mut self,
        access_token: impl Into<Arc<Box<dyn AccessTokenLoader>>>,
    ) -> Self {
        self.access_token = access_token.into();
        self
    }
}

impl UserGetVipInfoRequest {
    pub async fn request(&self) -> crate::Result<UserGetVipInfo> {
        let url = format!("{}/v1.0/user/getVipInfo", self.api_host.deref().as_str());
        let resp = self
            .agent
            .post(url)
            .load_access_token(self.access_token.clone())
            .await?
            .send()
            .await?;
        let resp = resp.json::<UserGetVipInfo>().await?;
        Ok(resp)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserGetVipInfo {
    pub identity: String,
    pub level: Option<String>,
    pub expire: i64,
    pub third_party_vip: Option<bool>,
    pub third_party_vip_expire: Option<i64>,
}
