pub mod adrive_open_file_create;
pub mod adrive_open_file_list;
pub mod adrive_user_get_drive_info;
pub mod adrive_user_get_space_info;
pub mod oauth_access_token;
pub mod oauth_authorize_url;
pub mod oauth_users_info;
pub mod oauth_users_scopes;

pub use adrive_open_file_create::*;
pub use adrive_open_file_list::*;
pub use adrive_user_get_drive_info::*;
pub use adrive_user_get_space_info::*;
pub use oauth_access_token::*;
pub use oauth_authorize_url::*;
pub use oauth_users_info::*;
pub use oauth_users_scopes::*;

use crate::access_token_store::{AccessToken, BoxedAccessTokenStore};
use crate::common::GrantType;
use crate::{AlipanError, Error, Result};
use std::sync::Arc;

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
            grant_type: GrantType::RefreshToken.into(),
            code: None.into(),
            refresh_token: token.refresh_token.into(),
            code_verifier: None.into(),
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
