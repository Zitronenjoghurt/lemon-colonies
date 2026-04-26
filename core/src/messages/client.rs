use crate::error::CoreResult;
use crate::math::rect::Rect;
use object_placement::ObjectPlacement;

pub mod object_placement;

#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
pub enum ClientMessage {
    Ping { client_time: f64 },
    ColonyPositions,
    ObjectPlacement(ObjectPlacement),
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
}
