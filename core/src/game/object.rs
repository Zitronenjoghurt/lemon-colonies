use crate::error::{CoreError, CoreResult};
use crate::game::chunk::ChunkObject;
use strum_macros::{EnumCount, EnumIter, FromRepr};
use uuid::Uuid;

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

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Object {
    pub id: ObjectId,
    pub chunk_x: i32,
    pub chunk_y: i32,
    pub x: u8,
    pub y: u8,
    pub data: ObjectData,
}

#[derive(Debug, Clone, Eq, PartialEq, EnumIter, EnumCount)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ObjectData {
    Bush,
}

impl ObjectData {
    /// The width of this object in pixels
    pub const fn width(&self) -> f32 {
        match self {
            Self::Bush => 10.0,
        }
    }

    /// The height of this object in pixels
    pub const fn height(&self) -> f32 {
        match self {
            Self::Bush => 10.0,
        }
    }

    pub const fn pivot(&self) -> (f32, f32) {
        (self.width() / 2.0, self.height())
    }

    pub const fn center(&self) -> (f32, f32) {
        (self.width() / 2.0, self.height() / 2.0)
    }

    pub const fn pivot_center_offset(&self) -> (f32, f32) {
        (
            self.pivot().0 - self.center().0,
            self.pivot().1 - self.center().1,
        )
    }

    pub const fn kind(&self) -> ObjectKind {
        match self {
            Self::Bush => ObjectKind::Bush,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, EnumIter, EnumCount, FromRepr)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u16)]
pub enum ObjectKind {
    Bush,
}

impl ObjectKind {
    pub const fn default_data(&self) -> ObjectData {
        match self {
            Self::Bush => ObjectData::Bush,
        }
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
impl TryFrom<crate::data::entity::object::Model> for Object {
    type Error = CoreError;

    fn try_from(model: crate::data::entity::object::Model) -> CoreResult<Self> {
        let data: ObjectData =
            serde_json::from_value(model.data).map_err(|_| CoreError::InvalidObjectData)?;
        Ok(Object {
            id: ObjectId::from(model.id),
            chunk_x: model.chunk_x,
            chunk_y: model.chunk_y,
            x: model.x as u8,
            y: model.y as u8,
            data,
        })
    }
}
