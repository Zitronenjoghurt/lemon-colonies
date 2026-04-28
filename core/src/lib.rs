#[cfg(feature = "data")]
pub mod data;
pub mod error;
pub mod game;
pub mod lingo;
pub mod math;
pub mod messages;
pub mod types;

/// Golden Ratio Constant for hashing
const GRC_64: u64 = 0x9E3779B97F4A7C15;
