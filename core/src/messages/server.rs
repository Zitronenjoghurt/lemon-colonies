use crate::error::CoreResult;
use crate::game::chunk::Chunk;
use crate::game::object::Object;
use crate::game::resource::ResourceBag;
use crate::math::coords::ChunkCoords;
use crate::messages::server::chunk_update::ChunkUpdate;
use crate::types::user_info::PrivateUserInfo;
use std::collections::HashSet;

pub mod chunk_update;

#[derive(Clone)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
pub enum ServerMessage {
    Pong { client_time: f64, server_time: f64 },
    Error(String),
    ColonyPositions(Vec<ChunkCoords>),
    Chunks(Vec<Chunk>),
    ChunkUpdate(ChunkUpdate),
    Objects(Vec<Object>),
    ResourceUpdate(ResourceBag),
    ResourceUpdateAll(ResourceBag),
    OwnedChunks(HashSet<ChunkCoords>),
    PlayerOwnedChunks(HashSet<ChunkCoords>),
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

    pub fn name(&self) -> &'static str {
        match self {
            Self::Pong { .. } => "pong",
            Self::Error(_) => "error",
            Self::ColonyPositions(_) => "colony_positions",
            Self::Chunks(_) => "chunks",
            Self::ChunkUpdate(_) => "chunk_update",
            Self::Objects(_) => "objects",
            Self::ResourceUpdate(_) => "resource_update",
            Self::ResourceUpdateAll(_) => "resource_update_all",
            Self::OwnedChunks(_) => "owned_chunks",
            Self::PlayerOwnedChunks(_) => "player_owned_chunks",
            Self::UserInfo(_) => "user_info",
        }
    }
}
