use crate::Pool;
use crate::account::user::Config;
use application_kernel::result::Error;
use chrono::{DateTime, Local};
use serde::de::Visitor;
use serde::{Deserialize, Deserializer, Serialize};
use sqlx::encode::IsNull;
use sqlx::error::BoxDynError;
use sqlx::postgres::{PgArgumentBuffer, PgTypeInfo, PgValueRef};
use sqlx::types::Json;
use sqlx::{Encode, FromRow, Postgres, Type};
use std::fmt::{Display, Formatter};
use std::time::Instant;
use tracing::{error, info};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ThirdUser {
    pub id: i64,
    pub user_id: i64,
    pub platform: Platform,
    pub third_id: String,
    pub config: Option<Json<Config>>,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub enum Platform {
    Wechat,
    Huawei,
    Unsupported,
}

impl Type<Postgres> for Platform {
    fn type_info() -> <Postgres as sqlx::Database>::TypeInfo {
        <&str as Type<Postgres>>::type_info()
    }

    fn compatible(_: &PgTypeInfo) -> bool {
        true
    }
}

impl sqlx::Encode<'_, Postgres> for Platform {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> Result<IsNull, BoxDynError> {
        <&str as Encode<Postgres>>::encode(self.into(), buf)
    }
}

impl sqlx::Decode<'_, Postgres> for Platform {
    fn decode(value: PgValueRef<'_>) -> Result<Self, BoxDynError> {
        Ok(value.as_str()?.into())
    }
}

impl Display for Platform {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", <&Platform as Into<&str>>::into(self))
    }
}

impl From<&Platform> for &str {
    fn from(v: &Platform) -> Self {
        match v {
            Platform::Wechat => "wechat",
            Platform::Huawei => "huawei",
            _ => "unsupported",
        }
    }
}

impl From<&str> for Platform {
    fn from(v: &str) -> Self {
        match v {
            "wechat" => Platform::Wechat,
            "huawei" => Platform::Huawei,
            _ => Platform::Unsupported,
        }
    }
}

impl From<String> for Platform {
    fn from(v: String) -> Self {
        Self::from(v.to_lowercase().as_str())
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

pub async fn fetch(
    platform: &Platform,
    third_id: &str,
) -> application_kernel::result::Result<ThirdUser> {
    let sql = "select * from account.third_user where platform = $1 and third_id = $2 limit 1";
    let started_at = Instant::now();

    let result: Option<ThirdUser> = sqlx::query_as(sql)
        .bind(platform)
        .bind(third_id)
        .fetch_optional(Pool::postgres("account")?)
        .await
        .map_err(|e| {
            error!("查询第三方平台用户失败: {:?}", e);

            Error::InternalDatabaseQuery(None)
        })?;

    let elapsed = started_at.elapsed().as_secs_f32();

    info!(elapsed, sql, third_id);

    if let Some(third_user) = result {
        return Ok(third_user);
    }

    Err(Error::ParamsThirdUserNotFound(None))
}

pub async fn insert(
    platform: Platform,
    third_id: &str,
    user_id: i64,
) -> application_kernel::result::Result<ThirdUser> {
    let sql = "insert into account.third_user (platform, third_id, user_id) values ($1, $2, $3) returning *";
    let started_at = Instant::now();

    let result = sqlx::query_as(sql)
        .bind(platform)
        .bind(third_id)
        .bind(user_id)
        .fetch_one(Pool::postgres("account")?)
        .await
        .map_err(|e| {
            error!("查询第三方平台用户失败: {:?}", e);

            Error::InternalDatabaseInsert(None)
        });

    let elapsed = started_at.elapsed().as_secs_f32();

    info!(elapsed, sql, third_id);

    result
}
