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
#[cfg(test)]
mod tests {
    use super::*;
    use crate::PointType;
    #[test]
    fn unit_1() {
        let v = Vector::X;
        assert!(v.mag() == 1.0);
    }
    #[test]
    fn unit_2() {
        let v = Vector::Y;
        assert!(v.mag() == 1.0);
    }
    #[test]
    fn unit_3() {
        let v = Vector::Z;
        assert!(v.mag() == 1.0);
    }
    #[test]
    fn unit_4() {
        let v = Vector::new(1.0, 2.0, 3.0);
        assert!(v.mag() == 14.0_f32.sqrt());
    }
    #[test]
    fn unit_5() {
        let v = Vector::new(-1.0, -2.0, -3.0);
        assert!(v.mag() == 14.0_f32.sqrt());
    }
}
