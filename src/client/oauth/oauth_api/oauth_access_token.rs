use crate::{response, AlipanError, GrantType, OAuthClient, OptionParam};
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;

impl OAuthClient {
    pub async fn oauth_access_token(&self) -> OauthAccessTokenRequest {
        OauthAccessTokenRequest {
            agent: self.clone_agent().await,
            api_host: self.clone_api_host().await,
            client_id: self.clone_client_id().await,
            client_secret: self.clone_client_secret().await,
            grant_type: None.into(),
            code: None.into(),
            refresh_token: None.into(),
            code_verifier: None.into(),
        }
    }
}

#[derive(Debug)]
pub struct OauthAccessTokenRequest {
    /// agent
    pub agent: Arc<reqwest::Client>,
    /// API Host
    pub api_host: Arc<String>,
    /// 创建应用时分配的 appId
    pub client_id: Arc<String>,
    /// 创建应用时分配的 appSecret
    pub client_secret: Arc<String>,
    /// 仅支持 authorization_code
    pub grant_type: OptionParam<GrantType>,
    // 身份类型 authorization_code 或 refresh_token
    pub code: OptionParam<String>,
    pub refresh_token: OptionParam<String>,
    pub code_verifier: OptionParam<String>,
}

impl OauthAccessTokenRequest {
    pub fn agent(mut self, agent: impl Into<Arc<reqwest::Client>>) -> Self {
        self.agent = agent.into();
        self
    }

    pub fn api_host(mut self, api_host: impl Into<Arc<String>>) -> Self {
        self.api_host = api_host.into();
        self
    }

    pub fn client_id(mut self, client_id: impl Into<String>) -> Self {
        self.client_id = Arc::new(client_id.into());
        self
    }

    pub fn client_secret(mut self, client_secret: impl Into<String>) -> Self {
        self.client_secret = Arc::new(client_secret.into());
        self
    }

    pub fn grant_type(mut self, grant_type: impl Into<OptionParam<GrantType>>) -> Self {
        self.grant_type = grant_type.into();
        self
    }

    pub fn code(mut self, code: impl Into<OptionParam<String>>) -> Self {
        self.code = code.into();
        self
    }

    pub fn refresh_token(mut self, refresh_token: impl Into<OptionParam<String>>) -> Self {
        self.refresh_token = refresh_token.into();
        self
    }

    pub fn code_verifier(mut self, code_verifier: impl Into<OptionParam<String>>) -> Self {
        self.code_verifier = code_verifier.into();
        self
    }

    pub async fn request(&self) -> crate::Result<OauthAccessToken> {
        let mut form = HashMap::<&str, &str>::new();
        form.insert("client_id", self.client_id.as_str());
        form.insert("client_secret", self.client_secret.as_str());
        if let Some(grant_type) = self.grant_type.deref() {
            match grant_type {
                GrantType::AuthorizationCode => {
                    if let Some(code) = &self.code.deref() {
                        form.insert("code", code.as_str());
                    } else {
                        return Err(AlipanError::require_param_missing("code"));
                    }
                    if let Some(code_verifier) = &self.code_verifier.deref() {
                        form.insert("code_verifier", code_verifier.as_str());
                    }
                }
                GrantType::RefreshToken => {
                    if let Some(refresh_token) = &self.refresh_token.deref() {
                        form.insert("refresh_token", refresh_token.as_str());
                    } else {
                        return Err(AlipanError::require_param_missing("refresh_token"));
                    }
                }
            }
            form.insert("grant_type", self.grant_type.unwrap().as_str());
        } else {
            return Err(AlipanError::require_param_missing("grant_type"));
        }
        let resp = self
            .agent
            .post(format!("{}/oauth/access_token", self.api_host.as_str()).as_str())
            .form(&form)
            .send()
            .await?;
        response(resp).await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OauthAccessToken {
    /// token_type
    pub token_type: String,
    /// access_token
    pub access_token: String,
    /// refresh_token
    pub refresh_token: String,
    /// expires_in
    pub expires_in: i64,
}
