use crate::game::object::command::{ObjectCommandKind, ObjectCommandResult};
use crate::game::object::kind::ObjectKind;
use crate::game::object::visuals::ObjectVisuals;
use crate::game::object::ObjectId;

pub mod bush;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ObjectData {
    Bush(bush::BushObject),
}

impl ObjectData {
    pub const fn kind(&self) -> ObjectKind {
        match self {
            Self::Bush(_) => ObjectKind::Bush,
        }
    }

    pub fn tick(&mut self, id: ObjectId, delta: f64) {
        match self {
            Self::Bush(bush) => bush.tick(id, delta),
        }
    }

    pub fn apply_command(&mut self, command_kind: ObjectCommandKind) -> ObjectCommandResult {
        match self {
            Self::Bush(bush) => bush.apply_command(command_kind),
        }
    }

    pub fn can_interact(&self) -> bool {
        match self {
            Self::Bush(bush) => bush.can_interact(),
        }
    }

    pub const fn visuals(&self) -> ObjectVisuals {
        match self {
            Self::Bush(bush) => ObjectVisuals::Bush(bush.visuals()),
        }
    }
}
