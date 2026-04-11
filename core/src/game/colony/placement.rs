use fastrand::Rng;

const SPACING_FACTOR: f64 = 16.0;
const JITTER_STRENGTH: f64 = 4.0;
const GOLDEN_ANGLE: f64 = 2.39996323f64;

/// https://en.wikipedia.org/wiki/Fermat%27s_spiral
pub fn determine_new_colony_position(total_colony_count: u64, world_seed: u64) -> (i32, i32) {
    if total_colony_count == 0 {
        return (0, 0);
    };

    let n = total_colony_count as f64;
    let theta = n * GOLDEN_ANGLE;
    let radius = SPACING_FACTOR * n.sqrt();

    let base_x = radius * theta.cos();
    let base_y = radius * theta.sin();

    let combined_seed = world_seed ^ total_colony_count.wrapping_mul(0x9E3779B97F4A7C15);
    let mut rng = Rng::with_seed(combined_seed);

    let rand_x = ((rng.f64() * 2.0) - 1.0) * JITTER_STRENGTH;
    let rand_y = ((rng.f64() * 2.0) - 1.0) * JITTER_STRENGTH;

    (
        (base_x + rand_x).round() as i32,
        (base_y + rand_y).round() as i32,
    )
}
