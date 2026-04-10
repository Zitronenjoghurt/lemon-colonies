use crate::error::CoreResult;

#[derive(Clone)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ServerMessage {
    Hello,
    Shutdown,
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

    #[cfg(feature = "serde")]
    pub fn as_json(&self) -> CoreResult<String> {
        Ok(serde_json::to_string(self)?)
    }

    #[cfg(feature = "serde")]
    pub fn from_json(json: &str) -> CoreResult<Self> {
        Ok(serde_json::from_str(json)?)
    }
}
