use config::{Config as RawConfig, ConfigError, Environment};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub jwt: JwtConfig,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    #[serde(default = "default_port")]
    pub port: u16,
}

fn default_host() -> String {
    "0.0.0.0".to_string()
}
fn default_port() -> u16 {
    8080
}

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct JwtConfig {
    pub secret: String,
    #[serde(default = "default_jwt_expiration")]
    pub expiration: i64,
}


fn default_jwt_expiration() -> i64 {
    86400 // 24 hours in seconds
}

pub fn load_config() -> Result<Config, ConfigError> {
    // Load environment variables from .env file
    dotenv::dotenv().ok();
    let config = RawConfig::builder().add_source(Environment::default().separator("_")).build()?;
    // Parse environment variables into config
    config.try_deserialize()
}