use crate::math::circle::Circle;
use crate::math::point::Point;

#[derive(Default, Clone)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChunkVisibility {
    pub areas: Vec<Circle<i32>>,
}

impl ChunkVisibility {
    pub fn insert(&mut self, x: i32, y: i32, radius: i32) {
        let center = Point::new(x, y);
        self.areas.push(Circle::new(center, radius));
    }

    pub fn is_visible(&self, target_x: i32, target_y: i32) -> bool {
        self.areas
            .iter()
            .any(|entry| entry.contains(&Point::new(target_x, target_y)))
    }
}
