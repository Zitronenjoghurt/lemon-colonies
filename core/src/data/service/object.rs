use crate::data::entity::object;
use crate::data::store::Store;
use crate::data::Data;
use crate::error::{CoreError, CoreResult};
use crate::game::object::command::{ObjectCommand, ObjectCommandResult};
use crate::game::object::data::ObjectData;
use crate::game::object::Object;
use crate::math::coords::{ChunkCoords, LocalCoords};
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
        let (min_chunk, max_chunk) = rect.chunk_range();
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

    pub async fn handle_command(
        &self,
        server_time: f64,
        command: ObjectCommand,
    ) -> CoreResult<Option<(ObjectCommandResult, object::Model)>> {
        let Some(model) = self.data.object.find_by_id(command.target).await? else {
            return Ok(None);
        };

        let mut object = Object::try_from(model.clone())?;
        object.tick(server_time);
        let result = object.apply_command(command);

        let model = if result.dirty {
            let active = object::ActiveModel::try_from(&object)?;
            self.data.object.update(active).await?
        } else {
            model
        };

        Ok(Some((result, model)))
    }
}
