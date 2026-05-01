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

#[macro_export]
macro_rules! metric_counter {
    ($($args:tt)*) => {
        #[cfg(feature = "metrics")]
        metrics::counter!($($args)*).increment(1);
    };
}

#[macro_export]
macro_rules! metric_gauge {
    ($name:expr, $value:expr) => {
        #[cfg(feature = "metrics")]
        metrics::gauge!($name).set($value);
    };
}

#[macro_export]
macro_rules! metric_histogram {
    ($name:expr, $value:expr) => {
        #[cfg(feature = "metrics")]
        metrics::histogram!($name).record($value);
    };
}
