use crate::game::chunk::CHUNK_EDGE_PIXELS;
use crate::math::coords::{ChunkCoords, WorldCoords};
use crate::math::iter_int_range;
use crate::math::point::Point;
use num_traits::Num;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Rect<N: Num + Copy> {
    pub min: Point<N>,
    pub max: Point<N>,
}

impl<N: Num + Copy> Rect<N> {
    #[inline(always)]
    pub const fn new(min: Point<N>, max: Point<N>) -> Self {
        Self { min, max }
    }

    #[inline(always)]
    pub fn from_size(origin: Point<N>, width: N, height: N) -> Self {
        Self {
            min: origin,
            max: Point {
                x: origin.x + width,
                y: origin.y + height,
            },
        }
    }

    #[inline(always)]
    pub fn width(&self) -> N {
        self.max.x - self.min.x
    }

    #[inline(always)]
    pub fn height(&self) -> N {
        self.max.y - self.min.y
    }

    #[inline(always)]
    pub fn area(&self) -> N {
        (self.max.x - self.min.x) * (self.max.y - self.min.y)
    }
}

impl<N: Num + Copy + PartialOrd> Rect<N> {
    #[inline(always)]
    pub fn contains_point(&self, p: &Point<N>) -> bool {
        p.x >= self.min.x && p.x <= self.max.x && p.y >= self.min.y && p.y <= self.max.y
    }

    pub fn iter_points(&self) -> impl Iterator<Item = Point<N>> {
        let min = self.min;
        let max = self.max;
        iter_int_range(min.y, max.y)
            .flat_map(move |y| iter_int_range(min.x, max.x).map(move |x| Point { x, y }))
    }

    #[inline(always)]
    pub fn overlaps_rect(&self, other: &Rect<N>) -> bool {
        self.min.x < other.max.x
            && self.max.x > other.min.x
            && self.min.y < other.max.y
            && self.max.y > other.min.y
    }

    #[inline(always)]
    pub fn within_rect(&self, other: &Rect<N>) -> bool {
        self.min.x >= other.min.x
            && self.max.x <= other.max.x
            && self.min.y >= other.min.y
            && self.max.y <= other.max.y
    }
}

impl Rect<f32> {
    pub fn chunk_range(&self) -> (ChunkCoords, ChunkCoords) {
        let min = WorldCoords::new(self.min.x, self.min.y).chunk();
        let max_x = self.max.x / CHUNK_EDGE_PIXELS as f32;
        let max_y = self.max.y / CHUNK_EDGE_PIXELS as f32;
        let max = ChunkCoords::new(
            if max_x == max_x.floor() {
                max_x as i32 - 1
            } else {
                max_x.floor() as i32
            },
            if max_y == max_y.floor() {
                max_y as i32 - 1
            } else {
                max_y.floor() as i32
            },
        );
        (min, max)
    }
}
