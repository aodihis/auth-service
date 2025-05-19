use config::{Config as RawConfig, ConfigError, Environment, File};
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    #[serde(default = "default_env")]
    pub env: String,
    pub verification_url: String,
}

fn default_env() -> String {
    "dev".to_string()
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub jwt: JwtConfig,
    pub smtp: SmtpConfig,
    pub app: AppConfig,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    #[serde(default = "default_host")]
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

#[derive(Debug, Deserialize)]
pub struct AuthConfig {}

#[derive(Debug, Deserialize)]
pub struct SmtpConfig {
    pub from_name: String,
    pub from_email: String,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub tls: bool,
}

fn default_jwt_expiration() -> i64 {
    86400 // 24 hours in seconds
}

pub fn load_config() -> Result<Config, ConfigError> {
    // Load environment variables from .env file
    let mut run_env = env::var("ENV").unwrap_or_else(|_| "dev".into());
    if cfg!(test) {
        dotenv::from_filename(".env.test").ok();
        run_env = "test".into();
    } else {
        dotenv::from_filename(".env").ok();
    }



    let config = RawConfig::builder()
        .add_source(File::with_name(&format!("config/{}", run_env)).required(false))
        .add_source(Environment::default().separator("_"))
        .build()?;
    // Parse environment variables into config
    config.try_deserialize()
}
