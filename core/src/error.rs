pub type CoreResult<T> = Result<T, CoreError>;

#[derive(Debug, thiserror::Error)]
pub enum CoreError {
    #[cfg(feature = "bitcode")]
    #[error("Bitcode error: {0}")]
    Bitcode(#[from] bitcode::Error),
    #[cfg(feature = "data")]
    #[error("Database error: {0}")]
    Database(#[from] sea_orm::error::DbErr),
    #[cfg(feature = "serde")]
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}
