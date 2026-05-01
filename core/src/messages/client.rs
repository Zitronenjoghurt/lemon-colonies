use crate::error::CoreResult;
use crate::game::object::command::ObjectCommand;
use crate::game::resource::ResourceId;
use crate::math::coords::ChunkCoords;
use crate::math::rect::Rect;
use crate::messages::client::object_placement::ObjectPlacement;
use crate::messages::client::object_purchase::ObjectPurchase;
use std::collections::HashSet;

pub mod object_placement;
pub mod object_purchase;

#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
pub enum ClientMessage {
    Ping { client_time: f64 },
    AllResources,
    ColonyPositions,
    ObjectCommand(ObjectCommand),
    ObjectsInChunks(HashSet<ChunkCoords>),
    ObjectPlacement(ObjectPlacement),
    ObjectPurchase(ObjectPurchase),
    Resources(HashSet<ResourceId>),
    SubscribeToChunks(Rect<i32>),
    OwnedChunks,
    UserInfo,
}

impl ClientMessage {
    #[cfg(feature = "bitcode")]
    pub fn as_bytes(&self) -> Vec<u8> {
        bitcode::encode(self)
    }

    #[cfg(feature = "bitcode")]
    pub fn from_bytes(bytes: &[u8]) -> CoreResult<Self> {
        Ok(bitcode::decode(bytes)?)
    }

    pub fn cost(&self) -> f64 {
        match self {
            Self::Ping { .. } => 1.0,
            Self::AllResources => 3.0,
            Self::ColonyPositions => 3.0,
            Self::ObjectCommand(_) => 6.0,
            Self::ObjectsInChunks(_) => 6.0,
            Self::ObjectPlacement(_) => 6.0,
            Self::ObjectPurchase(_) => 6.0,
            Self::Resources(_) => 3.0,
            Self::SubscribeToChunks(_) => 12.0,
            Self::OwnedChunks => 3.0,
            Self::UserInfo => 3.0,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Ping { .. } => "ping",
            Self::AllResources => "all_resources",
            Self::ColonyPositions => "colony_positions",
            Self::ObjectCommand(_) => "object_command",
            Self::ObjectsInChunks(_) => "object_in_chunks",
            Self::ObjectPlacement(_) => "object_placement",
            Self::ObjectPurchase(_) => "object_purchase",
            Self::Resources(_) => "resources",
            Self::SubscribeToChunks(_) => "subscribe_to_chunks",
            Self::OwnedChunks => "owned_chunks",
            Self::UserInfo => "user_info",
        }
    }
}
