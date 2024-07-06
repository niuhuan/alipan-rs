use crate::access_token_store::{AccessToken, AccessTokenStore};
use crate::{BoxedError, Client};
use async_trait::async_trait;

#[derive(Debug)]
pub struct FileAccessTokenStore(String);

impl FileAccessTokenStore {
    pub fn new(path: &str) -> Self {
        FileAccessTokenStore(path.to_string())
    }
}

#[async_trait]
impl AccessTokenStore for FileAccessTokenStore {
    async fn get_access_token(&self) -> std::result::Result<Option<AccessToken>, BoxedError> {
        let content = tokio::fs::read_to_string(&self.0).await?;
        let token: AccessToken = serde_json::from_str(content.as_str())?;
        Ok(Some(token))
    }

    async fn set_access_token(
        &self,
        access_token: AccessToken,
    ) -> std::result::Result<(), BoxedError> {
        let content = serde_json::to_string(&access_token)?;
        tokio::fs::write(&self.0, content).await?;
        Ok(())
    }
}

async fn client() -> Client {
    Client::default()
        .set_client_id(option_env!("client_id").unwrap_or(""))
        .await
        .set_client_secret(option_env!("client_secret").unwrap_or(""))
        .await
        .set_access_token_store(Box::new(FileAccessTokenStore::new(
            "target/access_token.json",
        )))
        .await
}

#[tokio::test]
async fn test_oauth_authorize() -> anyhow::Result<()> {
    let client = client().await;
    let url = client
        .api_oauth_authorize()
        .await
        .redirect_uri("http://localhost:58443/oauth_authorize")
        .scope("user:base,file:all:read,file:all:write,album:shared:read")
        .build()?;
    println!("{}", url);
    Ok(())
}

#[tokio::test]
async fn test_oauth_access_token() -> anyhow::Result<()> {
    let code = load_file("target/code.txt").await?;
    client()
        .await
        .client_oauth_parse_code(code.as_str())
        .await?;
    Ok(())
}

async fn load_file(path: &str) -> anyhow::Result<String> {
    use tokio::fs::File;
    use tokio::io::AsyncReadExt;
    let mut file = File::open(path).await?;
    let mut content = String::new();
    file.read_to_string(&mut content).await?;
    Ok(content.trim().to_string())
}

#[tokio::test]
async fn test_oauth_users_info() -> anyhow::Result<()> {
    let client = client().await;
    let users_info = client.api_oauth_users_info().await.request().await?;
    println!("{:?}", users_info);
    Ok(())
}
