use crate::Vector;
use std::simd::{f32x4, simd_swizzle};
pub(crate) const CROSS_SWIZZLE_0: [usize; 4] = [1, 2, 0, 3];
pub(crate) const CROSS_SWIZZLE_1: [usize; 4] = [2, 0, 1, 3];
pub trait Cross<Rhs: ?Sized> {
    type Output;
    fn cross(self, rhs: Rhs) -> Self::Output;
}
impl Cross<Vector> for Vector {
    type Output = Vector;
    #[inline]
    fn cross(self, rhs: Vector) -> Self::Output {
        let temp_0: f32x4 = simd_swizzle!(self.0 .0, CROSS_SWIZZLE_0);
        let temp_1: f32x4 = simd_swizzle!(rhs.0 .0, CROSS_SWIZZLE_1);
        let prod_0: f32x4 = temp_0 * temp_1;

        let temp_0: f32x4 = simd_swizzle!(self.0 .0, CROSS_SWIZZLE_1);
        let temp_1: f32x4 = simd_swizzle!(rhs.0 .0, CROSS_SWIZZLE_0);
        let prod_1: f32x4 = temp_0 * temp_1;
        Self((prod_0 - prod_1).into())
    }
}
impl Cross<Vector> for &Vector {
    type Output = Vector;
    #[inline]
    fn cross(self, rhs: Vector) -> Self::Output {
        let temp_0: f32x4 = simd_swizzle!(self.0 .0, CROSS_SWIZZLE_0);
        let temp_1: f32x4 = simd_swizzle!(rhs.0 .0, CROSS_SWIZZLE_1);
        let prod_0: f32x4 = temp_0 * temp_1;

        let temp_0: f32x4 = simd_swizzle!(self.0 .0, CROSS_SWIZZLE_1);
        let temp_1: f32x4 = simd_swizzle!(rhs.0 .0, CROSS_SWIZZLE_0);
        let prod_1: f32x4 = temp_0 * temp_1;
        Vector((prod_0 - prod_1).into())
    }
}
impl Cross<&Vector> for Vector {
    type Output = Vector;
    #[inline]
    fn cross(self, rhs: &Vector) -> Self::Output {
        let temp_0: f32x4 = simd_swizzle!(self.0 .0, CROSS_SWIZZLE_0);
        let temp_1: f32x4 = simd_swizzle!(rhs.0 .0, CROSS_SWIZZLE_1);
        let prod_0: f32x4 = temp_0 * temp_1;

        let temp_0: f32x4 = simd_swizzle!(self.0 .0, CROSS_SWIZZLE_1);
        let temp_1: f32x4 = simd_swizzle!(rhs.0 .0, CROSS_SWIZZLE_0);
        let prod_1: f32x4 = temp_0 * temp_1;
        Self((prod_0 - prod_1).into())
    }
}
impl Cross<&Vector> for &Vector {
    type Output = Vector;
    #[inline]
    fn cross(self, rhs: &Vector) -> Self::Output {
        let temp_0: f32x4 = simd_swizzle!(self.0 .0, CROSS_SWIZZLE_0);
        let temp_1: f32x4 = simd_swizzle!(rhs.0 .0, CROSS_SWIZZLE_1);
        let prod_0: f32x4 = temp_0 * temp_1;

        let temp_0: f32x4 = simd_swizzle!(self.0 .0, CROSS_SWIZZLE_1);
        let temp_1: f32x4 = simd_swizzle!(rhs.0 .0, CROSS_SWIZZLE_0);
        let prod_1: f32x4 = temp_0 * temp_1;
        Vector((prod_0 - prod_1).into())
    }
}
pub trait CrossAssign<Rhs: ?Sized> {
    fn cross_assign(&mut self, rhs: Rhs);
}
impl CrossAssign<Vector> for Vector {
    #[inline]
    fn cross_assign(&mut self, rhs: Vector) {
        let temp: f32x4 = simd_swizzle!(rhs.0 .0, CROSS_SWIZZLE_1);
        self.0 .0 = temp * simd_swizzle!(self.0 .0, CROSS_SWIZZLE_0);
        let temp: f32x4 = simd_swizzle!(rhs.0 .0, CROSS_SWIZZLE_0);
        self.0 .0 -= temp * simd_swizzle!(self.0 .0, CROSS_SWIZZLE_1);
    }
}
impl CrossAssign<&Vector> for Vector {
    #[inline]
    fn cross_assign(&mut self, rhs: &Vector) {
        let temp: f32x4 = simd_swizzle!(rhs.0 .0, CROSS_SWIZZLE_1);
        self.0 .0 = temp * simd_swizzle!(self.0 .0, CROSS_SWIZZLE_0);
        let temp: f32x4 = simd_swizzle!(rhs.0 .0, CROSS_SWIZZLE_0);
        self.0 .0 -= temp * simd_swizzle!(self.0 .0, CROSS_SWIZZLE_1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Pointlike;
    #[test]
    fn unit_1() {
        let a = Vector::new(1.0, 2.0, 3.0);
        let b = Vector::new(2.0, 3.0, 4.0);
        assert!(a.cross(b) == Vector::new(-1.0, 2.0, -1.0));
    }
    #[test]
    fn unit_2() {
        let a = Vector::new(1.0, 2.0, 3.0);
        let b = Vector::new(2.0, 3.0, 4.0);
        assert!(b.cross(a) == Vector::new(1.0, -2.0, 1.0));
    }
    #[test]
    fn unit_3() {
        let mut v = Vector::new(1.0, 2.0, 3.0);
        v.cross_assign(Vector::new(2.0, 3.0, 4.0));
        assert!(v == Vector::new(-1.0, 2.0, -1.0))
    }
    #[test]
    fn unit_4() {
        let mut v = Vector::new(2.0, 3.0, 4.0);
        v.cross_assign(Vector::new(1.0, 2.0, 3.0));
        assert!(v == Vector::new(1.0, -2.0, 1.0))
    }
}
