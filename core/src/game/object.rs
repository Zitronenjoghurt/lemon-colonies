use crate::error::{CoreError, CoreResult};
use crate::game::chunk::ChunkObject;
use crate::math::coords::{ChunkCoords, ChunkLocal, LocalCoords};
use data::ObjectData;
use uuid::Uuid;

pub mod data;
pub mod kind;

#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ObjectId([u8; 16]);

impl From<Uuid> for ObjectId {
    fn from(value: Uuid) -> Self {
        Self(value.into_bytes())
    }
}

impl From<ObjectId> for Uuid {
    fn from(value: ObjectId) -> Self {
        Self::from_bytes(value.0)
    }
}

impl ObjectId {
    pub fn seed(&self) -> u64 {
        u64::from_le_bytes(self.0[..8].try_into().unwrap())
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Object {
    pub id: ObjectId,
    pub pos: ChunkLocal,
    pub data: ObjectData,
    pub last_update: f64,
}

impl Object {
    pub fn tick(&mut self, server_time: f64) {
        let delta = server_time - self.last_update;
        self.data.tick(self.id, delta);
        self.last_update = server_time;
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
                pos: LocalCoords::new(model.x as u8, model.y as u8),
                data: ObjectData::from(data),
                last_update: model.updated_at.and_utc().timestamp_millis() as f64 / 1000.0,
            },
        ))
    }
}

#[cfg(feature = "data")]
impl TryFrom<crate::data::entity::object::Model> for Object {
    type Error = CoreError;

    fn try_from(model: crate::data::entity::object::Model) -> CoreResult<Self> {
        let data: ObjectData =
            serde_json::from_value(model.data).map_err(|_| CoreError::InvalidObjectData)?;
        Ok(Object {
            id: ObjectId::from(model.id),
            pos: ChunkLocal::new(
                ChunkCoords::new(model.chunk_x, model.chunk_y),
                LocalCoords::new(model.x as u8, model.y as u8),
            ),
            data: ObjectData::from(data),
            last_update: model.updated_at.and_utc().timestamp_millis() as f64 / 1000.0,
        })
    }
}
