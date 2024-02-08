//! Vector type
use crate::{Pointlike, Tuple};
use std::{
    ops::{Add, AddAssign, DivAssign, Mul, Sub, SubAssign},
    simd::num::SimdFloat,
};
pub struct Vector(pub(crate) Tuple);
impl Vector {
    pub const ZERO: Self = Self(Tuple::ZERO);
    pub const X: Self = Self(Tuple::X);
    pub const Y: Self = Self(Tuple::Y);
    pub const Z: Self = Self(Tuple::Z);
    #[inline]
    pub fn magnitude(&self) -> f32 {
        self.0 .0.mul(self.0 .0).reduce_sum().sqrt()
    }
    #[inline]
    pub fn normalize(&mut self) {
        self.0.div_assign(self.magnitude())
    }
}
impl Pointlike for Vector {
    #[inline]
    fn new(x: f32, y: f32, z: f32) -> Self {
        Self(Tuple::new(x, y, z, 0.0_f32))
    }
    #[inline]
    fn is_vector(&self) -> bool {
        true
    }
}
impl Add for Vector {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0.add(rhs.0))
    }
}
impl Add<&Self> for Vector {
    type Output = Self;
    #[inline]
    fn add(self, rhs: &Self) -> Self::Output {
        Self(self.0 .0.add(rhs.0 .0).into())
    }
}
impl Add<Vector> for &Vector {
    type Output = Vector;
    #[inline]
    fn add(self, rhs: Vector) -> Self::Output {
        Vector(self.0 .0.add(rhs.0 .0).into())
    }
}
impl Add for &Vector {
    type Output = Vector;
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Vector(self.0 .0.add(rhs.0 .0).into())
    }
}
impl AddAssign for Vector {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.0.add_assign(rhs.0)
    }
}
impl AddAssign<&Self> for Vector {
    #[inline]
    fn add_assign(&mut self, rhs: &Self) {
        self.0 .0.add_assign(rhs.0 .0)
    }
}
impl Sub for Vector {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0.sub(rhs.0))
    }
}
impl Sub<&Self> for Vector {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: &Self) -> Self::Output {
        Self(self.0 .0.sub(rhs.0 .0).into())
    }
}
impl Sub<Vector> for &Vector {
    type Output = Vector;
    #[inline]
    fn sub(self, rhs: Vector) -> Self::Output {
        Vector(self.0 .0.sub(rhs.0 .0).into())
    }
}
impl Sub for &Vector {
    type Output = Vector;
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Vector(self.0 .0.sub(rhs.0 .0).into())
    }
}
impl SubAssign for Vector {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.0.sub_assign(rhs.0)
    }
}
impl SubAssign<&Self> for Vector {
    #[inline]
    fn sub_assign(&mut self, rhs: &Self) {
        self.0 .0.sub_assign(rhs.0 .0)
    }
}
impl PartialEq for Vector {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(other)
    }
}
impl PartialEq<Tuple> for Vector {
    #[inline]
    fn eq(&self, other: &Tuple) -> bool {
        self.0.eq(other)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn vector_constructor_w_eq_zero() {
        let v = Vector::new(4.0, -4.0, 3.0);
        assert!(v == Tuple::new(4.0, -4.0, 3.0, 0.0));
    }
    #[test]
    fn a_tuple_with_w_eq_zero_is_a_vector() {
        let a = Vector::new(4.3, -4.2, 3.1);
        assert!(a == Tuple::new(4.3, -4.2, 3.1, 0.0));
        assert!(!a.is_point());
        assert!(a.is_vector());
    }
    #[test]
    fn sub_two_vectors() {
        let v1 = vector!(3.0, 2.0, 1.0);
        let v2 = vector!(5.0, 6.0, 7.0);
        assert!((v1 - v2) == vector!(-2.0, -4.0, -6.0))
    }
    #[test]
    fn sub_vector_from_zero_vector() {
        let v = vector!(1.0, -2.0, 3.0);
        assert!((Vector::ZERO - v) == vector!(-1.0, 2.0, -3.0))
    }
    #[test]
    fn magnitude() {
        let v = Vector::X;
        assert!(v.magnitude() == 1.0);
        let v = Vector::Y;
        assert!(v.magnitude() == 1.0);
        let v = Vector::Z;
        assert!(v.magnitude() == 1.0);
        let v = vector!(1.0, 2.0, 3.0);
        assert!(v.magnitude() == 14.0_f32.sqrt());
        let v = vector!(-1.0, -2.0, -3.0);
        assert!(v.magnitude() == 14.0_f32.sqrt());
    }
    // TODO Normalize tests
}
