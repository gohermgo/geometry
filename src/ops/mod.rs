//! Operators for geometry
mod cross;
use std::{
    ops::Sub,
    simd::{f32x4, num::SimdFloat},
};

pub use cross::Cross;
mod dot;
pub use dot::Dot;
pub trait AbsDiff<Rhs>: Sub<Rhs> {
    fn abs_diff(self, rhs: Rhs) -> Self::Output;
}
impl AbsDiff<f32> for f32 {
    #[inline]
    fn abs_diff(self, rhs: f32) -> Self::Output {
        (self - rhs).abs()
    }
}
impl AbsDiff<f32> for &f32 {
    #[inline]
    fn abs_diff(self, rhs: f32) -> Self::Output {
        (self - rhs).abs()
    }
}
impl AbsDiff<&f32> for f32 {
    #[inline]
    fn abs_diff(self, rhs: &f32) -> Self::Output {
        (self - rhs).abs()
    }
}
impl AbsDiff<&f32> for &f32 {
    #[inline]
    fn abs_diff(self, rhs: &f32) -> Self::Output {
        (self - rhs).abs()
    }
}
impl AbsDiff<f32x4> for f32x4 {
    #[inline]
    fn abs_diff(self, rhs: f32x4) -> Self::Output {
        (self - rhs).abs()
    }
}
impl AbsDiff<f32x4> for &f32x4 {
    #[inline]
    fn abs_diff(self, rhs: f32x4) -> Self::Output {
        (self - rhs).abs()
    }
}
impl AbsDiff<&f32x4> for f32x4 {
    #[inline]
    fn abs_diff(self, rhs: &f32x4) -> Self::Output {
        (self - rhs).abs()
    }
}
impl AbsDiff<&f32x4> for &f32x4 {
    #[inline]
    fn abs_diff(self, rhs: &f32x4) -> Self::Output {
        (self - rhs).abs()
    }
}
