use num_traits::Num;

pub mod circle;
pub mod point;
pub mod rect;

pub fn iter_int_range<N: Num + Copy + PartialOrd>(start: N, end: N) -> impl Iterator<Item = N> {
    std::iter::successors((start <= end).then_some(start), move |&x| {
        let next = x + N::one();
        (next <= end).then_some(next)
    })
}

pub trait Widen: Copy {
    type Wide: Copy
        + PartialOrd
        + std::ops::Add<Output = Self::Wide>
        + std::ops::Mul<Output = Self::Wide>;
    fn widen(self) -> Self::Wide;
}

impl Widen for i16 {
    type Wide = i32;
    fn widen(self) -> i32 {
        self as i32
    }
}
impl Widen for i32 {
    type Wide = i64;
    fn widen(self) -> i64 {
        self as i64
    }
}
impl Widen for u16 {
    type Wide = u32;
    fn widen(self) -> u32 {
        self as u32
    }
}
impl Widen for u32 {
    type Wide = u64;
    fn widen(self) -> u64 {
        self as u64
    }
}
impl Widen for f32 {
    type Wide = f32;
    fn widen(self) -> f32 {
        self
    }
}
impl Widen for f64 {
    type Wide = f64;
    fn widen(self) -> f64 {
        self
    }
}
