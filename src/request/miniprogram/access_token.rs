use crate::model::miniprogram::wechat_access_token::AccessToken;
use crate::model::result::Error;
use crate::request::Validator;
use serde::de::Visitor;
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub enum Platform {
    Wechat,
    Huawei,
    Unsupported,
}

impl Display for Platform {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Platform::Wechat => String::from("wechat"),
            Platform::Huawei => String::from("huawei"),
            _ => String::from("unsupported"),
        };
        write!(f, "{}", str)
    }
}

impl From<String> for Platform {
    fn from(v: String) -> Self {
        match v.to_lowercase().as_str() {
            "wechat" => Platform::Wechat,
            "huawei" => Platform::Huawei,
            _ => Platform::Unsupported,
        }
    }
}

impl From<Platform> for String {
    fn from(v: Platform) -> Self {
        v.to_string()
    }
}

struct PlatformVisitor;

impl Visitor<'_> for PlatformVisitor {
    type Value = Platform;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("反序列化失败，值应该为 string/str.")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(v.to_owned().into())
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(v.into())
    }
}

impl<'de> Deserialize<'de> for Platform {
    fn deserialize<D>(deserializer: D) -> Result<Platform, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(PlatformVisitor)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct LoginRequest {
    pub platform: Platform,
    pub code: Option<String>,
}

impl Validator for LoginRequest {
    type Data = String;

    fn validate(&self) -> crate::model::result::Result<Self::Data> {
        if self.platform == Platform::Unsupported {
            return Err(Error::ParamsMiniprogramLoginPlatformUnsupported(None));
        }

        if self.code.is_none() {
            return Err(Error::ParamsMiniprogramLoginCodeEmpty(None));
        }

        if self.code.to_owned().unwrap().chars().count() < 8 {
            return Err(Error::ParamsMiniprogramLoginCodeLengthShort(None));
        }

        Ok(self.code.to_owned().unwrap())
    }
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub access_token: String,
}

impl From<AccessToken> for LoginResponse {
    fn from(data: AccessToken) -> Self {
        Self {
            access_token: data.access_token,
        }
    }
}
