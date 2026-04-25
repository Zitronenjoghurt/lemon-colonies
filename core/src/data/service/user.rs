use crate::data::entity::{colony, user};
use crate::data::store::Store;
use crate::data::Data;
use crate::error::CoreResult;
use crate::game::colony::placement::determine_new_colony_position;
use crate::game::config::GameConfig;
use crate::types::user_info::{PrivateUserInfo, PublicUserInfo};
use sea_orm::Set;
use std::sync::Arc;

pub struct UserService {
    pub config: Arc<GameConfig>,
    pub data: Arc<Data>,
}

impl UserService {
    pub fn new(data: &Arc<Data>, game_config: &Arc<GameConfig>) -> Self {
        Self {
            config: Arc::clone(game_config),
            data: Arc::clone(data),
        }
    }

    pub fn private_info(&self, user: &user::Model) -> PrivateUserInfo {
        PrivateUserInfo {
            public: PublicUserInfo {
                username: user.username.clone(),
            },
            permissions: user.permissions,
        }
    }

    async fn initiate_new_user(&self, user: user::Model) -> CoreResult<user::Model> {
        let colony_count = self.data.colony.count_all().await?;
        let pos = determine_new_colony_position(colony_count, self.config.world_seed);
        let chunk = self
            .data
            .chunk
            .load_or_generate(pos, self.config.world_seed)
            .await?;

        let new_colony = colony::ActiveModel {
            chunk_x: Set(chunk.pos.x),
            chunk_y: Set(chunk.pos.y),
            user_id: Set(user.id),
            ..Default::default()
        };
        self.data.colony.insert(new_colony).await?;

        Ok(user)
    }

    pub async fn register_from_discord(
        &self,
        discord_id: impl AsRef<str>,
        username: impl AsRef<str>,
    ) -> CoreResult<user::Model> {
        let new_user = self
            .data
            .user
            .create_from_discord(discord_id.as_ref(), username.as_ref())
            .await?;
        self.initiate_new_user(new_user).await
    }
}
