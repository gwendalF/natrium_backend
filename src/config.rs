use config::ConfigError;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u32,
}
#[derive(Deserialize, Clone)]
pub struct Key {
    pub key: String,
    pub refresh_key: String,
}

#[derive(Deserialize, Clone)]
pub struct Config {
    pub server: ServerConfig,
    pub workers: usize,
    pub secret: Key,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let mut cfg = config::Config::new();
        cfg.merge(config::Environment::new())?;
        cfg.try_into()
    }
}
