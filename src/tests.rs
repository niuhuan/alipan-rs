use crate::client::common::access_token_loader::{AccessToken, AccessTokenStore};
use crate::{AdriveClient, AdriveOpenFileType, BoxedError, CheckNameMode};
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

async fn client() -> AdriveClient {
    AdriveClient::default()
        .set_client_id(option_env!("client_id").unwrap_or(""))
        .await
        .set_client_secret(option_env!("client_secret").unwrap_or(""))
        .await
        .set_access_token_store(Box::new(FileAccessTokenStore::new(
            "target/access_token.json",
        )))
        .await
}

async fn drive_id() -> anyhow::Result<String> {
    let content = tokio::fs::read_to_string("target/drive_id.txt").await?;
    Ok(content.trim().to_string())
}

#[tokio::test]
async fn test_oauth_authorize() -> anyhow::Result<()> {
    let client = client().await;
    let url = client
        .oauth_authorize()
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
    let users_info = client.oauth_users_info().await.request().await?;
    println!("{:?}", users_info);
    Ok(())
}

#[tokio::test]
async fn test_oauth_users_scopes() -> anyhow::Result<()> {
    let client = client().await;
    let users_scopes = client.oauth_users_scopes().await.request().await?;
    println!("{:?}", users_scopes);
    Ok(())
}

#[tokio::test]
async fn test_adrive_user_get_drive_info() -> anyhow::Result<()> {
    let client = client().await;
    let drive_info = client.adrive_user_get_drive_info().await.request().await?;
    println!("{:?}", drive_info);
    Ok(())
}

#[tokio::test]
async fn test_adrive_user_get_space_info() -> anyhow::Result<()> {
    let client = client().await;
    let space_info = client.adrive_user_get_space_info().await.request().await?;
    println!("{:?}", space_info);
    Ok(())
}

#[tokio::test]
async fn test_adrive_open_file_list() -> anyhow::Result<()> {
    let client = client().await;
    let open_file_list = client
        .adrive_open_file_list()
        .await
        .drive_id(drive_id().await?)
        .request()
        .await?;
    println!("{:?}", open_file_list);
    Ok(())
}

#[tokio::test]
async fn test_adrive_open_file_create() -> anyhow::Result<()> {
    let client = client().await;
    let drive_id = drive_id().await?;
    let parent_file_id = "root".to_string();
    let name = "test_folder".to_string();
    let r#type = AdriveOpenFileType::Folder;
    let check_name_mode = CheckNameMode::Refuse;
    let content_hash = "".to_string();
    let open_file_create = client
        .adrive_open_file_create()
        .await
        .drive_id(drive_id)
        .parent_file_id(parent_file_id)
        .name(name)
        .r#type(r#type)
        .check_name_mode(check_name_mode)
        .content_hash(content_hash)
        .request()
        .await?;
    println!("{:?}", open_file_create);
    println!("{}", serde_json::to_string(&open_file_create)?);
    Ok(())
}
