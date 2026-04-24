use crate::error::CoreResult;
use crate::game::chunk::Chunk;

pub mod chunk_update;

#[derive(Clone)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
pub enum ServerMessage {
    Hello,
    ColonyPositions(Vec<(i32, i32)>),
    Chunks(Vec<Chunk>),
    ChunkUpdate(chunk_update::ChunkUpdateMessage),
}

impl ServerMessage {
    #[cfg(feature = "bitcode")]
    pub fn as_bytes(&self) -> Vec<u8> {
        bitcode::encode(self)
    }

    #[cfg(feature = "bitcode")]
    pub fn from_bytes(bytes: &[u8]) -> CoreResult<Self> {
        Ok(bitcode::decode(bytes)?)
    }
}
