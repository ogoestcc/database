#[derive(Debug, serde::Deserialize)]
pub struct ServerConfig {
    pub port: u32,
}
#[derive(Debug, serde::Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub postgres: deadpool_postgres::Config,
}

impl Config {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        let mut cfg = config::Config::new();
        cfg.merge(config::Environment::new())?;
        cfg.try_into()
    }
}
