use crate::game::icon::Icon;
use crate::game::object::data::bush::BushObject;
use crate::game::object::data::ObjectData;
use crate::game::resource::ResourceId;
use strum_macros::{EnumCount, EnumIter, FromRepr};

#[derive(Debug, Clone, Copy, Eq, PartialEq, EnumIter, EnumCount, FromRepr)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u16)]
pub enum PurchasableObject {
    BlueberryBush,
    RaspberryBush,
    GolberryBush,
}

impl PurchasableObject {
    pub fn icon(&self) -> Icon {
        match self {
            Self::BlueberryBush => Icon::BlueberryBush,
            Self::RaspberryBush => Icon::RaspberryBush,
            Self::GolberryBush => Icon::GolberryBush,
        }
    }

    pub fn base_costs(&self) -> &[(ResourceId, u64)] {
        match self {
            Self::BlueberryBush => &[(ResourceId::Blueberry, 3)],
            Self::RaspberryBush => &[(ResourceId::Blueberry, 10)],
            Self::GolberryBush => &[(ResourceId::Raspberry, 5)],
        }
    }

    pub fn resource_adjustments(&self) -> Vec<(ResourceId, i64)> {
        self.base_costs()
            .iter()
            .map(|(rid, amt)| (*rid, -(*amt as i64)))
            .collect()
    }

    pub fn object_data(&self) -> ObjectData {
        match self {
            Self::BlueberryBush => ObjectData::Bush(BushObject::blueberry()),
            Self::RaspberryBush => ObjectData::Bush(BushObject::raspberry()),
            Self::GolberryBush => ObjectData::Bush(BushObject::golberry()),
        }
    }
}
