use crate::{Point, Vector};
use cmp::SortaEq;
use std::{
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    simd::{f32x2, f32x4, Simd},
};
pub trait Vertex {}
const SIMD_2_ZERO: Simd<f32, 2> = Simd::from_array([0.0_f32, 0.0_f32]);
const SIMD_2_X: Simd<f32, 2> = Simd::from_array([1.0_f32, 0.0_f32]);
const SIMD_2_Y: Simd<f32, 2> = Simd::from_array([0.0_f32, 1.0_f32]);
#[derive(Debug)]
pub struct Vert2(pub(crate) f32x2);
impl Vertex for Vert2 {}
impl Vert2 {
    pub const ZERO: Self = Self(SIMD_2_ZERO);
    pub const X: Self = Self(SIMD_2_X);
    pub const Y: Self = Self(SIMD_2_Y);
    #[inline]
    pub const fn new(x: f32, y: f32) -> Self {
        Self(f32x2::from_array([x, y]))
    }
}
impl From<f32x2> for Vert2 {
    #[inline]
    fn from(value: f32x2) -> Self {
        Self(value)
    }
}
impl From<&f32x2> for Vert2 {
    #[inline]
    fn from(value: &f32x2) -> Self {
        Self(*value)
    }
}
const SIMD_3_ZERO: [f32; 3] = [0.0_f32, 0.0_f32, 0.0_f32];
const SIMD_3_X: [f32; 3] = [1.0_f32, 0.0_f32, 0.0_f32];
const SIMD_3_Y: [f32; 3] = [0.0_f32, 1.0_f32, 0.0_f32];
const SIMD_3_Z: [f32; 3] = [0.0_f32, 0.0_f32, 1.0_f32];
#[derive(Debug)]
pub struct Vert3(pub(crate) [f32; 3]);
impl Vertex for Vert3 {}
impl Vert3 {
    pub const ZERO: Self = Self(SIMD_3_ZERO);
    pub const X: Self = Self(SIMD_3_X);
    pub const Y: Self = Self(SIMD_3_Y);
    pub const Z: Self = Self(SIMD_3_Z);
    #[inline]
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self([x, y, z])
    }
}
impl From<[f32; 3]> for Vert3 {
    #[inline]
    fn from(value: [f32; 3]) -> Self {
        Self(value)
    }
}
impl From<&[f32; 3]> for Vert3 {
    #[inline]
    fn from(value: &[f32; 3]) -> Self {
        Self(*value)
    }
}
const SIMD_4_ZERO: Simd<f32, 4> = Simd::from_array([0.0_f32, 0.0_f32, 0.0_f32, 0.0_f32]);
const SIMD_4_X: Simd<f32, 4> = Simd::from_array([1.0_f32, 0.0_f32, 0.0_f32, 0.0_f32]);
const SIMD_4_Y: Simd<f32, 4> = Simd::from_array([0.0_f32, 1.0_f32, 0.0_f32, 0.0_f32]);
const SIMD_4_Z: Simd<f32, 4> = Simd::from_array([0.0_f32, 0.0_f32, 1.0_f32, 0.0_f32]);
const SIMD_4_W: Simd<f32, 4> = Simd::from_array([0.0_f32, 0.0_f32, 0.0_f32, 1.0_f32]);
#[derive(Debug)]
pub struct Vert4(pub(crate) f32x4);
impl Vertex for Vert4 {}
impl Vert4 {
    pub const ZERO: Self = Self(SIMD_4_ZERO);
    pub const X: Self = Self(SIMD_4_X);
    pub const Y: Self = Self(SIMD_4_Y);
    pub const Z: Self = Self(SIMD_4_Z);
    pub const W: Self = Self(SIMD_4_W);
    #[inline]
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self(f32x4::from_array([x, y, z, w]))
    }
    #[inline]
    pub fn x(&self) -> f32 {
        self.0[0]
    }
    #[inline]
    pub fn y(&self) -> f32 {
        self.0[1]
    }
}
// From impls---------
impl From<f32x4> for Vert4 {
    #[inline]
    fn from(value: f32x4) -> Self {
        Self(value)
    }
}
impl From<&f32x4> for Vert4 {
    #[inline]
    fn from(value: &f32x4) -> Self {
        Self(value.to_owned())
    }
}
macro_rules! imp {
    (Add, Self = $t:ty $(, Rhs = $rhs:ty)?$(, Output = $o:ty)?$(, Ex = $e:expr)?) => {
        add_impl!(Self = $t$(, Rhs = $rhs)?$(, Output = $o)?$(, Ex = $e)?);
    };
    (AddAssign, Self = $t:ty$(, Rhs = $rhs:ty)?, Ex = $e:expr) => {
        add_assign_impl!(Self = $t$(, Rhs = $rhs)?, Ex = $e);
    };
}
macro_rules! add_assign_impl {
    (Self = $t:ty, Rhs = $rhs:ty, Ex = $e:expr) => {
        impl AddAssign<$rhs> for $t {
            #[inline]
            fn add_assign(&mut self, rhs: $rhs) {
                $e
            }
        }
    };
    (Self = $t:ty, Ex = $e:expr) => {
        impl AddAssign for $t {
            #[inline]
            fn add_assign(&mut self, rhs: $t) {
                $e
            }
        }
    };
}
// Add impls---------
macro_rules! add_impl {
    (Self = $t:ty, Ex = $e:expr) => {
        add_impl!(Self = $t, Rhs = $t, Output = $t, Ex = $e);
    };
    (Self = $t:ty) => {
        add_impl!(Self = $t, Rhs = $t, Output = $t);
    };
    (Self = $t:ty, Rhs = $rhs:ty, Ex = $e:expr) => {
        add_impl!(Self = $t, Rhs = $rhs, Output = $t, Ex = $e);
    };
    (Self = $t:ty, Rhs = $rhs:ty) => {
        add_impl!(Self = $t, Rhs = $rhs, Output = $t);
    };
    (Self = $t:ty, Rhs = $rhs:ty, Output = $o:ty) => {
        impl Add<$rhs> for $t {
            type Output = $o;
            #[inline]
            fn add(self, rhs: $rhs) -> Self::Output {
                self.0.add(rhs.0).into()
            }
        }
    };
    (Self = $t:ty, Rhs = $rhs:ty, Output = $o:ty, Ex = $e:expr) => {
        impl Add<$rhs> for $t {
            type Output = $o;
            #[inline]
            fn add(self, rhs: $rhs) -> Self::Output {
                self.0.add(rhs.0).into()
            }
        }
    };
}
// imp!(Add, Self = Vert4);
impl Add for Vert4 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        self.0.add(rhs.0).into()
    }
}
// imp!(AddAssign, Self = Vert4, Ex = self.0.add_assign(rhs.0));
impl AddAssign for Vert4 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.0.add_assign(rhs.0)
    }
}
// add_impl!(Self = Vert4, Rhs = &Vert4);
impl Add<&Self> for Vert4 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: &Self) -> Self::Output {
        self.0.add(rhs.0).into()
    }
}
impl AddAssign<&Self> for Vert4 {
    #[inline]
    fn add_assign(&mut self, rhs: &Self) {
        self.0.add_assign(rhs.0)
    }
}
impl Add for &Vert4 {
    type Output = Vert4;
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        self.0.add(rhs.0).into()
    }
}
impl Add<&Self> for &Vert4 {
    type Output = Vert4;
    #[inline]
    fn add(self, rhs: &Self) -> Self::Output {
        self.0.add(rhs.0).into()
    }
}
impl Sub for Vert4 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        self.0.sub(rhs.0).into()
    }
}
impl SubAssign for Vert4 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.0.sub_assign(rhs.0)
    }
}
impl Sub<&Self> for Vert4 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: &Self) -> Self::Output {
        self.0.sub(rhs.0).into()
    }
}
impl SubAssign<&Self> for Vert4 {
    #[inline]
    fn sub_assign(&mut self, rhs: &Self) {
        self.0.sub_assign(rhs.0)
    }
}
impl Sub<&Self> for &Vert4 {
    type Output = Vert4;
    #[inline]
    fn sub(self, rhs: &Self) -> Self::Output {
        self.0.sub(rhs.0).into()
    }
}
impl Neg for Vert4 {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self::Output {
        self.0.neg().into()
    }
}
impl Neg for &Vert4 {
    type Output = Vert4;
    #[inline]
    fn neg(self) -> Self::Output {
        self.0.neg().into()
    }
}
impl Mul<f32> for Vert4 {
    type Output = Vert4;
    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        self.0.mul(simd_4!(rhs)).into()
    }
}
impl MulAssign for Vert4 {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        self.0.mul_assign(rhs.0)
    }
}
impl Mul<Vert4> for &Vert4 {
    type Output = Vert4;
    #[inline]
    fn mul(self, rhs: Vert4) -> Self::Output {
        self.0.mul(rhs.0).into()
    }
}
impl Mul<&Self> for Vert4 {
    type Output = Vert4;
    #[inline]
    fn mul(self, rhs: &Self) -> Self::Output {
        self.0.mul(rhs.0).into()
    }
}
impl MulAssign<&Self> for Vert4 {
    #[inline]
    fn mul_assign(&mut self, rhs: &Self) {
        self.0.mul_assign(rhs.0)
    }
}
impl Mul<Self> for &Vert4 {
    type Output = Vert4;
    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        self.0.mul(rhs.0).into()
    }
}
impl Div<f32> for Vert4 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        self.0.div(simd_4!(rhs)).into()
    }
}
impl DivAssign<f32> for Vert4 {
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        self.0.div_assign(simd_4!(rhs))
    }
}
impl Div<f32> for &Vert4 {
    type Output = Vert4;
    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        self.0.div(simd_4!(rhs)).into()
    }
}
impl Div<&f32> for Vert4 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: &f32) -> Self::Output {
        self.0.div(simd_4!(*rhs)).into()
    }
}
impl DivAssign<&f32> for Vert4 {
    #[inline]
    fn div_assign(&mut self, rhs: &f32) {
        self.0.div_assign(simd_4!(*rhs))
    }
}
impl Div<&f32> for &Vert4 {
    type Output = Vert4;
    #[inline]
    fn div(self, rhs: &f32) -> Self::Output {
        self.0.div(simd_4!(*rhs)).into()
    }
}
impl PartialEq for Vert4 {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0.ehh_maybe(&other.0)
    }
}
impl PartialEq<Vector> for Vert4 {
    #[inline]
    fn eq(&self, other: &Vector) -> bool {
        self.0.ehh_maybe(&other.0 .0)
    }
}
impl PartialEq<Point> for Vert4 {
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
