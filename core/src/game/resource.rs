use strum_macros::{EnumCount, EnumIter, FromRepr};

#[derive(Debug, Clone, Copy, Eq, PartialEq, EnumIter, EnumCount, FromRepr)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u16)]
pub enum ResourceId {
    Blueberry = 0,
    Raspberry = 1,
    Golberry = 2,
}
