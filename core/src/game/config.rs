use crate::error::CoreResult;

pub struct GameConfig {
    pub world_seed: u64,
}

impl GameConfig {
    pub fn from_env() -> CoreResult<Self> {
        Ok(Self {
            world_seed: std::env::var("WORLD_SEED")?.parse::<u64>()?,
        })
    }
}
