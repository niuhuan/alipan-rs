use serde_derive::{Deserialize, Serialize};

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
