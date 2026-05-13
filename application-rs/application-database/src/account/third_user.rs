use crate::{Pool, insert, query_optional};
use crate::account::Platform;
use crate::account::user::Config;
use application_kernel::result::Error;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::types::Json;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ThirdUser {
    pub id: u64,
    pub user_id: u64,
    pub platform: Platform,
    pub third_id: String,
    pub config: Option<Json<Config>>,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

pub async fn fetch(
    platform: &Platform,
    third_id: &str,
) -> application_kernel::result::Result<ThirdUser> {
    let sql = "select * from account.third_user where platform = ? and third_id = ? limit 1";
    let pool = Pool::mysql("account")?;

    let result: Option<ThirdUser> = query_optional!(pool, sql, platform, third_id);

    if let Some(third_user) = result {
        return Ok(third_user);
    }

    Err(Error::ParamsThirdUserNotFound(None))
}

pub async fn insert(
    platform: &Platform,
    third_id: &str,
    user_id: u64,
) -> application_kernel::result::Result<u64> {
    let sql = "insert into account.third_user (platform, third_id, user_id) values (?, ?, ?)";
    let pool = Pool::mysql("account")?;

    let result = insert!(pool, sql, platform, third_id, user_id);

    Ok(result.last_insert_id())
}
