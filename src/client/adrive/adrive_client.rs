use std::sync::Arc;

use crate::client::common::access_token_loader::BoxedAccessTokenLoader;
use crate::define::DEFAULT_API_HOST;
use tokio::sync::Mutex;

#[derive(Debug)]
pub struct AdriveClient {
    pub api_host: Mutex<Arc<String>>,
    pub agent: Mutex<Arc<reqwest::Client>>,
    pub client_id: Mutex<Arc<String>>,
    pub access_token_loader: Mutex<Arc<BoxedAccessTokenLoader>>,
}

impl Default for AdriveClient {
    fn default() -> Self {
        Self {
            api_host: Mutex::new(Arc::new(DEFAULT_API_HOST.to_string())),
            agent: Mutex::new(Arc::new(reqwest::Client::new())),
            client_id: Mutex::new(Arc::new("".to_string())),
            access_token_loader: Mutex::new(Arc::new(Box::new(
                crate::access_token_loader::UninitializedAccessTokenLoader {},
            ))),
        }
    }
}

impl AdriveClient {
    pub async fn set_client_id(self, client_id: impl Into<Arc<String>>) -> Self {
        *self.client_id.lock().await = client_id.into();
        self
    }

    pub async fn set_api_host(self, api_host: impl Into<Arc<String>>) -> Self {
        *self.api_host.lock().await = api_host.into();
        self
    }

    pub async fn set_agent(self, agent: impl Into<Arc<reqwest::Client>>) -> Self {
        *self.agent.lock().await = agent.into();
        self
    }

    pub async fn set_access_token_loader(
        self,
        access_token_loader: BoxedAccessTokenLoader,
    ) -> Self {
        *self.access_token_loader.lock().await = Arc::new(access_token_loader);
        self
    }

    pub(crate) async fn clone_agent(&self) -> Arc<reqwest::Client> {
        self.agent.lock().await.clone()
    }

    pub(crate) async fn clone_api_host(&self) -> Arc<String> {
        self.api_host.lock().await.clone()
    }

    pub(crate) async fn clone_access_token_loader(&self) -> Arc<BoxedAccessTokenLoader> {
        self.access_token_loader.lock().await.clone()
    }
}
