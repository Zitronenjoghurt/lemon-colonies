use crate::data::entity::{colony, colony_chunk};
use crate::data::store::Store;
use crate::data::Data;
use crate::error::{CoreError, CoreResult};
use crate::math::coords::ChunkCoords;
use crate::math::rect::Rect;
use crate::types::chunk_visibility::ChunkVisibility;
use futures::TryStreamExt;
use sea_orm::prelude::Uuid;
use sea_orm::{ColumnTrait, ExprTrait};
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
            chunk_visibility.insert(colony.origin_chunk_x, colony.origin_chunk_y, 128);
        }

        Ok(chunk_visibility)
    }

    pub async fn validate_chunk_owned(&self, user_id: Uuid, pos: ChunkCoords) -> CoreResult<()> {
        let Some((_, owner)) = self
            .data
            .colony_chunk
            .find_one_by_with_owner(
                colony_chunk::Column::ChunkX
                    .eq(pos.x)
                    .and(colony_chunk::Column::ChunkY.eq(pos.y)),
            )
            .await?
        else {
            return Err(CoreError::ChunkNotOwned);
        };

        if owner.id != user_id {
            return Err(CoreError::ChunkNotOwned);
        }

        Ok(())
    }

    pub async fn validate_chunks_owned(&self, user_id: Uuid, rect: Rect<f32>) -> CoreResult<()> {
        let (min_chunk, max_chunk) = rect.chunk_range();
        for chunk_y in min_chunk.y..=max_chunk.y {
            for chunk_x in min_chunk.x..=max_chunk.x {
                self.validate_chunk_owned(user_id, ChunkCoords::new(chunk_x, chunk_y))
                    .await?;
            }
        }

        Ok(())
    }
}
