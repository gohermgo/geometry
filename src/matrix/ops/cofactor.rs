use crate::{Determinant, Mat3, Mat4, Matrix, Minor};
use std::ops::Index;

pub trait Cofactor<const DIM: usize, const SUB: usize>: Matrix<DIM> + Minor<DIM, SUB> {
    #[inline]
    fn cofactor(&self, row: usize, col: usize) -> f32
    where
        // The implementor can be indexed by tuples
        Self: Index<(usize, usize), Output = f32>,
        // The implementors resultant submatrix implements determinant
        Self::SubmatrixOutput: Determinant + Matrix<SUB>,
        // Bound allowing SUB * SUB to be used
        [(); SUB * SUB]:,
    {
        if (row + col) % 2 != 0 {
            -self.minor(row, col)
        } else {
            self.minor(row, col)
        }
    }
}

impl Cofactor<3, 2> for Mat3 {}

impl Cofactor<4, 3> for Mat4 {}

#[cfg(test)]
mod tests {
    use super::*;
    mod mat3 {
        use super::*;
        #[test]
        fn calc_cofactor() {
            let a = Mat3::new([3.0, 5.0, 0.0, 2.0, -1.0, -7.0, 6.0, -1.0, 5.0]);

            assert_eq!(a.minor(0, 0), -12.0);
            assert_eq!(a.cofactor(0, 0), -12.0);

            assert_eq!(a.minor(1, 0), 25.0);
            assert_eq!(a.cofactor(1, 0), -25.0);
        }
    }
}
