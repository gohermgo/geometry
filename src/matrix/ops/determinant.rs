use crate::{Cofactor, Mat2, Mat3, Mat4};

pub trait Determinant {
    fn determinant(&self) -> f32;
}

impl Determinant for Mat2 {
    #[inline]
    fn determinant(&self) -> f32 {
        (self[(0, 0)] * self[(1, 1)]) - (self[(0, 1)] * self[(1, 0)])
    }
}

impl Determinant for Mat3 {
    #[inline]
    fn determinant(&self) -> f32 {
        let mut tally = 0.0;
        let row = 0;
        for col in 0..3 {
            tally += self[(row, col)] * self.cofactor(row, col);
        }
        tally
    }
}
impl Determinant for Mat4 {
    #[inline]
    fn determinant(&self) -> f32 {
        let mut tally = 0.0;
        let row = 0;
        for col in 0..4 {
            tally += self[(row, col)] * self.cofactor(row, col);
        }
        tally
    }
}

#[cfg(test)]
use crate::Matrix;

#[cfg(test)]
mod tests {
    use super::*;
    mod mat2 {
        use super::{Determinant, Mat2, Matrix};
        #[test]
        fn calc_det() {
            let a = Mat2::new([1.0, 5.0, -3.0, 2.0]);
            assert_eq!(a.determinant(), 17.0)
        }
    }
    mod mat3 {
        use super::{Cofactor, Determinant, Mat3, Matrix};
        #[test]
        fn calc_det() {
            let a = Mat3::new([1.0, 2.0, 6.0, -5.0, 8.0, -4.0, 2.0, 6.0, 4.0]);
            assert_eq!(a.cofactor(0, 0), 56.0);
            assert_eq!(a.cofactor(0, 1), 12.0);
            assert_eq!(a.cofactor(0, 2), -46.0);
            assert_eq!(a.determinant(), -196.0);
        }
    }

    mod mat4 {
        use super::{Cofactor, Determinant, Mat4, Matrix};
        #[test]
        fn calc_det() {
            let a = Mat4::new([
                -2.0, -8.0, 3.0, 5.0, -3.0, 1.0, 7.0, 3.0, 1.0, 2.0, -9.0, 6.0, -6.0, 7.0, 7.0,
                -9.0,
            ]);
            assert_eq!(a.cofactor(0, 0), 690.0);
            assert_eq!(a.cofactor(0, 1), 447.0);
            assert_eq!(a.cofactor(0, 2), 210.0);
            assert_eq!(a.cofactor(0, 3), 51.0);
            assert_eq!(a.determinant(), -4071.0);
        }
    }
}
