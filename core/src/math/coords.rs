use crate::game::chunk::CHUNK_EDGE_PIXELS;
use crate::math::point::Point;
use std::ops::{Add, Sub};

#[derive(Debug, Default, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WorldCoords {
    pub x: f32,
    pub y: f32,
}

impl WorldCoords {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn chunk(&self) -> ChunkCoords {
        ChunkCoords::new(
            (self.x / CHUNK_EDGE_PIXELS as f32).floor() as i32,
            (self.y / CHUNK_EDGE_PIXELS as f32).floor() as i32,
        )
    }

    pub fn local(&self) -> LocalCoords {
        LocalCoords::new(
            (self.x as i32).rem_euclid(CHUNK_EDGE_PIXELS as i32) as u8,
            (self.y as i32).rem_euclid(CHUNK_EDGE_PIXELS as i32) as u8,
        )
    }

    pub fn chunk_local(&self) -> ChunkLocal {
        ChunkLocal::new(self.chunk(), self.local())
    }

    pub fn floor(&self) -> WorldCoords {
        WorldCoords::new(self.x.floor(), self.y.floor())
    }

    pub fn ceil(&self) -> WorldCoords {
        WorldCoords::new(self.x.ceil(), self.y.ceil())
    }
}

impl From<WorldCoords> for (f32, f32) {
    fn from(coords: WorldCoords) -> Self {
        (coords.x, coords.y)
    }
}

impl From<(f32, f32)> for WorldCoords {
    fn from((x, y): (f32, f32)) -> Self {
        Self { x, y }
    }
}

impl Add<WorldCoords> for WorldCoords {
    type Output = WorldCoords;

    fn add(self, other: WorldCoords) -> Self::Output {
        Self::new(self.x + other.x, self.y + other.y)
    }
}

impl Sub<WorldCoords> for WorldCoords {
    type Output = WorldCoords;

    fn sub(self, other: WorldCoords) -> Self::Output {
        Self::new(self.x - other.x, self.y - other.y)
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChunkCoords {
    pub x: i32,
    pub y: i32,
}

impl ChunkCoords {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn point(&self) -> Point<i32> {
        Point::new(self.x, self.y)
    }
}

impl From<ChunkCoords> for (i32, i32) {
    fn from(coords: ChunkCoords) -> Self {
        (coords.x, coords.y)
    }
}

impl From<(i32, i32)> for ChunkCoords {
    fn from((x, y): (i32, i32)) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LocalCoords {
    pub x: u8,
    pub y: u8,
}

impl LocalCoords {
    pub fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }

    pub fn with_chunk(&self, chunk: ChunkCoords) -> ChunkLocal {
        ChunkLocal::new(chunk, *self)
    }
}

impl From<LocalCoords> for (u8, u8) {
    fn from(coords: LocalCoords) -> Self {
        (coords.x, coords.y)
    }
}

impl From<(u8, u8)> for LocalCoords {
    fn from((x, y): (u8, u8)) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChunkLocal {
    pub chunk: ChunkCoords,
    pub local: LocalCoords,
}

impl ChunkLocal {
    pub fn new(chunk: ChunkCoords, local: LocalCoords) -> Self {
        Self { chunk, local }
    }

    pub fn world(&self) -> WorldCoords {
        WorldCoords::new(
            self.chunk.x as f32 * CHUNK_EDGE_PIXELS as f32 + self.local.x as f32,
            self.chunk.y as f32 * CHUNK_EDGE_PIXELS as f32 + self.local.y as f32,
        )
    }
}
