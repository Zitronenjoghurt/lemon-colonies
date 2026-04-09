use crate::error::ServerResult;
use crate::integrations::IntegrationsConfig;

pub struct Config {
    pub database_url: String,
    pub dev_mode: bool,
    pub domain: String,
    pub integrations: IntegrationsConfig,
    pub session_secret: String,
}

impl Config {
    pub fn from_env() -> ServerResult<Self> {
        Ok(Self {
            database_url: std::env::var("DATABASE_URL")?,
            dev_mode: env_bool("DEV_MODE").unwrap_or(false),
            domain: std::env::var("DOMAIN")?,
            integrations: IntegrationsConfig::from_env()?,
            session_secret: std::env::var("SESSION_SECRET")?,
        })
    }
}

fn env_bool(name: &str) -> ServerResult<bool> {
    Ok(std::env::var(name).map(|s| s == "1" || s.eq_ignore_ascii_case("true"))?)
}
