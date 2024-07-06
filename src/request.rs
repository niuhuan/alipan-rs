use super::response::*;
use crate::access_token_store::{AccessToken, BoxedAccessTokenStore};
use crate::common::GrantType;
use crate::{AlipanError, Error, OauthAccessToken, Result};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug)]
pub struct OauthAuthorizeUrl {
    pub api_host: Arc<String>,
    /// 创建应用时分配的 appId
    pub client_id: Arc<String>,
    /// 授权后要回调的 URI，即接收 Authorization Code的 URI。请使用 urlEncode 对链接进行处理。云盘会把授权后的 code 放到 redirect_uri URL参数上。示例 redirect_uri&code={code}
    pub redirect_uri: String,
    /// 申请的授权范围，多个用逗号分隔。示例 user:base,file:all:read 详见 https://www.yuque.com/aliyundrive/zpfszx/dspik0
    pub scope: String,
    /// 仅支持 code
    pub response_type: String,
    /// 用于保持请求和回调的状态，授权服务器在回调时（重定向用户浏览器到“redirect_uri”时），会在Query Parameter中原样回传该参数。OAuth2.0标准协议推荐，利用state参数来防止CSRF攻击。
    pub state: Option<String>,
    /// h5 下 true 强制用户登录，默认 false
    pub relogin: Option<bool>,
    /// 指定必选的drive,  backup 或 resource。多个用 , 分隔。例如：backup,resource
    pub drive: Option<String>,
}

impl OauthAuthorizeUrl {
    pub fn redirect_uri(mut self, redirect_uri: impl Into<String>) -> Self {
        self.redirect_uri = redirect_uri.into();
        self
    }
    pub fn scope(mut self, scope: impl Into<String>) -> Self {
        self.scope = scope.into();
        self
    }
    pub fn response_type(mut self, response_type: impl Into<String>) -> Self {
        self.response_type = response_type.into();
        self
    }
    pub fn state(mut self, state: impl Into<String>) -> Self {
        self.state = Some(state.into());
        self
    }
    pub fn build(&self) -> Result<String> {
        if self.client_id.is_empty() {
            return Err(Error::msg("client_id is required"));
        }
        if self.redirect_uri.is_empty() {
            return Err(Error::msg("redirect_uri is required"));
        }
        if self.scope.is_empty() {
            return Err(Error::msg("scope is required"));
        }
        let mut url = url::Url::parse(self.api_host.as_str())?;
        url.set_path("/oauth/authorize");
        url.query_pairs_mut()
            .append_pair("client_id", self.client_id.as_str())
            .append_pair("redirect_uri", self.redirect_uri.as_str())
            .append_pair("scope", self.scope.as_str())
            .append_pair("response_type", self.response_type.as_str());
        if let Some(state) = &self.state {
            url.query_pairs_mut().append_pair("state", state.as_str());
        }
        if let Some(relogin) = &self.relogin {
            url.query_pairs_mut()
                .append_pair("relogin", relogin.to_string().as_str());
        }
        if let Some(drive) = &self.drive {
            url.query_pairs_mut().append_pair("drive", drive.as_str());
        }
        Ok(url.to_string())
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
    pub grant_type: GrantType,
    // 身份类型 authorization_code 或 refresh_token
    pub code: Option<String>,
    pub refresh_token: Option<String>,
    pub code_verifier: Option<String>,
}

impl OauthAccessTokenRequest {
    pub fn grant_type(mut self, grant_type: impl Into<GrantType>) -> Self {
        self.grant_type = grant_type.into();
        self
    }

    pub fn code(mut self, code: impl Into<String>) -> Self {
        self.code = Some(code.into());
        self
    }
    pub fn refresh_token(mut self, refresh_token: impl Into<String>) -> Self {
        self.refresh_token = Some(refresh_token.into());
        self
    }
    pub fn code_verifier(mut self, code_verifier: impl Into<String>) -> Self {
        self.code_verifier = Some(code_verifier.into());
        self
    }
    pub async fn request(&self) -> Result<OauthAccessToken> {
        let mut form = HashMap::<&str, &str>::new();
        form.insert("client_id", self.client_id.as_str());
        form.insert("client_secret", self.client_secret.as_str());
        match self.grant_type {
            GrantType::AuthorizationCode => {
                if let Some(code) = &self.code {
                    form.insert("code", code.as_str());
                } else {
                    return Err(AlipanError::msg(
                        "code is required for authorization_code grant_type",
                    ));
                }
                if let Some(code_verifier) = &self.code_verifier {
                    form.insert("code_verifier", code_verifier.as_str());
                }
            }
            GrantType::RefreshToken => {
                if let Some(refresh_token) = &self.refresh_token {
                    form.insert("refresh_token", refresh_token.as_str());
                } else {
                    return Err(AlipanError::msg(
                        "refresh_token is required for refresh_token grant_type",
                    ));
                }
            }
            _ => {
                return Err(AlipanError::msg("grant_type is required"));
            }
        }
        form.insert("grant_type", self.grant_type.as_str());
        let resp = self
            .agent
            .post(format!("{}/oauth/access_token", self.api_host.as_str()).as_str())
            .form(&form)
            .send()
            .await?;
        response(resp).await
    }
}

async fn response<T: for<'de> serde::Deserialize<'de>>(response: reqwest::Response) -> Result<T> {
    let code = response.status();
    let text = response.text().await?;
    if !code.is_success() {
        return Err(AlipanError::server(code, text.as_str()));
    }
    let data: T = serde_json::from_str(&text)?;
    Ok(data)
}

pub struct AccessTokenLoader {
    pub agent: Arc<reqwest::Client>,
    pub api_host: Arc<String>,
    pub client_id: Arc<String>,
    pub client_secret: Arc<String>,
    pub access_token_store: Arc<BoxedAccessTokenStore>,
}

impl AccessTokenLoader {
    pub async fn load_access_token(&self) -> Result<AccessToken> {
        let token = self.access_token_store.get_access_token().await?;
        let token = match token {
            Some(token) => {
                let now = chrono::Utc::now().timestamp();
                if now - token.created_at < token.expires_in * 3 / 4 {
                    return Ok(token);
                }
                token
            }
            None => return Err(Error::msg("no access token")),
        };
        let token = OauthAccessTokenRequest {
            agent: self.agent.clone(),
            api_host: self.api_host.clone(),
            client_id: self.client_id.clone(),
            client_secret: self.client_secret.clone(),
            grant_type: GrantType::RefreshToken,
            code: None,
            refresh_token: Some(token.refresh_token),
            code_verifier: None,
        }
        .request()
        .await?;
        let access_token = AccessToken::wrap_oauth_token(token);
        self.access_token_store
            .set_access_token(access_token.clone())
            .await?;
        Ok(access_token)
    }
}

pub struct OauthUsersInfoRequest {
    pub agent: Arc<reqwest::Client>,
    pub api_host: Arc<String>,
    pub access_token: AccessTokenLoader,
}

impl OauthUsersInfoRequest {
    pub async fn request(&self) -> Result<OauthUsersInfo> {
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
