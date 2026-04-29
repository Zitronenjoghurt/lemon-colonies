use crate::game::object::ObjectId;
use crate::game::resource::ResourceId;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ObjectCommand {
    pub target: ObjectId,
    pub kind: ObjectCommandKind,
}

impl ObjectCommand {
    pub fn interact(id: ObjectId) -> Self {
        Self {
            target: id,
            kind: ObjectCommandKind::Interact,
        }
    }
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
pub struct ObjectCommandResult {
    pub dirty: bool,
    pub kind: ObjectCommandResultKind,
}

impl ObjectCommandResult {
    pub fn with_dirty(mut self, dirty: bool) -> Self {
        self.dirty = dirty;
        self
    }

    pub fn none() -> Self {
        Self {
            dirty: false,
            kind: ObjectCommandResultKind::None,
        }
    }

    pub fn receive_resources(id: ResourceId, amount: u64) -> Self {
        Self {
            dirty: false,
            kind: ObjectCommandResultKind::ReceiveResources { id, amount },
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ObjectCommandResultKind {
    None,
    ReceiveResources { id: ResourceId, amount: u64 },
}
