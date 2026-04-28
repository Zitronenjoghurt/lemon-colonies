use crate::math::coords::ChunkCoords;
use crate::GRC_64;
use fastrand::Rng;

const SPACING_FACTOR: f64 = 16.0;
const JITTER_STRENGTH: f64 = 4.0;
const GOLDEN_ANGLE: f64 = 2.39996323f64;

/// https://en.wikipedia.org/wiki/Fermat%27s_spiral
pub fn determine_new_colony_position(total_colony_count: u64, world_seed: u64) -> ChunkCoords {
    if total_colony_count == 0 {
        return ChunkCoords::new(0, 0);
    };

    let n = total_colony_count as f64;
    let theta = n * GOLDEN_ANGLE;
    let radius = SPACING_FACTOR * n.sqrt();

    let base_x = radius * theta.cos();
    let base_y = radius * theta.sin();

    let combined_seed = world_seed ^ total_colony_count.wrapping_mul(GRC_64);
    let mut rng = Rng::with_seed(combined_seed);

    let rand_x = ((rng.f64() * 2.0) - 1.0) * JITTER_STRENGTH;
    let rand_y = ((rng.f64() * 2.0) - 1.0) * JITTER_STRENGTH;

    ChunkCoords::new(
        (base_x + rand_x).round() as i32,
        (base_y + rand_y).round() as i32,
    )
}
