use crate::Pool;
use crate::account::Platform;
use crate::account::user::Config;

use application_kernel::result::Error;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::types::Json;
use std::time::Instant;
use tracing::{error, info};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ThirdConfig {
    pub id: u64,
    pub platform: Platform,
    pub third_id: String,
    pub config: Option<Json<Config>>,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

pub async fn fetch(
    platform: &Platform,
    third_id: &str,
) -> application_kernel::result::Result<ThirdConfig> {
    let sql = "select * from account.third_id where platform = ? and third_id = ? limit 1";
    let started_at = Instant::now();

    let result: Option<ThirdConfig> = sqlx::query_as(sql)
        .bind(platform)
        .bind(third_id)
        .fetch_optional(Pool::mysql("account")?)
        .await
        .map_err(|e| {
            error!("查询第三方平台配置信息失败: {:?}", e);

            Error::InternalDatabaseQuery(None)
        })?;

    let elapsed = started_at.elapsed().as_secs_f32();

    info!(elapsed, sql, third_id);

    if let Some(config) = result {
        return Ok(config);
    }

    Err(Error::ParamsThirdConfigNotFound(None))
}
