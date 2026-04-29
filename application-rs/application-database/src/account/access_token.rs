use crate::Pool;
use crate::account::Platform;
use application_kernel::config::G_CONFIG;
use application_kernel::result::{Error, Result};
use application_util::wechat::LoginResponse;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::types::Json;
use std::time::Instant;
use tracing::{error, info};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AccessToken {
    pub id: u64,
    pub user_id: u64,
    pub platform: Platform,
    pub third_id: String,
    pub access_token: String,
    pub data: Json<AccessTokenData>,
    pub expired_at: Option<DateTime<Local>>,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

impl AccessToken {
    pub fn is_expired(&self) -> bool {
        if let Some(expired_at) = self.expired_at {
            return Local::now() > expired_at;
        }

        false
    }

    pub fn get_expired_in(&self) -> u32 {
        // todo： 微信的 access_token 永不过期，后续需要处理
        if let Some(expired_at) = self.expired_at {
            let duration = expired_at.signed_duration_since(Local::now());

            return duration.num_seconds() as u32;
        }

        G_CONFIG.access_token.expired_in
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessTokenData {
    pub wechat: Option<WechatAccessTokenData>,
    pub huawei: Option<HuaweiAccessTokenData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatAccessTokenData {
    pub open_id: String,
    pub union_id: String,
}

impl From<LoginResponse> for WechatAccessTokenData {
    fn from(response: LoginResponse) -> Self {
        WechatAccessTokenData {
            open_id: response.openid,
            union_id: response.unionid,
        }
    }
}

impl From<LoginResponse> for AccessTokenData {
    fn from(response: LoginResponse) -> Self {
        AccessTokenData {
            wechat: Some(WechatAccessTokenData::from(response)),
            huawei: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HuaweiAccessTokenData {
    pub token_type: String,
    pub scope: String,
    pub refresh_token: String,
    pub client_id: String,
    pub union_id: String,
    pub project_id: String,
    #[serde(rename = "type")]
    pub r#type: i64,
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

pub async fn fetch_by_id(id: u64) -> Result<AccessToken> {
    let sql = "select * from account.access_token where id = ? limit 1";
    let started_at = Instant::now();

    let result: Option<AccessToken> = sqlx::query_as(sql)
        .bind(id)
        .fetch_optional(Pool::mysql("account")?)
        .await
        .map_err(|e| {
            error!("查询 access_token 失败: {:?}", e);

            Error::InternalDatabaseQuery(None)
        })?;

    let elapsed = started_at.elapsed().as_secs_f32();

    info!(elapsed, sql, id);

    if let Some(data) = result {
        return Ok(data);
    }

    Err(Error::ParamsAccessTokenNotFound(None))
}

pub async fn fetch_by_user_id(platform: &Platform, user_id: u64) -> Result<AccessToken> {
    let sql = "select * from account.access_token where user_id = ? and platform = ? limit 1";
    let started_at = Instant::now();

    let result: Option<AccessToken> = sqlx::query_as(sql)
        .bind(user_id)
        .bind(platform)
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

pub async fn update_or_insert(
    platform: &Platform,
    third_id: &str,
    user_id: u64,
    data: AccessTokenData,
) -> Result<AccessToken> {
    match fetch_by_user_id(platform, user_id).await {
        Ok(access_token) => update(access_token, data).await,
        Err(_) => insert(platform, third_id, user_id, data).await,
    }
}

pub async fn insert(
    platform: &Platform,
    third_id: &str,
    user_id: u64,
    data: AccessTokenData,
) -> Result<AccessToken> {
    let sql = "insert into account.access_token (user_id, access_token, data, platform, third_id, expired_at) values (?, ?, ?, ?, ?, ?)";
    let access_token = Uuid::now_v7().to_string();
    let started_at = Instant::now();
    let mut expired_at = Some(G_CONFIG.access_token.get_expired_at());

    // todo: 微信的 access_token 永不过期，后续需要处理
    if Platform::Wechat == *platform {
        expired_at = None;
    }

    let result = sqlx::query(sql)
        .bind(user_id)
        .bind(&access_token)
        .bind(Json(&data))
        .bind(platform)
        .bind(third_id)
        .bind(expired_at)
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
        third_id: third_id.to_string(),
        access_token,
        data: Json(data),
        expired_at,
        created_at: Local::now(),
        updated_at: Local::now(),
    })
}

pub async fn update(mut access_token: AccessToken, data: AccessTokenData) -> Result<AccessToken> {
    let sql =
        "update account.access_token set access_token = ?, data = ?, expired_at = ? where id = ?";
    let started_at = Instant::now();
    let access_token_value = Uuid::now_v7().to_string();
    let mut expired_at = Some(G_CONFIG.access_token.get_expired_at());

    // todo: 微信的 access_token 永不过期，后续需要处理
    if Platform::Wechat == access_token.platform {
        expired_at = None;
    }

    let _ = sqlx::query(sql)
        .bind(&access_token_value)
        .bind(Json(&data))
        .bind(expired_at)
        .bind(access_token.id)
        .execute(Pool::mysql("account")?)
        .await
        .map_err(|e| {
            error!("更新 access_token 失败: {:?}", e);

            Error::InternalDatabaseUpdate(None)
        });

    let elapsed = started_at.elapsed().as_secs_f32();

    info!(elapsed, sql, access_token.id, ?data);

    access_token.access_token = access_token_value;
    access_token.data = Json(data);
    access_token.expired_at = expired_at;
    access_token.updated_at = Local::now();

    Ok(access_token)
}
