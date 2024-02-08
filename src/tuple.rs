use crate::{Point, SortaEq, Vector};
use std::{
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    simd::{f32x4, Simd},
};
const SIMD_ZERO: Simd<f32, 4> = Simd::from_array([0.0_f32, 0.0_f32, 0.0_f32, 0.0_f32]);
const SIMD_X: Simd<f32, 4> = Simd::from_array([1.0_f32, 0.0_f32, 0.0_f32, 0.0_f32]);
const SIMD_Y: Simd<f32, 4> = Simd::from_array([0.0_f32, 1.0_f32, 0.0_f32, 0.0_f32]);
const SIMD_Z: Simd<f32, 4> = Simd::from_array([0.0_f32, 0.0_f32, 1.0_f32, 0.0_f32]);
const SIMD_W: Simd<f32, 4> = Simd::from_array([0.0_f32, 0.0_f32, 0.0_f32, 1.0_f32]);
#[derive(Debug)]
pub struct Tuple(pub(crate) f32x4);
impl Tuple {
    pub const ZERO: Self = Self(SIMD_ZERO);
    pub const X: Self = Self(SIMD_X);
    pub const Y: Self = Self(SIMD_Y);
    pub const Z: Self = Self(SIMD_Z);
    pub const W: Self = Self(SIMD_W);
    #[inline]
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self(f32x4::from_slice(&[x, y, z, w]))
    }
}
// From impls---------
impl From<f32x4> for Tuple {
    #[inline]
    fn from(value: f32x4) -> Self {
        Self(value)
    }
}
impl From<&f32x4> for Tuple {
    #[inline]
    fn from(value: &f32x4) -> Self {
        Self(value.to_owned())
    }
}
// Add impls---------
impl Add for Tuple {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        self.0.add(rhs.0).into()
    }
}
impl AddAssign for Tuple {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.0.add_assign(rhs.0)
    }
}
impl Add<&Self> for Tuple {
    type Output = Self;
    #[inline]
    fn add(self, rhs: &Self) -> Self::Output {
        self.0.add(rhs.0).into()
    }
}
impl AddAssign<&Self> for Tuple {
    #[inline]
    fn add_assign(&mut self, rhs: &Self) {
        self.0.add_assign(rhs.0)
    }
}
impl Add for &Tuple {
    type Output = Tuple;
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        self.0.add(rhs.0).into()
    }
}
impl Add<&Self> for &Tuple {
    type Output = Tuple;
    #[inline]
    fn add(self, rhs: &Self) -> Self::Output {
        self.0.add(rhs.0).into()
    }
}
impl Sub for Tuple {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        self.0.sub(rhs.0).into()
    }
}
impl SubAssign for Tuple {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.0.sub_assign(rhs.0)
    }
}
impl Sub<&Self> for Tuple {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: &Self) -> Self::Output {
        self.0.sub(rhs.0).into()
    }
}
impl SubAssign<&Self> for Tuple {
    #[inline]
    fn sub_assign(&mut self, rhs: &Self) {
        self.0.sub_assign(rhs.0)
    }
}
impl Sub<&Self> for &Tuple {
    type Output = Tuple;
    #[inline]
    fn sub(self, rhs: &Self) -> Self::Output {
        self.0.sub(rhs.0).into()
    }
}
impl Neg for Tuple {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self::Output {
        self.0.neg().into()
    }
}
impl Neg for &Tuple {
    type Output = Tuple;
    #[inline]
    fn neg(self) -> Self::Output {
        self.0.neg().into()
    }
}
impl Mul<f32> for Tuple {
    type Output = Tuple;
    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        self.0.mul(simd_4!(rhs)).into()
    }
}
impl MulAssign for Tuple {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        self.0.mul_assign(rhs.0)
    }
}
impl Mul<Tuple> for &Tuple {
    type Output = Tuple;
    #[inline]
    fn mul(self, rhs: Tuple) -> Self::Output {
        self.0.mul(rhs.0).into()
    }
}
impl Mul<&Self> for Tuple {
    type Output = Tuple;
    #[inline]
    fn mul(self, rhs: &Self) -> Self::Output {
        self.0.mul(rhs.0).into()
    }
}
impl MulAssign<&Self> for Tuple {
    #[inline]
    fn mul_assign(&mut self, rhs: &Self) {
        self.0.mul_assign(rhs.0)
    }
}
impl Mul<Self> for &Tuple {
    type Output = Tuple;
    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        self.0.mul(rhs.0).into()
    }
}
impl Div<f32> for Tuple {
    type Output = Self;
    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        self.0.div(simd_4!(rhs)).into()
    }
}
impl DivAssign<f32> for Tuple {
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        self.0.div_assign(simd_4!(rhs))
    }
}
impl Div<f32> for &Tuple {
    type Output = Tuple;
    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        self.0.div(simd_4!(rhs)).into()
    }
}
impl Div<&f32> for Tuple {
    type Output = Self;
    #[inline]
    fn div(self, rhs: &f32) -> Self::Output {
        self.0.div(simd_4!(*rhs)).into()
    }
}
impl DivAssign<&f32> for Tuple {
    #[inline]
    fn div_assign(&mut self, rhs: &f32) {
        self.0.div_assign(simd_4!(*rhs))
    }
}
impl Div<&f32> for &Tuple {
    type Output = Tuple;
    #[inline]
    fn div(self, rhs: &f32) -> Self::Output {
        self.0.div(simd_4!(*rhs)).into()
    }
}
impl PartialEq for Tuple {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0.ehh_maybe(&other.0)
    }
}
impl PartialEq<Vector> for Tuple {
    #[inline]
    fn eq(&self, other: &Vector) -> bool {
        self.0.ehh_maybe(&other.0 .0)
    }
}
impl PartialEq<Point> for Tuple {
    #[inline]
    fn eq(&self, other: &Point) -> bool {
        self.0.ehh_maybe(&other.0 .0)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_tuples() {
        let a1 = tuple!(3.0, -2.0, 5.0, 1.0);
        let a2 = tuple!(-2.0, 3.0, 1.0, 0.0);
        assert!((a1 + a2) == tuple!(1.0, 1.0, 6.0, 1.0));
    }
    #[test]
    fn negating_tuple() {
        let a = tuple!(1.0, -2.0, 3.0, -4.0);
        assert!(-a == tuple!(-1.0, 2.0, -3.0, 4.0))
    }
    #[test]
    fn multiplying_tuple_by_scalar() {
        let a = tuple!(1.0, -2.0, 3.0, -4.0);
        assert!((a * 3.5) == tuple!(3.5, -7.0, 10.5, -14.0))
    }
    #[test]
    fn multiplying_tuple_by_fraction() {
        let a = tuple!(1.0, -2.0, 3.0, -4.0);
        assert!((a * 0.5) == tuple!(0.5, -1.0, 1.5, -2.0))
    }
    #[test]
    fn dividing_tuple_by_scalar() {
        let a = tuple!(1.0, -2.0, 3.0, -4.0);
        assert!((a / 2.0) == tuple!(0.5, -1.0, 1.5, -2.0))
    }
}
