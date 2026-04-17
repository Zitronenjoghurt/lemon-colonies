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
    pub fn new(min: Point<N>, max: Point<N>) -> Self {
        Self { min, max }
    }

    pub fn area(&self) -> N {
        (self.max.x - self.min.x) * (self.max.y - self.min.y)
    }
}

impl<N: Num + Copy + PartialOrd> Rect<N> {
    pub fn contains(&self, p: &Point<N>) -> bool {
        p.x >= self.min.x && p.x <= self.max.x && p.y >= self.min.y && p.y <= self.max.y
    }

    pub fn iter_points(&self) -> impl Iterator<Item = Point<N>> {
        let min = self.min;
        let max = self.max;
        iter_int_range(min.y, max.y)
            .flat_map(move |y| iter_int_range(min.x, max.x).map(move |x| Point { x, y }))
    }
}
