//! Minor operation
use std::ops::Index;

use crate::{Determinant, Mat2, Mat3, Matr4, Matrix, Submatrix};

/// The determinant of the submatrix
pub trait Minor<const DIM: usize, const SUB: usize>: Matrix<DIM> + Submatrix<DIM, SUB> {
    #[inline]
    fn minor(&self, row: usize, col: usize) -> f32
    where
        Self: Index<(usize, usize), Output = f32>,
        Self::SubmatrixOutput: Determinant + Matrix<SUB>,
        [(); SUB * SUB]:,
    {
        self.submatrix(row, col).determinant()
    }
}

impl Minor<3, 2> for Mat3 {}

impl Minor<4, 3> for Matr4 {}

// impl<'mat, 'submat> Minor<'mat, 'submat, 3, 2> for Mat3 {
//     fn minor(&'mat self, row: usize, col: usize) -> f32 {
//         self.submat(row, col).det()
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    mod mat3 {
        use super::{Determinant, Mat3, Matrix, Minor, Submatrix};
        #[test]
        fn calc_minor() {
            let a = Mat3::new([3.0, 5.0, 0.0, 2.0, -1.0, -7.0, 6.0, -1.0, 5.0]);
            let b = a.submatrix(1, 0);
            assert_eq!(b.determinant(), a.minor(1, 0))
        }
    }
}
