use config::{Config as C, Environment, File};
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::LazyLock;

pub static G_CONFIG: LazyLock<Config> = LazyLock::new(|| {
    let config = C::builder()
        .add_source(File::with_name("./config.toml").required(false))
        .add_source(
            Environment::with_prefix("APP")
                .try_parsing(true)
                .separator("__"),
        )
        .build()
        .expect("加载配置失败");

    config.try_deserialize::<Config>().expect("解析配置失败")
});

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct Config {
    pub name: String,
    pub bin: HashMap<String, Bin>,
    pub databases: HashMap<String, Database>,
    pub short_url: ShortUrl,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            name: "rs-application".to_string(),
            bin: HashMap::new(),
            databases: HashMap::new(),
            short_url: ShortUrl::default(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct Bin {
    pub listen: String,
    pub port: u16,
    pub debug: bool,
}

impl Default for Bin {
    fn default() -> Self {
        Self {
            listen: "0.0.0.0".to_string(),
            port: 8080,
            debug: false,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct Database {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub acquire_timeout: u64,
    pub idle_timeout: u64,
}

impl Default for Database {
    fn default() -> Self {
        Self {
            url: "mysql://root:root@127.0.0.1:3306/test".to_string(),
            max_connections: 20,
            min_connections: 2,
            acquire_timeout: 3,
            idle_timeout: 300,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct ShortUrl {
    pub domain: String,
}

impl Default for ShortUrl {
    fn default() -> Self {
        Self {
            domain: "https://u.ysdor.cn".to_string(),
        }
    }
}
