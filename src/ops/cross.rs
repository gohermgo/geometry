use crate::Vert4;

use std::simd::{f32x4, simd_swizzle};

pub(crate) const CROSS_SWIZZLE_0: [usize; 4] = [1, 2, 0, 3];
pub(crate) const CROSS_SWIZZLE_1: [usize; 4] = [2, 0, 1, 3];

pub trait Cross<Rhs: ?Sized> {
    type Output;
    fn cross(self, rhs: Rhs) -> Self::Output;
}
impl Cross<Vert4> for Vert4 {
    type Output = Vert4;
    #[inline]
    fn cross(self, rhs: Vert4) -> Self::Output {
        let temp_0: f32x4 = simd_swizzle!(self.0, CROSS_SWIZZLE_0);
        let temp_1: f32x4 = simd_swizzle!(rhs.0, CROSS_SWIZZLE_1);
        let prod_0: f32x4 = temp_0 * temp_1;

        let temp_0: f32x4 = simd_swizzle!(self.0, CROSS_SWIZZLE_1);
        let temp_1: f32x4 = simd_swizzle!(rhs.0, CROSS_SWIZZLE_0);
        let prod_1: f32x4 = temp_0 * temp_1;
        Self((prod_0 - prod_1).into())
    }
}
impl Cross<Vert4> for &Vert4 {
    type Output = Vert4;
    #[inline]
    fn cross(self, rhs: Vert4) -> Self::Output {
        let temp_0: f32x4 = simd_swizzle!(self.0, CROSS_SWIZZLE_0);
        let temp_1: f32x4 = simd_swizzle!(rhs.0, CROSS_SWIZZLE_1);
        let prod_0: f32x4 = temp_0 * temp_1;

        let temp_0: f32x4 = simd_swizzle!(self.0, CROSS_SWIZZLE_1);
        let temp_1: f32x4 = simd_swizzle!(rhs.0, CROSS_SWIZZLE_0);
        let prod_1: f32x4 = temp_0 * temp_1;
        Vert4((prod_0 - prod_1).into())
    }
}
impl Cross<&Vert4> for Vert4 {
    type Output = Vert4;
    #[inline]
    fn cross(self, rhs: &Vert4) -> Self::Output {
        let temp_0: f32x4 = simd_swizzle!(self.0, CROSS_SWIZZLE_0);
        let temp_1: f32x4 = simd_swizzle!(rhs.0, CROSS_SWIZZLE_1);
        let prod_0: f32x4 = temp_0 * temp_1;

        let temp_0: f32x4 = simd_swizzle!(self.0, CROSS_SWIZZLE_1);
        let temp_1: f32x4 = simd_swizzle!(rhs.0, CROSS_SWIZZLE_0);
        let prod_1: f32x4 = temp_0 * temp_1;
        Self((prod_0 - prod_1).into())
    }
}
impl Cross<&Vert4> for &Vert4 {
    type Output = Vert4;
    #[inline]
    fn cross(self, rhs: &Vert4) -> Self::Output {
        let temp_0: f32x4 = simd_swizzle!(self.0, CROSS_SWIZZLE_0);
        let temp_1: f32x4 = simd_swizzle!(rhs.0, CROSS_SWIZZLE_1);
        let prod_0: f32x4 = temp_0 * temp_1;

        let temp_0: f32x4 = simd_swizzle!(self.0, CROSS_SWIZZLE_1);
        let temp_1: f32x4 = simd_swizzle!(rhs.0, CROSS_SWIZZLE_0);
        let prod_1: f32x4 = temp_0 * temp_1;
        Vert4((prod_0 - prod_1).into())
    }
}
// pub trait CrossAssign<Rhs: ?Sized> {
//     fn cross_assign(&mut self, rhs: Rhs);
// }
// impl CrossAssign<Vert4> for Vert4 {
//     #[inline]
//     fn cross_assign(&mut self, rhs: Vert4) {
//         let temp: f32x4 = simd_swizzle!(rhs.0, CROSS_SWIZZLE_1);
//         self.0 = temp * simd_swizzle!(self.0, CROSS_SWIZZLE_0);
//         let temp: f32x4 = simd_swizzle!(rhs.0, CROSS_SWIZZLE_0);
//         self.0 -= temp * simd_swizzle!(self.0, CROSS_SWIZZLE_1);
//     }
// }
// impl CrossAssign<&Vert4> for Vert4 {
//     #[inline]
//     fn cross_assign(&mut self, rhs: &Vert4) {
//         let temp: f32x4 = simd_swizzle!(rhs.0, CROSS_SWIZZLE_1);
//         self.0 = temp * simd_swizzle!(self.0, CROSS_SWIZZLE_0);
//         let temp: f32x4 = simd_swizzle!(rhs.0, CROSS_SWIZZLE_0);
//         self.0 -= temp * simd_swizzle!(self.0, CROSS_SWIZZLE_1);
//     }
// }
// impl CrossAssign<Vector> for Vector {
//     #[inline]
//     fn cross_assign(&mut self, rhs: Vector) {
//         self.0.cross_assign(rhs.0);
//     }
// }
// impl CrossAssign<&Vector> for Vector {
//     #[inline]
//     fn cross_assign(&mut self, rhs: &Vector) {
//         self.0.cross_assign(&rhs.0);
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn the_cross_product_of_two_vectors() {
        let a = Vert4::vector(1., 2., 3.);
        let b = Vert4::vector(2., 3., 4.);

        let v1 = Cross::cross(&a, &b);
        assert_eq!(v1, Vert4::vector(-1., 2., -1.));

        let v2 = Cross::cross(&b, &a);
        assert_eq!(v2, Vert4::vector(1., -2., 1.));
    }
}
