use crate::error::{CoreError, CoreResult};
use crate::game::object::{ObjectData, ObjectId};
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
    pub data: ObjectData,
}

#[cfg(feature = "data")]
impl TryFrom<crate::data::entity::chunk::Model> for Chunk {
    type Error = CoreError;

    fn try_from(model: crate::data::entity::chunk::Model) -> CoreResult<Self> {
        Self::try_from((model, Vec::new()))
    }
}

#[cfg(feature = "data")]
impl
    TryFrom<(
        crate::data::entity::chunk::Model,
        Vec<crate::data::entity::object::Model>,
    )> for Chunk
{
    type Error = CoreError;

    fn try_from(
        (chunk, objects): (
            crate::data::entity::chunk::Model,
            Vec<crate::data::entity::object::Model>,
        ),
    ) -> CoreResult<Self> {
        let terrain: [Terrain; CHUNK_SIZE] = chunk
            .terrain
            .chunks_exact(2)
            .map(|b| {
                Terrain::from_repr(u16::from_le_bytes([b[0], b[1]]))
                    .ok_or(CoreError::InvalidTerrain)
            })
            .collect::<Result<Vec<_>, _>>()?
            .try_into()
            .map_err(|_| CoreError::InvalidChunkTerrainSize)?;

        let objects = objects
            .into_iter()
            .map(<(ObjectId, ChunkObject)>::try_from)
            .collect::<CoreResult<HashMap<_, _>>>()?;

        Ok(Self {
            x: chunk.x,
            y: chunk.y,
            terrain,
            objects,
        })
    }
}

#[cfg(feature = "data")]
impl TryFrom<crate::data::entity::object::Model> for (ObjectId, ChunkObject) {
    type Error = CoreError;

    fn try_from(model: crate::data::entity::object::Model) -> CoreResult<Self> {
        let data: ObjectData =
            serde_json::from_value(model.data).map_err(|_| CoreError::InvalidObjectData)?;
        Ok((
            ObjectId::from(model.id),
            ChunkObject {
                x: model.x as u8,
                y: model.y as u8,
                data,
            },
        ))
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
