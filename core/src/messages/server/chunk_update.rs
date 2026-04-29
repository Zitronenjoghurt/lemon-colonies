use crate::game::object::Object;
use crate::math::coords::ChunkCoords;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
pub struct ChunkUpdate {
    pub coords: ChunkCoords,
    pub kind: ChunkUpdateKind,
}

impl ChunkUpdate {
    pub fn update_object(coords: ChunkCoords, object: Object) -> Self {
        Self {
            coords,
            kind: ChunkUpdateKind::UpdateObject(object),
        }
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
pub enum ChunkUpdateKind {
    UpdateObject(Object),
}
