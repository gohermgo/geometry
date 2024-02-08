use crate::Vector;
use std::simd::num::SimdFloat;
pub trait Dot<Rhs: ?Sized> {
    type Output;
    fn dot(self, rhs: Rhs) -> Self::Output;
}
impl Dot<Vector> for Vector {
    type Output = f32;
    #[inline]
    fn dot(self, rhs: Vector) -> Self::Output {
        (self.0 .0 * rhs.0 .0).reduce_sum()
    }
}
impl Dot<&Vector> for Vector {
    type Output = f32;
    #[inline]
    fn dot(self, rhs: &Vector) -> Self::Output {
        (self.0 .0 * rhs.0 .0).reduce_sum()
    }
}
impl Dot<Vector> for &Vector {
    type Output = f32;
    #[inline]
    fn dot(self, rhs: Vector) -> Self::Output {
        (self.0 .0 * rhs.0 .0).reduce_sum()
    }
}
impl Dot<&Vector> for &Vector {
    type Output = f32;
    #[inline]
    fn dot(self, rhs: &Vector) -> Self::Output {
        (self.0 .0 * rhs.0 .0).reduce_sum()
    }
}
