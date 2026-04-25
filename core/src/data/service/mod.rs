use crate::data::Data;
use crate::game::config::GameConfig;
use std::sync::Arc;

mod chunk;
mod object;
mod user;

pub struct Services {
    pub chunk: chunk::ChunkService,
    pub object: object::ObjectService,
    pub user: user::UserService,
}

impl Services {
    pub fn new(data: &Arc<Data>, game_config: &Arc<GameConfig>) -> Self {
        Self {
            chunk: chunk::ChunkService::new(data),
            object: object::ObjectService::new(data),
            user: user::UserService::new(data, game_config),
        }
    }
}
