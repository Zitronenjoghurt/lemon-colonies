use crate::game::terrain::Terrain;

pub const CHUNK_EDGE_LENGTH: usize = 32;
pub const CHUNK_SIZE: usize = CHUNK_EDGE_LENGTH * CHUNK_EDGE_LENGTH;

pub struct Chunk {
    pub x: i32,
    pub y: i32,
    pub terrain: [Terrain; CHUNK_SIZE],
}

impl Chunk {
    pub fn generate(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
            terrain: [Terrain::GrassPlain; CHUNK_SIZE],
        }
    }

    pub fn get_terrain(&self, x: usize, y: usize) -> Option<Terrain> {
        self.terrain.get(y * CHUNK_EDGE_LENGTH + x).copied()
    }
}
