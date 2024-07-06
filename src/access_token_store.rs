use crate::{BoxedError, OauthAccessToken};
use async_trait::async_trait;
use serde_derive::{Deserialize, Serialize};
use std::fmt::Debug;
use std::ops::Deref;
use tokio::sync::Mutex;

pub type BoxedAccessTokenStore = Box<dyn AccessTokenStore>;

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
pub trait AccessTokenStore: Debug + Send + Sync {
    async fn get_access_token(&self) -> Result<Option<AccessToken>, BoxedError>;

    async fn set_access_token(&self, access_token: AccessToken) -> Result<(), BoxedError>;
}

#[derive(Debug)]
pub struct InMemoryAccessTokenStore {
    access_token: Mutex<Option<AccessToken>>,
}

impl InMemoryAccessTokenStore {
    pub fn new() -> Self {
        InMemoryAccessTokenStore {
            access_token: Mutex::new(None),
        }
    }
}

impl Default for InMemoryAccessTokenStore {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl AccessTokenStore for InMemoryAccessTokenStore {
    async fn get_access_token(&self) -> Result<Option<AccessToken>, BoxedError> {
        Ok(self.access_token.lock().await.deref().clone())
    }

    async fn set_access_token(&self, access_token: AccessToken) -> Result<(), BoxedError> {
        *self.access_token.lock().await = Some(access_token);
        Ok(())
    }
}
