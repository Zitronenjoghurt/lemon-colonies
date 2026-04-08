use crate::error::ServerResult;

pub struct Config {
    pub database_url: String,
}

impl Config {
    pub fn from_env() -> ServerResult<Self> {
        Ok(Self {
            database_url: std::env::var("DATABASE_URL")?,
        })
    }
}
