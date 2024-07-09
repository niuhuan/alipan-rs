use reqwest::StatusCode;
use serde_derive::{Deserialize, Serialize};
#[cfg(feature = "backtrace")]
use std::backtrace::Backtrace;
use std::fmt::{Debug, Display, Formatter};

pub type Error = AlipanError;
pub type Result<T> = std::result::Result<T, Error>;
pub type BoxedError = Box<dyn std::error::Error + Send + Sync>;

#[derive(Debug)]
pub struct AlipanError {
    #[cfg(feature = "backtrace")]
    pub backtrace: Backtrace,
    pub inner: ErrorInfo,
}

impl AlipanError {
    pub fn new(info: ErrorInfo) -> Self {
        AlipanError {
            #[cfg(feature = "backtrace")]
            backtrace: Backtrace::capture(),
            inner: info,
        }
    }
    pub fn msg(msg: impl Into<String>) -> Self {
        AlipanError::new(ErrorInfo::Msg(msg.into()))
    }
    pub fn server(code: StatusCode, content: &str) -> Self {
        let decoded: serde_json::Result<ServerError> = serde_json::from_str(content);
        if let Ok(server_error) = decoded {
            AlipanError::new(ErrorInfo::ServerError(server_error))
        } else {
            AlipanError::new(ErrorInfo::Msg(format!(
                "server error: code: {}, content: {}",
                code.as_u16(),
                content,
            )))
        }
    }
}

impl Display for AlipanError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl std::error::Error for AlipanError {}

#[derive(Debug)]
pub enum ErrorInfo {
    ReqwestError(reqwest::Error),
    SerdeJsonError(serde_json::Error),
    SerdeJsonErrorPath(serde_path_to_error::Error<serde_json::Error>),
    UrlParseError(url::ParseError),
    ServerError(ServerError),
    Io(std::io::Error),
    // todo: split into more specific error types
    Msg(String),
    Boxed(Box<dyn std::error::Error + Send + Sync>),
}

impl Display for ErrorInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ServerError {
    pub code: String,
    pub message: String,
    #[serde(rename = "requestId")]
    pub request_id: String,
}

impl From<reqwest::Error> for AlipanError {
    fn from(e: reqwest::Error) -> Self {
        AlipanError::new(ErrorInfo::ReqwestError(e))
    }
}

impl From<serde_json::Error> for AlipanError {
    fn from(e: serde_json::Error) -> Self {
        AlipanError::new(ErrorInfo::SerdeJsonError(e))
    }
}

impl From<serde_path_to_error::Error<serde_json::Error>> for AlipanError {
    fn from(e: serde_path_to_error::Error<serde_json::Error>) -> Self {
        AlipanError::new(ErrorInfo::SerdeJsonErrorPath(e))
    }
}

impl From<url::ParseError> for AlipanError {
    fn from(e: url::ParseError) -> Self {
        AlipanError::new(ErrorInfo::UrlParseError(e))
    }
}

impl From<ServerError> for AlipanError {
    fn from(e: ServerError) -> Self {
        AlipanError::new(ErrorInfo::ServerError(e))
    }
}

impl From<std::io::Error> for AlipanError {
    fn from(e: std::io::Error) -> Self {
        AlipanError::new(ErrorInfo::Io(e))
    }
}

impl From<BoxedError> for AlipanError {
    fn from(e: BoxedError) -> Self {
        AlipanError::new(ErrorInfo::Boxed(e))
    }
}
