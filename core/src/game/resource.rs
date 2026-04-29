use std::collections::HashMap;
use std::fmt::Display;
use strum_macros::{EnumCount, EnumIter, FromRepr};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, EnumIter, EnumCount, FromRepr)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u16)]
pub enum ResourceId {
    Blueberry = 0,
    Raspberry = 1,
    Golberry = 2,
}

impl Display for ResourceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
pub struct ResourceBag {
    resources: HashMap<ResourceId, u64>,
}

impl ResourceBag {
    pub fn new(resources: HashMap<ResourceId, u64>) -> Self {
        Self { resources }
    }

    pub fn get(&self, resource_id: ResourceId) -> u64 {
        self.resources
            .get(&resource_id)
            .copied()
            .unwrap_or_default()
    }

    pub fn update(&mut self, resource_id: ResourceId, amount: u64) {
        self.resources
            .entry(resource_id)
            .and_modify(|amt| *amt += amount)
            .or_insert(amount);
    }

    pub fn merge_override(&mut self, other: Self) {
        for (rid, amt) in other.resources {
            self.resources.insert(rid, amt);
        }
    }
}
