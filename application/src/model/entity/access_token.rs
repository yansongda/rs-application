use crate::model::entity::third_user::Platform;
use crate::model::result::{Error, Result};
use crate::model::wechat::LoginResponse;
use chrono::{DateTime, Local};
use fasthash::murmur3;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::types::Json;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AccessToken {
    pub id: i64,
    pub user_id: i64,
    pub platform: Platform,
    pub access_token: String,
    pub data: Json<AccessTokenData>,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessTokenData {
    pub wechat: Option<WechatAccessTokenData>,
}

impl AccessTokenData {
    pub fn to_access_token(&self, platform: &Platform) -> Result<String> {
        match platform {
            Platform::Wechat => {
                if let Some(data) = &self.wechat {
                    Ok(base62::encode(murmur3::hash128(
                        format!("{}:{}", data.open_id, data.session_key).as_bytes(),
                    )))
                } else {
                    Err(Error::InternalDataToAccessTokenError(None))
                }
            }
            _ => Err(Error::ParamsLoginPlatformUnsupported(None)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatAccessTokenData {
    pub open_id: String,
    pub session_key: String,
    pub union_id: Option<String>,
}

impl From<LoginResponse> for WechatAccessTokenData {
    fn from(response: LoginResponse) -> Self {
        WechatAccessTokenData {
            open_id: response.openid.unwrap_or_default(),
            session_key: response.session_key.unwrap_or_default(),
            union_id: response.unionid,
        }
    }
}

impl From<LoginResponse> for AccessTokenData {
    fn from(response: LoginResponse) -> Self {
        AccessTokenData {
            wechat: Some(WechatAccessTokenData::from(response)),
        }
    }
}
