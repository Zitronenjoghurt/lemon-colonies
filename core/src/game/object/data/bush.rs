use crate::game::object::ObjectId;
use crate::GOLDEN_RATIO_CONSTANT;
use strum_macros::{EnumCount, EnumIter, FromRepr};

const GROWTH_RATE_FLUCTUATION: f64 = 0.2;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BushObject {
    pub kind: BushKind,
    pub berries: u8,
    pub growth: f64,
    pub harvest_count: u32,
}

impl Default for BushObject {
    fn default() -> Self {
        Self {
            kind: BushKind::Blueberry,
            berries: 0,
            growth: 0.0,
            harvest_count: 0,
        }
    }
}

impl BushObject {
    pub fn tick(&mut self, id: ObjectId, delta: f64) {
        if self.berries >= self.max_berries() {
            return;
        };

        self.growth += self.growth_rate(id) * delta;

        let new_berries = (self.growth.floor() as u8).min(self.max_berries() - self.berries);
        self.berries = self.berries.saturating_add(new_berries);
        self.growth -= new_berries as f64;
    }

    pub fn growth_rate(&self, id: ObjectId) -> f64 {
        let seed = id.seed() ^ (self.harvest_count as u64).wrapping_mul(GOLDEN_RATIO_CONSTANT);
        let jitter = fastrand::Rng::with_seed(seed).f64();
        self.kind.base_growth_rate()
            * (1.0 - GROWTH_RATE_FLUCTUATION + GROWTH_RATE_FLUCTUATION * 2.0 * jitter)
    }

    pub fn max_berries(&self) -> u8 {
        self.kind.max_berries()
    }
}

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, EnumIter, EnumCount, FromRepr)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum BushKind {
    #[default]
    Blueberry = 0,
    Raspberry = 1,
    Golberry = 2,
}

impl BushKind {
    pub fn base_growth_rate(&self) -> f64 {
        match self {
            Self::Blueberry => 1.0 / 60.0,
            Self::Raspberry => 1.0 / 600.0,
            Self::Golberry => 1.0 / 3600.0,
        }
    }

    pub fn max_berries(&self) -> u8 {
        match self {
            Self::Blueberry => 20,
            Self::Raspberry => 10,
            Self::Golberry => 5,
        }
    }
}
