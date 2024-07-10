use crate::{Error, OAuthClient};
use std::sync::Arc;

impl OAuthClient {
    pub async fn oauth_authorize(&self) -> OauthAuthorizeUrl {
        OauthAuthorizeUrl {
            api_host: self.api_host.lock().await.clone(),
            client_id: self.client_id.lock().await.clone(),
            redirect_uri: "".to_string(),
            scope: "".to_string(),
            response_type: "code".to_string(),
            state: None,
            relogin: None,
            drive: None,
        }
    }
}

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
    pub fn api_host(mut self, api_host: impl Into<Arc<String>>) -> Self {
        self.api_host = api_host.into();
        self
    }

    pub fn client_id(mut self, client_id: impl Into<String>) -> Self {
        self.client_id = Arc::new(client_id.into());
        self
    }

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

    pub fn relogin(mut self, relogin: bool) -> Self {
        self.relogin = Some(relogin);
        self
    }

    pub fn drive(mut self, drive: impl Into<String>) -> Self {
        self.drive = Some(drive.into());
        self
    }

    pub fn build(&self) -> crate::Result<String> {
        if self.client_id.is_empty() {
            return Err(Error::require_param_missing("client_id"));
        }
        if self.redirect_uri.is_empty() {
            return Err(Error::require_param_missing("redirect_uri"));
        }
        if self.scope.is_empty() {
            return Err(Error::require_param_missing("scope"));
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
