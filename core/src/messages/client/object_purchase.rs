use crate::error::{CoreError, CoreResult};
use crate::game::object::purchase::PurchasableObject;
use crate::math::coords::{ChunkLocal, WorldCoords};
use crate::math::rect::Rect;

#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
pub struct ObjectPurchase {
    pub kind: PurchasableObject,
    pub pos: ChunkLocal,
}

impl ObjectPurchase {
    pub fn world_pos(&self) -> WorldCoords {
        self.pos.world()
    }

    pub fn collision_rect(&self) -> Rect<f32> {
        self.kind.object_data().collision_rect(self.pos.world())
    }
}

#[cfg(all(feature = "data", feature = "serde"))]
impl TryFrom<ObjectPurchase> for crate::data::entity::object::ActiveModel {
    type Error = CoreError;

    fn try_from(p: ObjectPurchase) -> CoreResult<Self> {
        let data = p.kind.object_data();
        let kind = data.kind();
        let data = serde_json::to_value(data)?;
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
