use crate::model::miniprogram::third_user::{Platform, ThirdUser};
use crate::model::result::{Error, Result};
use crate::repository::Pool;
use std::time::Instant;
use tracing::{error, info};

pub async fn fetch(platform: &Platform, third_id: &str) -> Result<ThirdUser> {
    let sql = "select * from miniprogram.third_user where platform = $1 and third_id = $2 limit 1";
    let started_at = Instant::now();

    let result: Option<ThirdUser> = sqlx::query_as(sql)
        .bind(platform)
        .bind(third_id)
        .fetch_optional(Pool::postgres("miniprogram")?)
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

    Err(Error::ParamsMiniprogramThirdUserNotFound(None))
}

pub async fn insert(platform: Platform, third_id: &str) -> Result<ThirdUser> {
    let sql = "insert into miniprogram.third_user (platform, third_id) values ($1, $2) returning *";
    let started_at = Instant::now();

    let result = sqlx::query_as(sql)
        .bind(platform)
        .bind(third_id)
        .fetch_one(Pool::postgres("miniprogram")?)
        .await
        .map_err(|e| {
            error!("查询第三方平台用户失败: {:?}", e);

            Error::InternalDatabaseInsert(None)
        });

    let elapsed = started_at.elapsed().as_secs_f32();

    info!(elapsed, sql, third_id);

    result
}
