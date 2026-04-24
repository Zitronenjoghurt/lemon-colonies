use crate::game::object::Object;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
pub struct ChunkUpdateMessage {
    pub coords: (i32, i32),
    pub kind: ChunkUpdateKind,
}

impl ChunkUpdateMessage {
    pub fn update_object(coords: (i32, i32), object: Object) -> Self {
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
