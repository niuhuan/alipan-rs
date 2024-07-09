use crate::request::response;
use crate::{AlipanError, GrantType, OauthAccessToken, OptionParam};
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;

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
    pub fn agent(mut self, agent: Arc<reqwest::Client>) -> Self {
        self.agent = agent;
        self
    }

    pub fn api_host(mut self, api_host: impl Into<String>) -> Self {
        self.api_host = Arc::new(api_host.into());
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
                        return Err(AlipanError::msg(
                            "code is required for authorization_code grant_type",
                        ));
                    }
                    if let Some(code_verifier) = &self.code_verifier.deref() {
                        form.insert("code_verifier", code_verifier.as_str());
                    }
                }
                GrantType::RefreshToken => {
                    if let Some(refresh_token) = &self.refresh_token.deref() {
                        form.insert("refresh_token", refresh_token.as_str());
                    } else {
                        return Err(AlipanError::msg(
                            "refresh_token is required for refresh_token grant_type",
                        ));
                    }
                }
            }
            form.insert("grant_type", self.grant_type.unwrap().as_str());
        } else {
            return Err(AlipanError::msg("grant_type is required"));
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
