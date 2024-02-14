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
        use super::*;
        #[test]
        fn is_invertible() {
            let a = Mat4::new([
                6.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 6.0, 4.0, -9.0, 3.0, -7.0, 9.0, 1.0, 7.0, -6.0,
            ]);
            assert_eq!(a.determinant(), -2120.0);
            assert!(a.is_invertible())
        }
        #[test]
        fn isnt_invertible() {
            let a = Mat4::new([
                -4.0, 2.0, -2.0, -3.0, 9.0, 6.0, 2.0, 6.0, 0.0, -5.0, 1.0, -5.0, 0.0, 0.0, 0.0, 0.0,
            ]);
            assert_eq!(a.determinant(), 0.0);
            assert!(!a.is_invertible())
        }
    }
}
