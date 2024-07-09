use crate::BoxedError;
use reqwest::Body;
use std::sync::Arc;
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

    pub fn channel_resource() -> (Sender<Result<Vec<u8>, BoxedError>>, Body) {
        let (sender, receiver) = tokio::sync::mpsc::channel::<Result<Vec<u8>, BoxedError>>(1);
        let body = Body::wrap_stream(tokio_stream::wrappers::ReceiverStream::new(receiver));
        (sender, body)
    }

    pub fn bytes_body(bytes: Vec<u8>) -> Body {
        Body::from(bytes)
    }
}
