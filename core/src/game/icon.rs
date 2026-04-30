use strum_macros::{EnumCount, EnumIter, FromRepr};

#[derive(Debug, Clone, Eq, PartialEq, Hash, EnumIter, EnumCount, FromRepr)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u16)]
pub enum Icon {
    BlueberryBush,
    RaspberryBush,
    GolberryBush,
    Blueberries,
    Raspberries,
    Golberries,
}
