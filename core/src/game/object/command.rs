use crate::game::object::ObjectId;
use crate::game::resource::ResourceId;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ObjectCommand {
    pub target: ObjectId,
    pub kind: ObjectCommandKind,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ObjectCommandKind {
    Interact,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ObjectCommandResult {
    None,
    ReceiveResources { id: ResourceId, amount: u64 },
}

impl ObjectCommandResult {
    pub fn receive_resources(id: ResourceId, amount: u64) -> Self {
        Self::ReceiveResources { id, amount }
    }
}
