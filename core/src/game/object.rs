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
    pub kind: ObjectKind,
}

#[derive(Debug, Default, Clone, Eq, PartialEq, EnumIter, EnumCount, FromRepr)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ObjectKind {
    #[default]
    Bush,
}

impl ObjectKind {
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
}
