use crate::math::point::Point;
use crate::math::Widen;
use num_traits::Num;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Circle<N: Num + Copy> {
    pub center: Point<N>,
    pub radius: N,
}

impl<N: Num + Copy> Circle<N> {
    pub fn new(center: Point<N>, radius: N) -> Self {
        Self { center, radius }
    }
}

impl<N> Circle<N>
where
    N: Num + Copy + Widen,
{
    pub fn contains(&self, p: &Point<N>) -> bool {
        let dx = (p.x - self.center.x).widen();
        let dy = (p.y - self.center.y).widen();
        let r = self.radius.widen();
        (dx * dx) + (dy * dy) <= (r * r)
    }
}
