use crate::data::entity::colony;
use crate::data::store::Store;
use crate::data::Data;
use crate::error::CoreResult;
use crate::math::coords::ChunkCoords;
use crate::types::chunk_visibility::ChunkVisibility;
use futures::TryStreamExt;
use sea_orm::prelude::Uuid;
use sea_orm::ColumnTrait;
use std::sync::Arc;

pub struct ChunkService {
    pub data: Arc<Data>,
}

impl ChunkService {
    pub fn new(data: &Arc<Data>) -> Self {
        Self { data: data.clone() }
    }
}

impl ChunkService {
    // ToDo: Consider caching if it becomes expensive
    pub async fn visibility_for_user(&self, user_id: Uuid) -> CoreResult<ChunkVisibility> {
        let mut colony_stream = self
            .data
            .colony
            .stream_by(colony::Column::UserId.eq(user_id))
            .await?;

        let mut chunk_visibility = ChunkVisibility::default();
        while let Some(colony) = colony_stream.try_next().await? {
            // ToDo: Make radius depend on intel or something
            chunk_visibility.insert(colony.chunk_x, colony.chunk_y, 128);
        }

        Ok(chunk_visibility)
    }

    // ToDo: Account for claimed chunks besides just the colony chunk
    pub async fn does_user_own_chunk(&self, user_id: Uuid, pos: ChunkCoords) -> CoreResult<bool> {
        let Some(colony) = self.data.colony.find_by_id((pos.x, pos.y)).await? else {
            return Ok(false);
        };
        Ok(colony.user_id == user_id)
    }
}
