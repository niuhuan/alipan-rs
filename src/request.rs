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
    pub fn api_host(mut self, api_host: impl Into<String>) -> Self {
        self.api_host = Arc::new(api_host.into());
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
    let data: T = from_str(&text)?;
    Ok(data)
}

pub fn from_str<T: for<'de> serde::Deserialize<'de>>(json: &str) -> Result<T> {
    Ok(serde_path_to_error::deserialize(
        &mut serde_json::Deserializer::from_str(json),
    )?)
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
    pub fn agent(mut self, agent: Arc<reqwest::Client>) -> Self {
        self.agent = agent;
        self
    }

    pub fn api_host(mut self, api_host: impl Into<String>) -> Self {
        self.api_host = Arc::new(api_host.into());
        self
    }

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

    pub async fn request(&self) -> Result<OauthUsersScopes> {
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

pub struct AdriveUserGetDriveInfoRequest {
    pub agent: Arc<reqwest::Client>,
    pub api_host: Arc<String>,
    pub access_token: AccessTokenLoader,
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

    pub async fn request(&self) -> Result<AdriveUserGetDriveInfo> {
        let token = self.access_token.load_access_token().await?;
        let resp = self
            .agent
            .post(format!("{}/adrive/v1.0/user/getDriveInfo", self.api_host.as_str()).as_str())
            .header("Authorization", format!("Bearer {}", token.access_token))
            .send()
            .await?;
        response(resp).await
    }
}

pub struct AdriveOpenFileListRequest {
    pub agent: Arc<reqwest::Client>,
    pub api_host: Arc<String>,
    pub access_token: AccessTokenLoader,
    pub drive_id: String,
    pub limit: Option<i64>,
    pub marker: Option<String>,
    pub order_by: Option<String>,
    pub order_direction: Option<String>,
    pub parent_file_id: String,
    pub category: Option<String>,
    pub file_type: Option<String>,
    pub video_thumbnail_time: Option<i64>,
    pub video_thumbnail_width: Option<i64>,
    pub image_thumbnail_width: Option<i64>,
    pub fields: Option<String>,
}

impl AdriveOpenFileListRequest {
    pub fn agent(mut self, agent: Arc<reqwest::Client>) -> Self {
        self.agent = agent;
        self
    }

    pub fn api_host(mut self, api_host: impl Into<String>) -> Self {
        self.api_host = Arc::new(api_host.into());
        self
    }

    pub fn drive_id(mut self, drive_id: impl Into<String>) -> Self {
        self.drive_id = drive_id.into();
        self
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn marker(mut self, marker: impl Into<String>) -> Self {
        self.marker = Some(marker.into());
        self
    }

    pub fn order_by(mut self, order_by: impl Into<String>) -> Self {
        self.order_by = Some(order_by.into());
        self
    }

    pub fn order_direction(mut self, order_direction: impl Into<String>) -> Self {
        self.order_direction = Some(order_direction.into());
        self
    }

    pub fn parent_file_id(mut self, parent_file_id: impl Into<String>) -> Self {
        self.parent_file_id = parent_file_id.into();
        self
    }

    pub fn category(mut self, category: impl Into<String>) -> Self {
        self.category = Some(category.into());
        self
    }

    pub fn file_type(mut self, file_type: impl Into<String>) -> Self {
        self.file_type = Some(file_type.into());
        self
    }

    pub fn video_thumbnail_time(mut self, video_thumbnail_time: i64) -> Self {
        self.video_thumbnail_time = Some(video_thumbnail_time);
        self
    }

    pub fn video_thumbnail_width(mut self, video_thumbnail_width: i64) -> Self {
        self.video_thumbnail_width = Some(video_thumbnail_width);
        self
    }

    pub fn image_thumbnail_width(mut self, image_thumbnail_width: i64) -> Self {
        self.image_thumbnail_width = Some(image_thumbnail_width);
        self
    }

    pub fn fields(mut self, fields: impl Into<String>) -> Self {
        self.fields = Some(fields.into());
        self
    }

    pub async fn request(&self) -> Result<AdriveOpenFileList> {
        if self.drive_id.is_empty() {
            return Err(Error::msg("drive_id is required"));
        }
        if self.parent_file_id.is_empty() {
            return Err(Error::msg("parent_file_id is required"));
        }
        let mut form = HashMap::<&str, String>::new();
        form.insert("drive_id", self.drive_id.clone());
        if let Some(limit) = self.limit {
            form.insert("limit", limit.to_string());
        }
        if let Some(marker) = &self.marker {
            form.insert("marker", marker.clone());
        }
        if let Some(order_by) = &self.order_by {
            form.insert("order_by", order_by.clone());
        }
        if let Some(order_direction) = &self.order_direction {
            form.insert("order_direction", order_direction.clone());
        }
        form.insert("parent_file_id", self.parent_file_id.clone());
        if let Some(category) = &self.category {
            form.insert("category", category.clone());
        }
        if let Some(file_type) = &self.file_type {
            form.insert("type", file_type.clone());
        }
        if let Some(video_thumbnail_time) = self.video_thumbnail_time {
            form.insert("video_thumbnail_time", video_thumbnail_time.to_string());
        }
        if let Some(video_thumbnail_width) = self.video_thumbnail_width {
            form.insert("video_thumbnail_width", video_thumbnail_width.to_string());
        }
        if let Some(image_thumbnail_width) = self.image_thumbnail_width {
            form.insert("image_thumbnail_width", image_thumbnail_width.to_string());
        }
        if let Some(fields) = &self.fields {
            form.insert("fields", fields.clone());
        }
        let token = self.access_token.load_access_token().await?;
        let url = url::Url::parse(
            format!("{}/adrive/v1.0/openFile/list", self.api_host.as_str()).as_str(),
        )?;
        let resp = self
            .agent
            .post(url)
            .header("Authorization", format!("Bearer {}", token.access_token))
            .json(&form)
            .send()
            .await?;
        response(resp).await
    }
}

/*
名称	类型	是否必填	说明
drive_id	string	必填	drive id
limit 	integer 	选填	返回文件数量，默认 50，最大 100
marker	string	选填	分页标记
order_by	string	选填	created_at
updated_at
name
size
name_enhanced（对数字编号的文件友好，排序结果为 1、2、3...99 而不是 1、10、11...2、21...9、91...99）
order_direction	string	选填	DESC ASC
parent_file_id	string	必填	根目录为root
category 	string	选填	分类，目前有枚举：video | doc | audio | zip | others | image
可任意组合，按照逗号分割，例如 video,doc,audio
image,doc
type	string	选填	all | file | folder，
默认所有类型
type为folder时，category不做检查
video_thumbnail_time	number	选填	生成的视频缩略图截帧时间，单位ms，默认120000ms
video_thumbnail_width	number	选填	生成的视频缩略图宽度，默认480px
image_thumbnail_width	number	选填	生成的图片缩略图宽度，默认480px
fields	string	选填	当填 * 时，返回文件所有字段。或某些字段，逗号分隔： id_path,name_path


 */
