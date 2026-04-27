use crate::game::object::data::bush::BushObject;
use crate::game::object::data::ObjectData;
use strum_macros::{EnumCount, EnumIter, FromRepr};

#[derive(Debug, Clone, Eq, PartialEq, EnumIter, EnumCount, FromRepr)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u16)]
pub enum ObjectKind {
    Bush,
}

impl ObjectKind {
    pub fn default_data(&self) -> ObjectData {
        match self {
            Self::Bush => ObjectData::Bush(BushObject::default()),
        }
    }
}
