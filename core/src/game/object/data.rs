use crate::game::object::command::{ObjectCommandKind, ObjectCommandResult};
use crate::game::object::kind::ObjectKind;
use crate::game::object::ObjectId;
use crate::math::coords::WorldCoords;
use crate::math::point::Point;
use crate::math::rect::Rect;

pub mod bush;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ObjectData {
    Bush(bush::BushObject),
}

impl ObjectData {
    /// The width of this object in pixels
    pub const fn width(&self) -> f32 {
        match self {
            Self::Bush(_) => 10.0,
        }
    }

    /// The height of this object in pixels
    pub const fn height(&self) -> f32 {
        match self {
            Self::Bush(_) => 10.0,
        }
    }

    pub const fn bounding_rect(&self, pos: WorldCoords) -> Rect<f32> {
        let hw = self.width() / 2.0;
        let h = self.height();
        Rect::new(
            Point::new(pos.x - hw, pos.y - h),
            Point::new(pos.x + hw, pos.y),
        )
    }

    pub const fn collision_height(&self) -> f32 {
        match self {
            Self::Bush(_) => 8.0,
        }
    }

    pub const fn collision_width(&self) -> f32 {
        match self {
            Self::Bush(_) => 8.0,
        }
    }

    pub const fn collision_rect(&self, pos: WorldCoords) -> Rect<f32> {
        let hw = self.collision_width() / 2.0;
        let h = self.collision_height();
        Rect::new(
            Point::new(pos.x - hw, pos.y - h),
            Point::new(pos.x + hw, pos.y),
        )
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
            Self::Bush(_) => ObjectKind::Bush,
        }
    }

    pub fn tick(&mut self, id: ObjectId, delta: f64) {
        match self {
            Self::Bush(bush) => bush.tick(id, delta),
        }
    }

    pub fn apply_command(&mut self, command_kind: ObjectCommandKind) -> ObjectCommandResult {
        match self {
            Self::Bush(bush) => bush.apply_command(command_kind),
        }
    }

    pub fn can_interact(&self) -> bool {
        match self {
            Self::Bush(bush) => bush.can_interact(),
        }
    }
}
