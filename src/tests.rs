use crate::client::common::access_token_loader::AccessToken;
use crate::{
    AdriveClient, AdriveOpenFileBatchGetRequestFileList, AdriveOpenFilePartInfo,
    AdriveOpenFileType, BoxedAccessTokenLoader, CheckNameMode, GrantType, OAuthClient,
    OAuthClientAccessTokenManager, OAuthClientAccessTokenStore,
};
use anyhow::{anyhow, Context};
use async_trait::async_trait;
use serde_derive::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncSeekExt};
// 使用文件保管客户端信息，方便调试

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ClientInfo {
    pub client_id: String,
    pub client_secret: String,
}

const CLIENT_INFO_JSON_PATH: &str = "target/client_info.json";

pub fn load_client_info() -> anyhow::Result<ClientInfo> {
    let content = std::fs::read_to_string(CLIENT_INFO_JSON_PATH)?;
    let client_info: ClientInfo = serde_json::from_str(content.as_str())?;
    Ok(client_info)
}

// 使用文件存储访问令牌

#[derive(Debug)]
pub struct FileAccessTokenStore(String);

impl FileAccessTokenStore {
    pub fn new(path: &str) -> Self {
        FileAccessTokenStore(path.to_string())
    }
}

#[async_trait]
impl OAuthClientAccessTokenStore for FileAccessTokenStore {
    async fn get_access_token(&self) -> anyhow::Result<Option<AccessToken>> {
        let content = tokio::fs::read_to_string(&self.0).await?;
        let token: AccessToken = serde_json::from_str(content.as_str())?;
        Ok(Some(token))
    }

    async fn set_access_token(&self, access_token: AccessToken) -> anyhow::Result<()> {
        let content = serde_json::to_string(&access_token)?;
        tokio::fs::write(&self.0, content).await?;
        Ok(())
    }
}

const ACCESS_TOKEN_JSON_PATH: &str = "target/access_token.json";

async fn access_token_loader() -> BoxedAccessTokenLoader {
    Box::new(OAuthClientAccessTokenManager {
        oauth_client: Arc::new(oauth_client().await),
        access_token_store: Arc::new(Box::new(FileAccessTokenStore::new(ACCESS_TOKEN_JSON_PATH))),
    })
}

// 构建客户端

async fn oauth_client() -> OAuthClient {
    let client_info = load_client_info().expect("load client info error");
    OAuthClient::default()
        .set_client_id(client_info.client_id)
        .await
        .set_client_secret(client_info.client_secret)
        .await
}

async fn client() -> AdriveClient {
    let client_info = load_client_info().expect("load client info error");
    AdriveClient::default()
        .set_client_id(client_info.client_id)
        .await
        .set_access_token_loader(access_token_loader().await)
        .await
        .set_agent(reqwest::Client::new())
        .await
}

// 测试内容

async fn drive_id() -> anyhow::Result<String> {
    let content = tokio::fs::read_to_string("target/drive_id.txt").await?;
    Ok(content.trim().to_string())
}

#[tokio::test]
async fn test_oauth_authorize() -> anyhow::Result<()> {
    let url = oauth_client()
        .await
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
    oauth_client()
        .await
        .oauth_access_token()
        .await
        .grant_type(GrantType::AuthorizationCode)
        .code(code)
        .request()
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
async fn test_user_get_vip_info() -> anyhow::Result<()> {
    let client = client().await;
    let vip_info = client.user_get_vip_info().await.request().await?;
    println!("{:?}", vip_info);
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
async fn test_adrive_open_file_get() -> anyhow::Result<()> {
    let client = client().await;
    let open_file_get = client
        .adrive_open_file_get()
        .await
        .drive_id(drive_id().await?)
        .file_id("file_id".to_string())
        .request()
        .await?;
    println!("{:?}", open_file_get);
    Ok(())
}

#[tokio::test]
async fn test_adrive_open_file_get_by_path() -> anyhow::Result<()> {
    let client = client().await;
    let open_file_get_by_path = client
        .adrive_open_file_get_by_path()
        .await
        .drive_id(drive_id().await?)
        .file_path("/test.txt".to_string())
        .request()
        .await?;
    println!("{:?}", open_file_get_by_path);
    Ok(())
}

#[tokio::test]
async fn test_adrive_open_file_batch_get() -> anyhow::Result<()> {
    let client = client().await;
    let open_file_batch_get = client
        .adrive_open_file_batch_get()
        .await
        .file_list(vec![
            AdriveOpenFileBatchGetRequestFileList {
                drive_id: drive_id().await?,
                file_id: "file_id".to_string(),
            },
            AdriveOpenFileBatchGetRequestFileList {
                drive_id: drive_id().await?,
                file_id: "file_id".to_string(),
            },
        ])
        .request()
        .await?;
    println!("{:?}", open_file_batch_get);
    Ok(())
}

#[tokio::test]
async fn test_adrive_open_file_get_download_url() -> anyhow::Result<()> {
    let client = client().await;
    let open_file_get_download_url = client
        .adrive_open_file_get_download_url()
        .await
        .drive_id(drive_id().await?)
        .file_id("file_id".to_string())
        .expire_sec(3600)
        .request()
        .await?;
    println!("{:?}", open_file_get_download_url);
    Ok(())
}

#[tokio::test]
async fn test_adrive_open_file_create_folder() -> anyhow::Result<()> {
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

const TEXT: &str = "Hello, World!";

#[tokio::test]
async fn test_adrive_open_file_create_file_rapid_upload() -> anyhow::Result<()> {
    let open_file_create = crate::tests::client()
        .await
        .adrive_open_file_create()
        .await
        .drive_id(crate::tests::drive_id().await?)
        .parent_file_id("root".to_string())
        .name("test.txt")
        .r#type(AdriveOpenFileType::File)
        .check_name_mode(CheckNameMode::Refuse)
        .size(TEXT.len() as i64)
        .content_hash_name("sha1")
        .content_hash(sha1(TEXT.as_bytes()))
        .request()
        .await?;
    println!("{:?}", open_file_create);
    println!("{}", serde_json::to_string(&open_file_create)?);
    Ok(())
}

fn sha1(bytes: &[u8]) -> String {
    use sha1::Digest;
    let mut hasher = sha1::Sha1::new();
    hasher.update(bytes);
    let result = hasher.finalize();
    hex::encode(result)
}

#[tokio::test]
async fn test_adrive_open_file_create_file_upload() -> anyhow::Result<()> {
    let open_file_create = crate::tests::client()
        .await
        .adrive_open_file_create()
        .await
        .drive_id(crate::tests::drive_id().await?)
        .parent_file_id("root".to_string())
        .name("test.txt")
        .r#type(AdriveOpenFileType::File)
        .check_name_mode(CheckNameMode::Refuse)
        .size(TEXT.len() as i64)
        .request()
        .await?;
    println!("{:?}", open_file_create);
    println!("{}", serde_json::to_string(&open_file_create)?);
    Ok(())
}

const LARGE_FILE_PATH: &str = "/Volumes/CACHE/A,系统/Windows/16,ARM/22000.1165.221013-1538.CO_RELEASE_SVC_PROD3_CLIENTPRO_OEMRET_A64FRE_EN-US.ISO";
const LARGE_FILE_NAME: &str =
    "22000.1165.221013-1538.CO_RELEASE_SVC_PROD3_CLIENTPRO_OEMRET_A64FRE_EN-US.ISO";

// 分片序列号，从 1 开始 单个文件分片最大限制5GB，最小限制 100KB

// 通过channel传递, 读取文件，分片上传, (更好的效果应该是直接复写reader只读指定区域进行分片)

#[tokio::test]
async fn test_adrive_open_file_create_large_upload() -> anyhow::Result<()> {
    let client = client().await;
    // 计算文件的sha1
    println!("sha1");
    let sha1 = sha1_file(LARGE_FILE_PATH).await?;
    println!("{}", sha1);
    // 获取文件大小
    let md = tokio::fs::metadata(LARGE_FILE_PATH).await?;
    let size = md.len() as i64;
    // 每1GB分片
    let part_size: i64 = 1 << 30;
    let part_count = size / part_size + if size % part_size == 0 { 0 } else { 1 };
    // 创建文件
    let parts = (1..=part_count)
        .map(|i| AdriveOpenFilePartInfo {
            part_number: i as i64,
        })
        .collect::<Vec<_>>();
    let open_file_create = client
        .adrive_open_file_create()
        .await
        .drive_id(crate::tests::drive_id().await?)
        .parent_file_id("root".to_string())
        .name(LARGE_FILE_NAME)
        .r#type(AdriveOpenFileType::File)
        .check_name_mode(CheckNameMode::Refuse)
        .part_info_list(parts.clone())
        .content_hash_name("sha1")
        .content_hash(sha1)
        .size(size as i64)
        .request()
        .await?;
    println!("{}", serde_json::to_string(&open_file_create)?);
    if open_file_create.exist {
        return Err(anyhow!(" d"));
    }
    // 确定有没有秒传
    if open_file_create.rapid_upload {
        println!("rapid upload");
        return Ok(());
    }
    // 上传分片

    for x in parts {
        println!("part_number: {}", x.part_number);
        let upload_urls = client
            .adrive_open_file_get_upload_url()
            .await
            .drive_id(open_file_create.drive_id.as_str())
            .file_id(open_file_create.file_id.as_str())
            .upload_id(
                open_file_create
                    .upload_id
                    .clone()
                    .with_context(|| "upload id ")?
                    .as_str(),
            )
            .part_info_list(vec![AdriveOpenFilePartInfo {
                part_number: x.part_number,
            }])
            .request()
            .await?;
        // steam skip and read_len
        let skip = (x.part_number - 1) * part_size;
        let read_len = if x.part_number == part_count {
            size - skip
        } else {
            part_size
        };
        // url
        let url = upload_urls.part_info_list[0].upload_url.as_str();
        // 上传
        put_file(url, LARGE_FILE_PATH, skip, read_len).await?;
    }
    println!("upload success");
    //
    client
        .adrive_open_file_complete()
        .await
        .drive_id(open_file_create.drive_id.as_str())
        .file_id(open_file_create.file_id.as_str())
        .upload_id(
            open_file_create
                .upload_id
                .with_context(|| "upload_id")?
                .as_str(),
        )
        .request()
        .await?;
    //
    println!("complete success");
    Ok(())
}

async fn put_file(url: &str, path: &str, skip: i64, read_len: i64) -> anyhow::Result<()> {
    let (sender, body) = PutResource::channel_resource();
    let request = reqwest::Client::new().put(url).body(body).send();
    let cp = sender.clone();
    let read_file_back = async move {
        let result = put_steam(cp, path, skip, read_len).await;
        if let Err(e) = result {
            let _ = sender.send(Err(e)).await;
        }
    };
    let (send, _read) = tokio::join!(request, read_file_back);
    send?.error_for_status()?;
    Ok(())
}

async fn put_steam(
    sender: tokio::sync::mpsc::Sender<anyhow::Result<Vec<u8>>>,
    path: &str,
    skip: i64,
    read_len: i64,
) -> anyhow::Result<()> {
    let mut buffer = vec![0u8; 1 << 10];
    let mut has_read = 0;
    let file = tokio::fs::File::open(path).await?;
    let mut reader = tokio::io::BufReader::new(file);
    reader.seek(tokio::io::SeekFrom::Start(skip as u64)).await?;
    while has_read < read_len {
        let n = reader.read(&mut buffer).await?;
        if n == 0 {
            break;
        }
        has_read += n as i64;
        if has_read > read_len {
            let n = n - (has_read - read_len) as usize;
            sender.send(Ok(buffer[..n].to_vec())).await?;
            break;
        } else {
            sender.send(Ok(buffer[..n].to_vec())).await?;
        }
    }
    Ok(())
}

async fn sha1_file(file: &str) -> anyhow::Result<String> {
    use sha1::Digest;
    let mut hasher = sha1::Sha1::new();
    let file = tokio::fs::File::open(file).await?;
    let mut reader = tokio::io::BufReader::new(file);
    let mut buffer = [0u8; 1024];
    loop {
        let n = reader.read(&mut buffer).await?;
        if n == 0 {
            break;
        }
        hasher.update(&buffer[..n]);
    }
    let result = hasher.finalize();
    Ok(hex::encode(result))
}

#[tokio::test]
async fn test_adrive_open_file_get_upload_url() -> anyhow::Result<()> {
    let open_file_get_upload_url = crate::tests::client()
        .await
        .adrive_open_file_get_upload_url()
        .await
        .drive_id(crate::tests::drive_id().await?)
        .file_id("file_id".to_string())
        .upload_id("upload_id".to_string())
        .request()
        .await?;
    println!("{:?}", open_file_get_upload_url);
    println!("{}", serde_json::to_string(&open_file_get_upload_url)?);
    Ok(())
}

#[tokio::test]
async fn test_adrive_open_file_list_uploaded_parts() -> anyhow::Result<()> {
    let open_file_list_uploaded_parts = crate::tests::client()
        .await
        .adrive_open_file_list_uploaded_parts()
        .await
        .drive_id(crate::tests::drive_id().await?)
        .file_id("file_id".to_string())
        .upload_id("upload_id".to_string())
        .request()
        .await?;
    println!("{:?}", open_file_list_uploaded_parts);
    println!("{}", serde_json::to_string(&open_file_list_uploaded_parts)?);
    Ok(())
}

#[tokio::test]
async fn test_adrive_open_file_upload_part() -> anyhow::Result<()> {
    let (sender, body) = PutResource::channel_resource();
    let put_resource = PutResource {
        agent: Arc::new(reqwest::Client::new()),
        url: "https://cn-beijing-data.aliyundrive.net/xxxxx".to_string(),
        resource: body,
    };
    let (a, b) = tokio::join!(put_resource.put(), send(sender));
    a.unwrap();
    b.unwrap();
    Ok(())
}

async fn send(sender: tokio::sync::mpsc::Sender<anyhow::Result<Vec<u8>>>) -> anyhow::Result<()> {
    sender.send(Ok(TEXT.as_bytes().to_vec())).await?;
    Ok(())
}

#[tokio::test]
async fn test_adrive_open_file_complete() -> anyhow::Result<()> {
    let open_file_complete = crate::tests::client()
        .await
        .adrive_open_file_complete()
        .await
        .drive_id(crate::tests::drive_id().await?)
        .file_id("file_id".to_string())
        .upload_id("upload_id".to_string())
        .request()
        .await?;
    println!("{:?}", open_file_complete);
    println!("{}", serde_json::to_string(&open_file_complete)?);
    Ok(())
}

#[tokio::test]
async fn test_adrive_open_file_update() -> anyhow::Result<()> {
    let open_file_update = crate::tests::client()
        .await
        .adrive_open_file_update()
        .await
        .drive_id(crate::tests::drive_id().await?)
        .file_id("file_id".to_string())
        .name("test.txt".to_string())
        .check_name_mode(CheckNameMode::Refuse)
        .starred(false)
        .request()
        .await?;
    println!("{:?}", open_file_update);
    println!("{}", serde_json::to_string(&open_file_update)?);
    Ok(())
}

#[tokio::test]
async fn test_adrive_open_file_move() -> anyhow::Result<()> {
    let open_file_move = crate::tests::client()
        .await
        .adrive_open_file_move()
        .await
        .drive_id(crate::tests::drive_id().await?)
        .file_id("file_id".to_string())
        .to_parent_file_id("parent_file_id".to_string())
        .check_name_mode(CheckNameMode::Refuse)
        .new_name("new_name".to_string())
        .request()
        .await?;
    println!("{:?}", open_file_move);
    println!("{}", serde_json::to_string(&open_file_move)?);
    Ok(())
}

#[tokio::test]
async fn test_adrive_open_file_copy() -> anyhow::Result<()> {
    let open_file_copy = crate::tests::client()
        .await
        .adrive_open_file_copy()
        .await
        .drive_id(crate::tests::drive_id().await?)
        .file_id("file_id".to_string())
        .to_drive_id("to_drive_id".to_string())
        .to_parent_file_id("to_parent_file_id".to_string())
        .auto_rename(true)
        .request()
        .await?;
    println!("{:?}", open_file_copy);
    println!("{}", serde_json::to_string(&open_file_copy)?);
    Ok(())
}

#[tokio::test]
async fn test_adrive_open_file_recyclebin_trash() -> anyhow::Result<()> {
    let open_file_trash = crate::tests::client()
        .await
        .adrive_open_file_recyclebin_trash()
        .await
        .drive_id(crate::tests::drive_id().await?)
        .file_id("file_id".to_string())
        .request()
        .await?;
    println!("{:?}", open_file_trash);
    println!("{}", serde_json::to_string(&open_file_trash)?);
    Ok(())
}

#[tokio::test]
async fn test_adrive_open_file_delete() -> anyhow::Result<()> {
    let client = client().await;
    let open_file_restore = client
        .adrive_open_file_delete()
        .await
        .drive_id(crate::tests::drive_id().await?)
        .file_id("file_id".to_string())
        .request()
        .await?;
    println!("{:?}", open_file_restore);
    println!("{}", serde_json::to_string(&open_file_restore)?);
    Ok(())
}

use reqwest::Body;
use tokio::sync::mpsc::Sender;

pub struct PutResource {
    pub agent: Arc<reqwest::Client>,
    pub url: String,
    pub resource: Body,
}

impl PutResource {
    pub async fn put(self) -> crate::Result<()> {
        let text = self
            .agent
            .request(reqwest::Method::PUT, self.url.as_str())
            .body(self.resource)
            .send()
            .await?
            .text()
            .await?;
        println!("{}", text);
        Ok(())
    }
}

impl PutResource {
    pub async fn file_resource(path: &str) -> crate::Result<Body> {
        let file = tokio::fs::read(path).await?;
        Ok(Body::from(file))
    }

    pub fn channel_resource() -> (Sender<anyhow::Result<Vec<u8>>>, Body) {
        let (sender, receiver) = tokio::sync::mpsc::channel::<anyhow::Result<Vec<u8>>>(16);
        let body = Body::wrap_stream(tokio_stream::wrappers::ReceiverStream::new(receiver));
        (sender, body)
    }

    pub fn bytes_body(bytes: Vec<u8>) -> Body {
        Body::from(bytes)
    }
}
