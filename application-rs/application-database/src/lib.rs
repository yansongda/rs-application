use application_kernel::config::{Database, G_CONFIG};
use application_kernel::result::{Error, Result};
use sqlx::MySqlPool;
use sqlx::mysql::{MySqlConnectOptions, MySqlPoolOptions};
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::LazyLock;
use std::time::Duration;
use tracing::error;

#[allow(unused_macros)]
macro_rules! query_optional {
    ($pool:expr, $sql:expr $(, $bind:expr)*) => {{
        let sql = $sql;
        let started_at = std::time::Instant::now();
        let result = sqlx::query_as(sql)
            $(.bind($bind))*
            .fetch_optional($pool)
            .await
            .map_err(|e| {
                tracing::error!("数据库查询失败: {:?}", e);

                application_kernel::result::Error::InternalDatabaseQuery(None)
            })?;

        let elapsed = started_at.elapsed().as_secs_f32();
        tracing::info!(elapsed, sql);

        result
    }};
}

#[allow(unused_macros)]
macro_rules! execute_write {
    ($pool:expr, $sql:expr, $error:expr $(, $bind:expr)*) => {{
        let sql = $sql;
        let started_at = std::time::Instant::now();
        let result = sqlx::query(sql)
            $(.bind($bind))*
            .execute($pool)
            .await
            .map_err(|e| {
                tracing::error!("数据库写入失败: {:?}", e);

                $error
            })?;

        let elapsed = started_at.elapsed().as_secs_f32();
        tracing::info!(elapsed, sql);

        result
    }};
}

#[allow(unused_imports)]
pub(crate) use execute_write;
#[allow(unused_imports)]
pub(crate) use query_optional;

pub mod account;
pub mod tool;

pub struct Pool;

static G_POOL_MYSQL: LazyLock<HashMap<&'static str, MySqlPool>> = LazyLock::new(|| {
    let databases = &G_CONFIG.databases;

    let mut mysql: HashMap<&'static str, MySqlPool> = HashMap::new();

    for database in databases {
        if database.1.url.starts_with("mysql://") {
            mysql.insert(database.0, Pool::connect_mysql(database.1));
        }
    }

    mysql
});

impl Pool {
    pub fn mysql(pool: &str) -> Result<&MySqlPool> {
        G_POOL_MYSQL.get(pool).ok_or_else(|| {
            error!("获取数据库连接失败: {}", pool);

            Error::InternalDatabaseAcquire(None)
        })
    }

    fn connect_mysql(config: &Database) -> MySqlPool {
        let connection_options =
            MySqlConnectOptions::from_str(config.url.as_str()).expect("数据库 URL 格式无效");

        MySqlPoolOptions::new()
            .acquire_timeout(Duration::from_secs(config.acquire_timeout))
            .min_connections(config.min_connections)
            .max_connections(config.max_connections)
            .idle_timeout(Duration::from_secs(config.idle_timeout))
            .connect_lazy_with(connection_options)
    }
}
