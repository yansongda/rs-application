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
use tracing::{error, info, instrument};

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

#[instrument]
pub async fn fetch(access_token: &str) -> Result<AccessToken> {
    let sql = "select * from account.access_token where access_token = $1 limit 1";

    let result: Option<AccessToken> = sqlx::query_as(sql)
        .bind(access_token)
        .fetch_optional(Pool::postgres("account")?)
        .await
        .map_err(|e| {
            error!("查询 access_token 失败: {:?}", e);

            Error::InternalDatabaseQuery(None)
        })?;

    if let Some(data) = result {
        return Ok(data);
    }

    Err(Error::ParamsAccessTokenNotFound(None))
}

pub async fn fetch_by_user_id(user_id: i64) -> Result<AccessToken> {
    let sql = "select * from account.access_token where user_id = $1 limit 1";
    let started_at = Instant::now();

    let result: Option<AccessToken> = sqlx::query_as(sql)
        .bind(user_id)
        .fetch_optional(Pool::postgres("account")?)
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
    platform: Platform,
    user_id: i64,
    data: &AccessTokenData,
) -> Result<AccessToken> {
    let sql = "insert into account.access_token (user_id, access_token, data, platform) values ($1, $2, $3, $4) returning *";
    let access_token = data.to_access_token(&platform)?;
    let started_at = Instant::now();

    let result = sqlx::query_as(sql)
        .bind(user_id)
        .bind(&access_token)
        .bind(Json(data))
        .bind(platform)
        .fetch_one(Pool::postgres("account")?)
        .await
        .map_err(|e| {
            error!("插入 access_token 失败: {:?}", e);

            Error::InternalDatabaseInsert(None)
        });

    let elapsed = started_at.elapsed().as_secs_f32();

    info!(elapsed, sql, user_id, access_token, ?data);

    result
}

pub async fn update(access_token: AccessToken, data: &AccessTokenData) -> Result<AccessToken> {
    let sql = "update account.access_token set access_token = $1, data = $2, updated_at = now() where id = $3 returning *";
    let started_at = Instant::now();

    let result = sqlx::query_as(sql)
        .bind(data.to_access_token(&access_token.platform)?)
        .bind(Json(data))
        .bind(access_token.id)
        .fetch_one(Pool::postgres("account")?)
        .await
        .map_err(|e| {
            error!("更新 access_token 失败: {:?}", e);

            Error::InternalDatabaseUpdate(None)
        });

    let elapsed = started_at.elapsed().as_secs_f32();

    info!(elapsed, sql, access_token.id, ?data);

    result
}
