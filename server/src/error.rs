use oauth2::url;

pub type ServerResult<T> = Result<T, ServerError>;

#[derive(Debug, thiserror::Error)]
pub enum ServerError {
    #[error("Core error: {0}")]
    Core(#[from] lemon_colonies_core::error::CoreError),
    #[error("Error reading environment variable: {0}")]
    Env(#[from] std::env::VarError),
    #[error("Json error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Parse int error: {0}")]
    ParseInt(#[from] std::num::ParseIntError),
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] oauth2::reqwest::Error),
    #[error("OAuth2 token request error: {0}")]
    TokenRequest(String),
    #[error("Error parsing URL: {0}")]
    Url(#[from] url::ParseError),
}
