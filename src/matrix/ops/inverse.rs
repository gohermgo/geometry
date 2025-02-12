use std::ops::Index;

use crate::{Cofactor, Determinant, Mat3, Mat4, Matrix};

pub trait Inverse<const DIM: usize, const SUB: usize>: Matrix<DIM> {
    fn is_invertible(&self) -> bool
    where
        Self: Sized + Determinant,
    {
        self.determinant() != 0.0
    }
    fn inverse(&self) -> Option<Self>
    where
        Self: Sized + Determinant + Cofactor<DIM, SUB> + Index<(usize, usize), Output = f32>,
        Self::SubmatrixOutput: Matrix<SUB> + Determinant,
        [(); DIM * DIM]:,
        [(); SUB * SUB]:,
    {
        if self.is_invertible() {
            let determinant = self.determinant();
            let mut array = [0.0; DIM * DIM];
            for row in 0..DIM {
                for col in 0..DIM {
                    // Transpose by flipping indexing
                    array[row + (col * DIM)] = self.cofactor(row, col) / determinant;
                }
            }
            Some(Self::new(array))
        } else {
            None
        }
    }
}

impl Inverse<3, 2> for Mat3 {}

impl Inverse<4, 3> for Mat4 {}

#[cfg(test)]
mod tests {
    use super::*;
    mod mat4 {
        use crate::Vert4;

        use super::*;
        #[test]
        fn invertible() {
            let a = Mat4::new([
                6.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 6.0, 4.0, -9.0, 3.0, -7.0, 9.0, 1.0, 7.0, -6.0,
            ]);
            assert_eq!(a.determinant(), -2120.0);
            assert!(a.is_invertible())
        }
        #[test]
        fn non_invertible() {
            let a = Mat4::new([
                -4.0, 2.0, -2.0, -3.0, 9.0, 6.0, 2.0, 6.0, 0.0, -5.0, 1.0, -5.0, 0.0, 0.0, 0.0, 0.0,
            ]);
            assert_eq!(a.determinant(), 0.0);
            assert!(!a.is_invertible())
        }
        #[test]
        fn calculating_inverse_1() {
            let a = Mat4::new([
                -5.0, 2.0, 6.0, -8.0, 1.0, -5.0, 1.0, 8.0, 7.0, 7.0, -6.0, -7.0, 1.0, -3.0, 7.0,
                4.0,
            ]);
            // Assert determinants and cofactors
            assert_eq!(a.determinant(), 532.0);
            assert_eq!(a.cofactor(2, 3), -160.0);
            assert_eq!(a.cofactor(3, 2), 105.0);

            let i = a.inverse().unwrap();

            // Assert manual calculation gives same results
            assert_eq!(i[(3, 2)], -160.0 / 532.0);
            assert_eq!(i[(2, 3)], 105.0 / 532.0);

            // Assert row-by-row equality
            let rows = i.as_row_vectors();
            assert_eq!(rows[0], Vert4::new(0.21805, 0.45113, 0.24060, -0.04511));
            assert_eq!(rows[1], Vert4::new(-0.80827, -1.45677, -0.44361, 0.52068));
            assert_eq!(rows[2], Vert4::new(0.07895, -0.22368, -0.05263, 0.19737));
            assert_eq!(rows[3], Vert4::new(-0.52256, -0.81391, -0.30075, 0.30639));
        }
        #[test]
        fn calculating_inverse_2() {
            let a = Mat4::new([
                8.0, -5.0, 9.0, 2.0, 7.0, 5.0, 6.0, 1.0, -6.0, 0.0, 9.0, 6.0, -3.0, 0.0, -9.0, -4.0,
            ]);

            let i = a.inverse().unwrap();

            // Assert row-by-row equality
            let rows = i.as_row_vectors();
            assert_eq!(rows[0], Vert4::new(-0.15385, -0.15385, -0.28205, -0.53846));
            assert_eq!(rows[1], Vert4::new(-0.07692, 0.12308, 0.02564, 0.03077));
            assert_eq!(rows[2], Vert4::new(0.35897, 0.35897, 0.43590, 0.92308));
            assert_eq!(rows[3], Vert4::new(-0.69231, -0.69231, -0.76923, -1.92308));
        }
        #[test]
        fn calculating_inverse_3() {
            let m = Mat4::new([
                9.0, 3.0, 0.0, 9.0, -5.0, -2.0, -6.0, -3.0, -4.0, 9.0, 6.0, 4.0, -7.0, 6.0, 6.0,
                2.0,
            ]);

            let i = m.inverse().unwrap();

            // Assert row-by-row equality
            let rows = i.as_row_vectors();
            assert_eq!(rows[0], Vert4::new(-0.04074, -0.07778, 0.14444, -0.22222));
            assert_eq!(rows[1], Vert4::new(-0.07778, 0.03333, 0.36667, -0.33333));
            assert_eq!(rows[2], Vert4::new(-0.02901, -0.14630, -0.10926, 0.12963));
            assert_eq!(rows[3], Vert4::new(0.17778, 0.06667, -0.26667, 0.33333));
        }
        #[test]
        fn matrix_product_and_product_with_inverse_of_second_gives_original() {
            let m1 = Mat4::new([
                3.0, -9.0, 7.0, 3.0, 3.0, -8.0, 2.0, -9.0, -4.0, 4.0, 4.0, 1.0, -6.0, 5.0, -1.0,
                1.0,
            ]);
            let m2 = Mat4::new([
                8.0, 2.0, 2.0, 2.0, 3.0, -1.0, 7.0, 0.0, 7.0, 0.0, 5.0, 4.0, 6.0, -2.0, 0.0, 5.0,
            ]);
            let m1_m2_prod = &m1 * &m2;
            let i2 = m2.inverse().unwrap();
            let m1_m2_prod_i2_prod = m1_m2_prod * &i2;
            assert_eq!(m1_m2_prod_i2_prod, m1);
        }
    }
}
