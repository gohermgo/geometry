use crate::{ops::Mag, Vector};
use std::simd::f32x4;

pub trait Norm {
    type Output;
    fn norm(&self) -> Self::Output;
}
impl Norm for f32x4 {
    type Output = f32x4;
    #[inline]
    fn norm(&self) -> Self::Output {
        self / f32x4::from_array([self.mag(); 4])
    }
}
impl Norm for Vector {
    type Output = Vector;
    #[inline]
    fn norm(&self) -> Self::Output {
        self / self.mag()
    }
}
pub trait NormAssign {
    fn norm_assign(&mut self);
}
impl NormAssign for f32x4 {
    #[inline]
    fn norm_assign(&mut self) {
        *self /= f32x4::from_array([self.mag(); 4])
    }
}
impl NormAssign for Vector {
    #[inline]
    fn norm_assign(&mut self) {
        *self /= self.mag()
    }
}
