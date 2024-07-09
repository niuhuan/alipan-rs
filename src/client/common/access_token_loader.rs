use crate::oauth_access_token::OauthAccessToken;
use crate::{BoxedError, GrantType, OAuthClient};
use async_trait::async_trait;
use serde_derive::{Deserialize, Serialize};
use std::fmt::Debug;
use std::sync::Arc;

pub type BoxedAccessTokenLoader = Box<dyn AccessTokenLoader>;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AccessToken {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub refresh_token: String,
    pub created_at: i64,
}

impl AccessToken {
    pub fn wrap_oauth_token(token: OauthAccessToken) -> Self {
        AccessToken {
            access_token: token.access_token,
            token_type: token.token_type,
            expires_in: token.expires_in,
            refresh_token: token.refresh_token,
            created_at: chrono::Utc::now().timestamp(),
        }
    }
}

#[async_trait]
pub trait AccessTokenLoader: Debug + Send + Sync {
    async fn get_access_token(&self) -> Result<AccessToken, BoxedError>;
}

#[derive(Debug)]
pub struct UninitializedAccessTokenLoader;

#[async_trait]
impl AccessTokenLoader for UninitializedAccessTokenLoader {
    async fn get_access_token(&self) -> Result<AccessToken, BoxedError> {
        Err("uninitialized access token loader".into())
    }
}

#[derive(Debug)]
pub struct OAuthClientAccessTokenManager {
    pub oauth_client: Arc<OAuthClient>,
    pub access_token_store: Arc<Box<dyn OAuthClientAccessTokenStore>>,
}

#[async_trait]
pub trait OAuthClientAccessTokenStore: Debug + Send + Sync {
    async fn get_access_token(&self) -> Result<Option<AccessToken>, BoxedError>;

    async fn set_access_token(&self, access_token: AccessToken) -> Result<(), BoxedError>;
}

#[async_trait]
impl AccessTokenLoader for OAuthClientAccessTokenManager {
    async fn get_access_token(&self) -> Result<AccessToken, BoxedError> {
        let token = self.access_token_store.get_access_token().await?;
        let token = match token {
            Some(token) => {
                let now = chrono::Utc::now().timestamp();
                if now - token.created_at < token.expires_in * 3 / 4 {
                    return Ok(token);
                }
                token
            }
            None => return Err("no access token".into()),
        };
        let token = self
            .oauth_client
            .oauth_access_token()
            .await
            .grant_type(GrantType::RefreshToken)
            .refresh_token(token.refresh_token.as_str())
            .request()
            .await?;
        let access_token = AccessToken::wrap_oauth_token(token);
        self.access_token_store
            .set_access_token(access_token.clone())
            .await?;
        Ok(access_token)
    }
}
