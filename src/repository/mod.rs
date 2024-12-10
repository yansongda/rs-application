use std::collections::HashMap;
use std::str::FromStr;
use std::sync::OnceLock;
use std::time::Duration;

use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::PgPool;

use crate::config::{Database, G_CONFIG};

pub mod miniprogram;

pub struct Pool;

static G_POOL_PG: OnceLock<HashMap<&'static str, PgPool>> = OnceLock::new();

impl Pool {
    pub async fn init() {
        Self::init_databases().await;
    }

    pub fn postgres(pool: &str) -> &PgPool {
        G_POOL_PG.get().unwrap().get(pool).unwrap()
    }

    async fn init_databases() {
        let databases = &G_CONFIG.databases;

        let mut pg: HashMap<&'static str, PgPool> = HashMap::new();

        for database in databases {
            if database.1.url.starts_with("postgres://") && G_POOL_PG.get().is_none() {
                pg.insert(database.0, Self::connect_postgres(database.1).await);
            }
        }

        G_POOL_PG.set(pg).unwrap();
    }

    async fn connect_postgres(config: &Database) -> PgPool {
        let connection_options = PgConnectOptions::from_str(config.url.as_str())
            .unwrap()
            .application_name(G_CONFIG.name.as_str());

        PgPoolOptions::new()
            .acquire_timeout(Duration::from_secs(config.acquire_timeout))
            .min_connections(config.min_connections)
            .max_connections(config.max_connections)
            .connect_with(connection_options)
            .await
            .unwrap()
    }
}
