use crate::AccessTokenLoader;
use async_trait::async_trait;
use reqwest::RequestBuilder;
use std::sync::Arc;

#[async_trait]
pub trait LoadAccessToken<T> {
    async fn load_access_token(self, loader: Arc<Box<dyn AccessTokenLoader>>) -> crate::Result<T>;
}

#[async_trait]
impl LoadAccessToken<RequestBuilder> for RequestBuilder {
    async fn load_access_token(
        self,
        loader: Arc<Box<dyn AccessTokenLoader>>,
    ) -> crate::Result<Self> {
        let token = loader.get_access_token().await?;
        Ok(self.header("Authorization", format!("Bearer {}", token.access_token)))
    }
}
