use crate::error::{CoreError, CoreResult};
use crate::game::object::{ObjectId, ObjectKind};
use crate::game::terrain::{Terrain, TERRAIN_SIZE};
use std::collections::HashMap;
use strum::EnumCount;

pub const CHUNK_EDGE_LENGTH: usize = 32;
pub const CHUNK_EDGE_PIXELS: usize = CHUNK_EDGE_LENGTH * TERRAIN_SIZE;
pub const CHUNK_SIZE: usize = CHUNK_EDGE_LENGTH * CHUNK_EDGE_LENGTH;

#[derive(Clone)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
pub struct Chunk {
    pub x: i32,
    pub y: i32,
    pub objects: HashMap<ObjectId, ChunkObject>,
    pub terrain: [Terrain; CHUNK_SIZE],
}

impl Chunk {
    pub fn generate(x: i32, y: i32, world_seed: u64) -> Self {
        let chunk_seed = world_seed
            .wrapping_add((x as u64).wrapping_mul(0x51492FB0))
            .wrapping_add((y as u64).wrapping_mul(0x9E3779B9));

        let mut rng = fastrand::Rng::with_seed(chunk_seed);
        let terrain = core::array::from_fn(|_| {
            let random_repr = rng.u16(0..Terrain::COUNT as u16);
            Terrain::from_repr(random_repr).unwrap_or_default()
        });

        Self {
            x,
            y,
            objects: HashMap::new(),
            terrain,
        }
    }

    pub fn get_terrain(&self, x: usize, y: usize) -> Option<Terrain> {
        self.terrain.get(y * CHUNK_EDGE_LENGTH + x).copied()
    }
}

#[derive(Clone)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
pub struct ChunkObject {
    pub x: u8,
    pub y: u8,
    pub kind: ObjectKind,
}

#[cfg(feature = "data")]
impl TryFrom<crate::data::entity::chunk::Model> for Chunk {
    type Error = CoreError;

    fn try_from(model: crate::data::entity::chunk::Model) -> CoreResult<Self> {
        let terrain: [Terrain; CHUNK_SIZE] = model
            .terrain
            .chunks_exact(2)
            .map(|b| {
                Terrain::from_repr(u16::from_le_bytes([b[0], b[1]]))
                    .ok_or(CoreError::InvalidTerrain)
            })
            .collect::<Result<Vec<_>, _>>()?
            .try_into()
            .map_err(|_| CoreError::InvalidChunkTerrainSize)?;

        Ok(Self {
            x: model.x,
            y: model.y,
            terrain,
            objects: HashMap::new(),
        })
    }
}

#[cfg(feature = "data")]
impl From<Chunk> for crate::data::entity::chunk::ActiveModel {
    fn from(chunk: Chunk) -> Self {
        let terrain = chunk
            .terrain
            .iter()
            .flat_map(|t| (*t as u16).to_le_bytes())
            .collect();
        crate::data::entity::chunk::ActiveModel {
            x: sea_orm::Set(chunk.x),
            y: sea_orm::Set(chunk.y),
            terrain: sea_orm::Set(terrain),
            ..Default::default()
        }
    }
}
