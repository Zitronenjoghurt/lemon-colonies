use crate::error::{CoreError, CoreResult};
use crate::game::chunk::ChunkObject;
use crate::game::object::command::ObjectCommandResult;
use crate::game::object::visuals::ObjectVisuals;
use crate::math::coords::{ChunkCoords, ChunkLocal, LocalCoords};
use data::ObjectData;
use uuid::Uuid;

pub mod command;
pub mod data;
pub mod kind;
pub mod purchase;
pub mod visuals;

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
    pub data: Option<ObjectData>,
    pub visuals: ObjectVisuals,
    pub last_update: f64,
    pub created_at: f64,
}

impl Object {
    pub fn tick(&mut self, server_time: f64) {
        let Some(data) = &mut self.data else {
            return;
        };

        let delta = server_time - self.last_update;
        data.tick(self.id, delta);
        self.last_update = server_time;

        self.visuals = data.visuals();
    }

    pub fn apply_command(&mut self, command: command::ObjectCommand) -> ObjectCommandResult {
        if let Some(data) = &mut self.data {
            data.apply_command(command.kind)
        } else {
            ObjectCommandResult::none()
        }
    }

    pub fn can_interact(&self) -> bool {
        self.data.as_ref().is_some_and(|data| data.can_interact())
    }

    pub fn anonymize(&mut self) {
        self.data = None;
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
                visuals: data.visuals(),
                data: Some(data),
                last_update: model.updated_at.and_utc().timestamp_millis() as f64 / 1000.0,
                created_at: model.created_at.and_utc().timestamp_millis() as f64 / 1000.0,
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
            visuals: data.visuals(),
            data: Some(data),
            last_update: model.updated_at.and_utc().timestamp_millis() as f64 / 1000.0,
            created_at: model.created_at.and_utc().timestamp_millis() as f64 / 1000.0,
        })
    }
}

#[cfg(feature = "data")]
impl TryFrom<&Object> for crate::data::entity::object::ActiveModel {
    type Error = CoreError;

    fn try_from(obj: &Object) -> CoreResult<Self> {
        Ok(Self {
            id: sea_orm::Unchanged(obj.id.into()),
            data: sea_orm::Set(
                serde_json::to_value(&obj.data).map_err(|_| CoreError::InvalidObjectData)?,
            ),
            x: sea_orm::Set(obj.pos.local.x as i16),
            y: sea_orm::Set(obj.pos.local.y as i16),
            chunk_x: sea_orm::Set(obj.pos.chunk.x),
            chunk_y: sea_orm::Set(obj.pos.chunk.y),
            ..Default::default()
        })
    }
}
