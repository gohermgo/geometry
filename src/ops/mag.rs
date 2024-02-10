use crate::Vector;
use std::{
    ops::Mul,
    simd::{f32x4, num::SimdFloat},
};
pub trait Mag {
    type Output;
    fn mag(&self) -> Self::Output;
}
impl Mag for f32x4 {
    type Output = f32;
    #[inline]
    fn mag(&self) -> Self::Output {
        self.mul(self).reduce_sum().sqrt()
    }
}
impl Mag for Vector {
    type Output = f32;
    #[inline]
    fn mag(&self) -> Self::Output {
        self.0 .0.mag()
    }
}
