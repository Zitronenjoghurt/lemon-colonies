#[derive(Default, Clone)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChunkVisibility {
    pub entries: Vec<ChunkVisibilityEntry>,
}

impl ChunkVisibility {
    pub fn insert(&mut self, x: i32, y: i32, radius: i32) {
        self.entries.push(ChunkVisibilityEntry { x, y, radius });
    }

    pub fn is_visible(&self, target_x: i32, target_y: i32) -> bool {
        self.entries.iter().any(|entry| {
            let dx = (entry.x - target_x) as i64;
            let dy = (entry.y - target_y) as i64;
            let radius = entry.radius as i64;
            (dx * dx) + (dy * dy) <= (radius * radius)
        })
    }
}

#[derive(Copy, Clone)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChunkVisibilityEntry {
    pub x: i32,
    pub y: i32,
    pub radius: i32,
}
