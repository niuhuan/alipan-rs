use crate::adrive_api::OauthAccessToken;
use crate::BoxedError;
use async_trait::async_trait;
use serde_derive::{Deserialize, Serialize};
use std::fmt::Debug;

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
