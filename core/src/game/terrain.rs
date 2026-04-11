use strum_macros::{EnumCount, EnumIter, FromRepr};

pub const TERRAIN_SIZE: usize = 8;

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, EnumIter, EnumCount, FromRepr)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u16)]
pub enum Terrain {
    #[default]
    GrassPlain = 0,
    GrassVariant1 = 1,
    GrassVariant2 = 2,
    GrassVariant3 = 3,
    GrassVariant4 = 4,
    GrassVariant5 = 5,
    GrassVariant6 = 6,
    GrassVariant7 = 7,
    GrassFlowersRoundYellowBig = 8,
    GrassFlowersRoundCyanBig = 9,
    GrassFlowersRoundMagentaBig = 10,
    GrassFlowersRoundWhiteBig = 11,
    GrassFlowersRoundYellowSmall = 12,
    GrassFlowersRoundCyanSmall = 13,
    GrassFlowersRoundMagentaSmall = 14,
    GrassFlowersRoundWhiteSmall = 15,
    GrassFlowersCrossWhiteBig = 16,
    GrassFlowersCrossWhiteSmall = 17,
    GrassFlowersCrossYellowBig = 18,
    GrassFlowersCrossYellowSmall = 19,
    GrassFlowersCrossCyanBig = 20,
    GrassFlowersCrossCyanSmall = 21,
    GrassFlowersCrossMagentaBig = 22,
    GrassFlowersCrossMagentaSmall = 23,
    GrassShroomsRedBig = 24,
    GrassShroomsBrownBig = 25,
    GrassShroomsRedSmall = 26,
    GrassShroomsBrownSmall = 27,
}
