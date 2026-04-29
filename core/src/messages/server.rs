use crate::error::CoreResult;
use crate::game::chunk::Chunk;
use crate::game::resource::ResourceBag;
use crate::math::coords::ChunkCoords;
use crate::messages::server::chunk_update::ChunkUpdate;
use crate::types::user_info::PrivateUserInfo;

pub mod chunk_update;

#[derive(Clone)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
pub enum ServerMessage {
    Pong { client_time: f64, server_time: f64 },
    Error(String),
    ColonyPositions(Vec<ChunkCoords>),
    Chunks(Vec<Chunk>),
    ChunkUpdate(ChunkUpdate),
    ResourceUpdate(ResourceBag),
    ResourceUpdateAll(ResourceBag),
    OwnedChunks(Vec<ChunkCoords>),
    UserInfo(PrivateUserInfo),
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
