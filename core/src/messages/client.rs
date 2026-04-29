use crate::error::CoreResult;
use crate::game::object::command::ObjectCommand;
use crate::game::resource::ResourceId;
use crate::math::rect::Rect;
use object_placement::ObjectPlacement;
use std::collections::HashSet;

pub mod object_placement;

#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
pub enum ClientMessage {
    Ping { client_time: f64 },
    AllResources,
    ColonyPositions,
    ObjectCommand(ObjectCommand),
    ObjectPlacement(ObjectPlacement),
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
            Self::ObjectPlacement(_) => 6.0,
            Self::Resources(_) => 3.0,
            Self::SubscribeToChunks(_) => 12.0,
            Self::OwnedChunks => 3.0,
            Self::UserInfo => 3.0,
        }
    }
}
