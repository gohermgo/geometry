use crate::{ops::Mag, Vector, Vert4};
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
impl Norm for Vert4 {
    type Output = Vert4;
    #[inline]
    fn norm(&self) -> Self::Output {
        Vert4(self.0 / f32x4::from_array([self.0.mag(); 4]))
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
#[cfg(test)]
mod tests {
    use crate::cmp::SortaEq;
    use crate::PointType;

    use super::*;
    #[test]
    fn unit_1() {
        let v = Vector::new(4.0, 0.0, 0.0);
        assert!(v.norm() == Vector::new(1.0, 0.0, 0.0));
    }
    #[test]
    fn unit_2() {
        let v = Vector::new(1.0, 2.0, 3.0);
        println!("NORM {:?}", v.norm().0);
        assert!(v.norm() == Vector::new(0.26726, 0.53452, 0.80178))
    }
    #[test]
    fn unit_3() {
        let v = Vector::new(1.0, 2.0, 3.0);
        let norm = v.norm();
        assert!(norm.mag().ehh_maybe(&1.0));
    }
}
