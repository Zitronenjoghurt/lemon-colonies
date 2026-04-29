use crate::game::resource::ResourceId;

pub type CoreResult<T> = Result<T, CoreError>;

#[derive(Debug, thiserror::Error)]
pub enum CoreError {
    #[cfg(feature = "bitcode")]
    #[error("Bitcode error: {0}")]
    Bitcode(#[from] bitcode::Error),
    #[cfg(feature = "data")]
    #[error("Database error: {0}")]
    Database(#[from] sea_orm::error::DbErr),
    #[error("Environment error: {0}")]
    Env(#[from] std::env::VarError),
    #[cfg(feature = "serde")]
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Parse int error: {0}")]
    ParseInt(#[from] std::num::ParseIntError),
    #[error("Invalid chunk terrain size")]
    InvalidChunkTerrainSize,
    #[error("Invalid object data")]
    InvalidObjectData,
    #[error("Invalid terrain")]
    InvalidTerrain,
    #[error("Chunk not owned")]
    ChunkNotOwned,
    #[error("Insufficient resources: {resource_id} ({available} available, {requested} requested)")]
    InsufficientResources {
        resource_id: ResourceId,
        available: u64,
        requested: u64,
    },
    #[error("Object collides with another object")]
    ObjectCollision,
}

impl CoreError {
    pub fn is_user_error(&self) -> bool {
        matches!(
            self,
            Self::ChunkNotOwned | Self::InsufficientResources { .. } | Self::ObjectCollision
        )
    }
}
