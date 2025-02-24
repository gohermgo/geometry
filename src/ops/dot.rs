use core::ops::Mul;
use core::simd::num::SimdFloat;
use core::simd::Simd;

use crate::Vert4;

pub trait Dot<Rhs: ?Sized = Self> {
    type Output;
    fn dot(self, rhs: Rhs) -> Self::Output;
}
impl Dot for Vert4 {
    type Output = f32;
    fn dot(self, rhs @ Vert4(rhs_simd): Self) -> Self::Output {
        debug_assert!(self.is_vector(), "Dot: Self is not a vector!");
        debug_assert!(rhs.is_vector(), "Dot: Rhs is not a vector!");
        Simd::mul(self.0, rhs_simd).reduce_sum()
    }
}
impl Dot<&Vert4> for Vert4 {
    type Output = f32;
    fn dot(self, rhs @ Vert4(rhs_simd): &Vert4) -> Self::Output {
        debug_assert!(self.is_vector(), "Dot: Self is not a vector!");
        debug_assert!(rhs.is_vector(), "Dot: Rhs is not a vector!");
        Simd::mul(self.0, rhs_simd).reduce_sum()
    }
}
impl Dot for &Vert4 {
    type Output = f32;
    fn dot(self, rhs @ Vert4(rhs_simd): Self) -> Self::Output {
        debug_assert!(self.is_vector(), "Dot: Self is not a vector!");
        debug_assert!(rhs.is_vector(), "Dot: Rhs is not a vector!");
        Simd::mul(self.0, rhs_simd).reduce_sum()
    }
}
impl Dot<Vert4> for &Vert4 {
    type Output = f32;
    fn dot(self, rhs @ Vert4(rhs_simd): Vert4) -> Self::Output {
        debug_assert!(self.is_vector(), "Dot: Self is not a vector!");
        debug_assert!(rhs.is_vector(), "Dot: Rhs is not a vector!");
        Simd::mul(self.0, rhs_simd).reduce_sum()
    }
}
// impl Dot<Vector> for Vector {
//     type Output = f32;
//     #[inline]
//     fn dot(self, rhs: Vector) -> Self::Output {
//         (self.0 .0 * rhs.0 .0).reduce_sum()
//     }
// }
// impl Dot<&Vector> for Vector {
//     type Output = f32;
//     #[inline]
//     fn dot(self, rhs: &Vector) -> Self::Output {
//         (self.0 .0 * rhs.0 .0).reduce_sum()
//     }
// }
// impl Dot<Vector> for &Vector {
//     type Output = f32;
//     #[inline]
//     fn dot(self, rhs: Vector) -> Self::Output {
//         (self.0 .0 * rhs.0 .0).reduce_sum()
//     }
// }
// impl Dot<&Vector> for &Vector {
//     type Output = f32;
//     #[inline]
//     fn dot(self, rhs: &Vector) -> Self::Output {
//         (self.0 .0 * rhs.0 .0).reduce_sum()
//     }
// }
