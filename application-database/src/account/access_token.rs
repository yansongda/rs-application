use crate::Pool;
use crate::account::third_user::Platform;
use application_kernel::result::{Error, Result};
use application_util::wechat::LoginResponse;
use chrono::{DateTime, Local};
use fasthash::murmur3;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::types::Json;
use std::time::Instant;
use tracing::{error, info};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AccessToken {
    pub id: u64,
    pub user_id: u64,
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

pub async fn fetch(access_token: &str) -> Result<AccessToken> {
    let sql = "select * from account.access_token where access_token = ? limit 1";
    let started_at = Instant::now();

    let result: Option<AccessToken> = sqlx::query_as(sql)
        .bind(access_token)
        .fetch_optional(Pool::mysql("account")?)
        .await
        .map_err(|e| {
            error!("查询 access_token 失败: {:?}", e);

            Error::InternalDatabaseQuery(None)
        })?;

    let elapsed = started_at.elapsed().as_secs_f32();

    info!(elapsed, sql, access_token);

    if let Some(data) = result {
        return Ok(data);
    }

    Err(Error::ParamsAccessTokenNotFound(None))
}

pub async fn fetch_by_user_id(user_id: u64) -> Result<AccessToken> {
    let sql = "select * from account.access_token where user_id = ? limit 1";
    let started_at = Instant::now();

    let result: Option<AccessToken> = sqlx::query_as(sql)
        .bind(user_id)
        .fetch_optional(Pool::mysql("account")?)
        .await
        .map_err(|e| {
            error!("通过 user_id 查询 access_token 失败: {:?}", e);

            Error::InternalDatabaseQuery(None)
        })?;

    let elapsed = started_at.elapsed().as_secs_f32();

    info!(elapsed, sql, user_id);

    if let Some(data) = result {
        return Ok(data);
    }

    Err(Error::ParamsAccessTokenNotFound(None))
}

pub async fn insert(
    platform: &Platform,
    user_id: u64,
    data: AccessTokenData,
) -> Result<AccessToken> {
    let sql = "insert into account.access_token (user_id, access_token, data, platform) values (?, ?, ?, ?)";
    let access_token = data.to_access_token(platform)?;
    let started_at = Instant::now();

    let result = sqlx::query(sql)
        .bind(user_id)
        .bind(&access_token)
        .bind(Json(&data))
        .bind(platform)
        .execute(Pool::mysql("account")?)
        .await
        .map_err(|e| {
            error!("插入 access_token 失败: {:?}", e);

            Error::InternalDatabaseInsert(None)
        });

    let elapsed = started_at.elapsed().as_secs_f32();

    info!(elapsed, sql, user_id, access_token, ?data);

    Ok(AccessToken {
        id: result?.last_insert_id(),
        user_id,
        platform: platform.to_owned(),
        access_token: access_token.clone(),
        data: Json(data),
        created_at: Local::now(),
        updated_at: Local::now(),
    })
}

pub async fn update(mut access_token: AccessToken, data: AccessTokenData) -> Result<AccessToken> {
    let sql = "update account.access_token set access_token = ?, data = ?, updated_at = now() where id = ?";
    let started_at = Instant::now();
    let access_token_string = data.to_access_token(&access_token.platform)?;

    let _ = sqlx::query(sql)
        .bind(access_token_string.clone())
        .bind(Json(&data))
        .bind(access_token.id)
        .execute(Pool::mysql("account")?)
        .await
        .map_err(|e| {
            error!("更新 access_token 失败: {:?}", e);

            Error::InternalDatabaseUpdate(None)
        });

    let elapsed = started_at.elapsed().as_secs_f32();

    info!(elapsed, sql, access_token.id, ?data);

    access_token.access_token = access_token_string;
    access_token.data = Json(data);
    access_token.updated_at = Local::now();

    Ok(access_token)
}
