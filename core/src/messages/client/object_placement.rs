use crate::error::{CoreError, CoreResult};
use crate::game::object::ObjectData;

#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
pub struct ObjectPlacement {
    pub data: ObjectData,
    pub chunk: (i32, i32),
    pub position: (u8, u8),
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
            chunk_x: sea_orm::Set(p.chunk.0),
            chunk_y: sea_orm::Set(p.chunk.1),
            x: sea_orm::Set(p.position.0 as i16),
            y: sea_orm::Set(p.position.1 as i16),
            ..Default::default()
        })
    }
}
