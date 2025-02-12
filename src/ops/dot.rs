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
#[cfg(test)]
mod tests {
    use super::*;
    use crate::PointType;
    #[test]
    fn two_vectors() {
        let a = Vector::new(1.0, 2.0, 3.0);
        let b = Vector::new(2.0, 3.0, 4.0);
        assert!(a.dot(b) == 20.0);
    }
}
