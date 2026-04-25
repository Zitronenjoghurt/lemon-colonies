use crate::data::entity::object;
use crate::data::store::Store;
use crate::data::Data;
use crate::error::{CoreError, CoreResult};
use crate::game::object::ObjectData;
use crate::math::coords::{ChunkCoords, LocalCoords, WorldCoords};
use crate::math::rect::Rect;
use futures::TryStreamExt;
use sea_orm::{ColumnTrait, ExprTrait};
use std::sync::Arc;

pub struct ObjectService {
    pub data: Arc<Data>,
}

impl ObjectService {
    pub fn new(data: &Arc<Data>) -> Self {
        Self { data: data.clone() }
    }

    pub async fn validate_placement_collision(&self, rect: Rect<f32>) -> CoreResult<()> {
        let min_chunk = WorldCoords::new(rect.min.x, rect.min.y).chunk();
        let max_chunk = WorldCoords::new(rect.max.x, rect.max.y).chunk();
        for chunk_y in min_chunk.y..=max_chunk.y {
            for chunk_x in min_chunk.x..=max_chunk.x {
                let chunk_coords = ChunkCoords::new(chunk_x, chunk_y);
                let mut object_stream = self
                    .data
                    .object
                    .stream_by(
                        object::Column::ChunkX
                            .eq(chunk_x)
                            .and(object::Column::ChunkY.eq(chunk_y)),
                    )
                    .await?;

                while let Some(object) = object_stream.try_next().await? {
                    let pos = chunk_coords
                        .with_local(LocalCoords::new(object.x as u8, object.y as u8))
                        .world();
                    let data: ObjectData = serde_json::from_value(object.data)?;
                    let object_collision = data.collision_rect(pos);
                    if rect.overlaps_rect(&object_collision) {
                        return Err(CoreError::ObjectCollision);
                    }
                }
            }
        }

        Ok(())
    }
}
