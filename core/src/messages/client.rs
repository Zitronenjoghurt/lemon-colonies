use crate::error::CoreResult;

#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
pub enum ClientMessage {
    Hello,
    ColonyPositions,
    RequestChunks(Vec<(i32, i32)>),
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
