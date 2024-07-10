use crate::AlipanError;
use chrono::Local;
use serde::{Deserialize, Serialize};
use serde_derive::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

macro_rules! enum_str {
    ($name:ident { $($variant:ident($str:expr), )* }) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub enum $name {
            $($variant,)*
        }

        impl $name {
            pub fn as_str(&self) -> &'static str {
                match self {
                    $( $name::$variant => $str, )*
                }
            }
        }

        impl ::core::fmt::Display for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $( $name::$variant => write!(f,"{}",$str), )*
                }
            }
        }

        impl ::serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where S: ::serde::Serializer,
            {
                // 将枚举序列化为字符串。
                serializer.serialize_str(match *self {
                    $( $name::$variant => $str, )*
                })
            }
        }

        impl<'de> ::serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where D: ::serde::Deserializer<'de>,
            {
                struct Visitor;

                impl<'de> ::serde::de::Visitor<'de> for Visitor {
                    type Value = $name;

                    fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                        write!(formatter, "a string for {}", stringify!($name))
                    }

                    fn visit_str<E>(self, value: &str) -> Result<$name, E>
                        where E: ::serde::de::Error,
                    {
                        match value {
                            $( $str => Ok($name::$variant), )*
                            _ => Err(E::invalid_value(::serde::de::Unexpected::Other(
                                &format!("unknown {} variant: {}", stringify!($name), value)
                            ), &self)),
                        }
                    }
                }

                // 从字符串反序列化枚举。
                deserializer.deserialize_str(Visitor)
            }
        }

    }
}

enum_str!(GrantType {
    AuthorizationCode("authorization_code"),
    RefreshToken("refresh_token"),
});

enum_str! (AdriveOpenFileType {
    File("file"),
    Folder("folder"),
});

enum_str!(CheckNameMode {
    AutoRename("auto_rename"),
    Refuse("refuse"),
    Ignore("ignore"),
});

enum_str!(AdriveAsyncTaskState {
    Succeed("Succeed"),
    Running("Running"),
    Failed("Failed"),
});

#[derive(Debug)]
pub struct OptionParam<T>(pub Option<T>);

impl<T> Deref for OptionParam<T> {
    type Target = Option<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for OptionParam<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> OptionParam<T> {
    pub fn is_some(&self) -> bool {
        self.0.is_some()
    }

    pub fn is_none(&self) -> bool {
        self.0.is_none()
    }
}

impl<T> From<Option<T>> for OptionParam<T> {
    fn from(opt: Option<T>) -> Self {
        OptionParam(opt)
    }
}

impl<T> From<T> for OptionParam<T> {
    fn from(s: T) -> Self {
        OptionParam(Some(s))
    }
}

impl From<&str> for OptionParam<String> {
    fn from(s: &str) -> Self {
        OptionParam(Some(s.to_string()))
    }
}

impl<T> Serialize for OptionParam<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        match &self.0 {
            Some(v) => v.serialize(serializer),
            None => serializer.serialize_none(),
        }
    }
}

impl<'de, T> Deserialize<'de> for OptionParam<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<OptionParam<T>, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Ok(OptionParam(Some(T::deserialize(deserializer)?)))
    }
}

pub async fn response<T: for<'de> serde::Deserialize<'de>>(
    response: reqwest::Response,
) -> crate::Result<T> {
    let code = response.status();
    let text = response.text().await?;
    if !code.is_success() {
        return Err(AlipanError::server(code, text.as_str()));
    }
    let data: T = from_str(&text)?;
    Ok(data)
}

pub fn from_str<T: for<'de> serde::Deserialize<'de>>(json: &str) -> crate::Result<T> {
    Ok(serde_path_to_error::deserialize(
        &mut serde_json::Deserializer::from_str(json),
    )?)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdriveOpenFileCreatePost {
    pub drive_id: String,
    pub parent_file_id: String,
    pub name: String,
    pub r#type: AdriveOpenFileType,
    pub check_name_mode: CheckNameMode,
    pub part_info_list: Option<Vec<AdriveOpenFilePartInfo>>,
    pub streams_info: Option<Vec<AdriveOpenFileStreamInfo>>,
    pub pre_hash: Option<String>,
    pub size: Option<i64>,
    pub content_hash: Option<String>,
    pub content_hash_name: Option<String>,
    pub proof_code: Option<String>,
    pub proof_version: Option<String>,
    pub local_created_at: Option<chrono::DateTime<Local>>,
    pub local_modified_at: Option<chrono::DateTime<Local>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdriveOpenFilePartInfo {
    pub part_number: i64,
}

impl AdriveOpenFilePartInfo {
    pub fn part_number(mut self, part_number: i64) -> Self {
        self.part_number = part_number;
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdriveOpenFileStreamInfo {
    pub content_hash: String,
    pub content_hash_name: String,
    pub proof_version: String,
    pub proof_code: String,
    pub content_md5: String,
    pub pre_hash: String,
    pub size: i64,
    pub part_info_list: Vec<AdriveOpenFilePartInfo>,
}

impl AdriveOpenFileStreamInfo {
    pub fn content_hash(mut self, content_hash: impl Into<String>) -> Self {
        self.content_hash = content_hash.into();
        self
    }

    pub fn content_hash_name(mut self, content_hash_name: impl Into<String>) -> Self {
        self.content_hash_name = content_hash_name.into();
        self
    }

    pub fn proof_version(mut self, proof_version: impl Into<String>) -> Self {
        self.proof_version = proof_version.into();
        self
    }

    pub fn proof_code(mut self, proof_code: impl Into<String>) -> Self {
        self.proof_code = proof_code.into();
        self
    }

    pub fn content_md5(mut self, content_md5: impl Into<String>) -> Self {
        self.content_md5 = content_md5.into();
        self
    }

    pub fn pre_hash(mut self, pre_hash: impl Into<String>) -> Self {
        self.pre_hash = pre_hash.into();
        self
    }

    pub fn size(mut self, size: i64) -> Self {
        self.size = size;
        self
    }

    pub fn part_info_list(mut self, part_info_list: Vec<AdriveOpenFilePartInfo>) -> Self {
        self.part_info_list = part_info_list;
        self
    }
}
