use crate::error::{CoreError, CoreResult};
use crate::game::object::command::{ObjectCommandKind, ObjectCommandResult};
use crate::game::object::data::ObjectData;
use crate::game::object::{Object, ObjectId};
use crate::game::terrain::{Terrain, TERRAIN_SIZE};
use crate::math::coords::{ChunkCoords, LocalCoords};
use crate::math::rect::Rect;
use std::collections::HashMap;
use strum::EnumCount;

pub const CHUNK_EDGE_LENGTH: usize = 32;
pub const CHUNK_EDGE_PIXELS: usize = CHUNK_EDGE_LENGTH * TERRAIN_SIZE;
pub const CHUNK_SIZE: usize = CHUNK_EDGE_LENGTH * CHUNK_EDGE_LENGTH;

#[derive(Clone)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
pub struct Chunk {
    pub pos: ChunkCoords,
    pub objects: HashMap<ObjectId, ChunkObject>,
    pub terrain: [Terrain; CHUNK_SIZE],
}

impl Chunk {
    pub fn generate(pos: ChunkCoords, world_seed: u64) -> Self {
        let chunk_seed = world_seed
            .wrapping_add((pos.x as u64).wrapping_mul(0x51492FB0))
            .wrapping_add((pos.y as u64).wrapping_mul(0x9E3779B9));

        let mut rng = fastrand::Rng::with_seed(chunk_seed);
        let terrain = core::array::from_fn(|_| {
            let random_repr = rng.u16(0..Terrain::COUNT as u16);
            Terrain::from_repr(random_repr).unwrap_or_default()
        });

        Self {
            pos,
            objects: HashMap::new(),
            terrain,
        }
    }

    pub fn get_terrain(&self, x: usize, y: usize) -> Option<Terrain> {
        self.terrain.get(y * CHUNK_EDGE_LENGTH + x).copied()
    }

    pub fn update_object(&mut self, object: Object) {
        if let Some(obj) = self.objects.get_mut(&object.id) {
            obj.pos = object.pos.local;
            obj.data = object.data;
            obj.last_update = object.last_update;
            obj.created_at = object.created_at;
        } else {
            self.objects.insert(
                object.id,
                ChunkObject {
                    pos: object.pos.local,
                    data: object.data,
                    last_update: object.last_update,
                    created_at: object.created_at,
                },
            );
        }
    }

    pub fn rect_collides_with_object_collision(&self, rect: Rect<f32>) -> bool {
        for obj in self.objects.values() {
            let pos = obj.pos.with_chunk(self.pos).world();
            let collision = obj.data.collision_rect(pos);
            if rect.overlaps_rect(&collision) {
                return true;
            }
        }
        false
    }

    pub fn tick(&mut self, server_time: f64) {
        for (id, obj) in self.objects.iter_mut() {
            obj.tick(*id, server_time);
        }
    }
}

#[derive(Clone)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
pub struct ChunkObject {
    pub pos: LocalCoords,
    pub data: ObjectData,
    pub last_update: f64,
    pub created_at: f64,
}

impl ChunkObject {
    pub fn tick(&mut self, id: ObjectId, server_time: f64) {
        let delta = server_time - self.last_update;
        self.data.tick(id, delta);
        self.last_update = server_time;
    }

    pub fn apply_command(&mut self, command_kind: ObjectCommandKind) -> ObjectCommandResult {
        self.data.apply_command(command_kind)
    }

    pub fn can_interact(&self) -> bool {
        self.data.can_interact()
    }
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
            pos: ChunkCoords::new(chunk.x, chunk.y),
            terrain,
            objects,
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
            x: sea_orm::Set(chunk.pos.x),
            y: sea_orm::Set(chunk.pos.y),
            terrain: sea_orm::Set(terrain),
            ..Default::default()
        }
    }
}
