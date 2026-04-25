use crate::error::{CoreError, CoreResult};
use crate::game::object::ObjectData;
use crate::math::coords::{ChunkCoords, ChunkLocal, WorldCoords};
use crate::math::rect::Rect;

#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
pub struct ObjectPlacement {
    pub data: ObjectData,
    pub pos: ChunkLocal,
}

impl ObjectPlacement {
    pub fn world_pos(&self) -> WorldCoords {
        self.pos.world()
    }

    pub fn collision_rect(&self) -> Rect<f32> {
        self.data.collision_rect(self.pos.world())
    }

    pub fn affected_chunks_min_max(&self) -> (ChunkCoords, ChunkCoords) {
        let rect = self.collision_rect();
        (
            WorldCoords::new(rect.min.x, rect.min.y).chunk(),
            WorldCoords::new(rect.max.x, rect.max.y).chunk(),
        )
    }
}

#[cfg(all(feature = "data", feature = "serde"))]
impl TryFrom<ObjectPlacement> for crate::data::entity::object::ActiveModel {
    type Error = CoreError;

    fn try_from(p: ObjectPlacement) -> CoreResult<Self> {
        let kind = p.data.kind();
        let data = serde_json::to_value(p.data)?;
        Ok(crate::data::entity::object::ActiveModel {
            kind: sea_orm::Set(kind as u16 as i16),
            data: sea_orm::Set(data),
            chunk_x: sea_orm::Set(p.pos.chunk.x),
            chunk_y: sea_orm::Set(p.pos.chunk.y),
            x: sea_orm::Set(p.pos.local.x as i16),
            y: sea_orm::Set(p.pos.local.y as i16),
            ..Default::default()
        })
    }
}
