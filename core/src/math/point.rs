use num_traits::Num;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Point<N: Num + Copy> {
    pub x: N,
    pub y: N,
}

impl<N: Num + Copy> Point<N> {
    pub const fn new(x: N, y: N) -> Self {
        Self { x, y }
    }
}

impl<N: Num + Copy> From<(N, N)> for Point<N> {
    fn from((x, y): (N, N)) -> Self {
        Self { x, y }
    }
}
