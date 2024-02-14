//! Submatrix-ing operation
use std::ops::Index;

use crate::{Mat2, Mat3, Mat4, Matrix};

pub trait Submatrix<const DIM: usize, const SUB: usize>: Matrix<DIM> {
    type SubmatrixOutput;
    #[inline]
    fn submatrix(&self, omitted_row: usize, omitted_col: usize) -> Self::SubmatrixOutput
    where
        Self: Index<(usize, usize), Output = f32>,
        Self::SubmatrixOutput: Matrix<SUB>,
        [(); SUB * SUB]:,
    {
        // Make square array
        let mut array = [0.0; SUB * SUB];

        // Make index counter, so as not to overindex, and write correctly
        let mut array_index = 0;

        // Iterate over the `Self` matrix
        for row in (0..DIM).filter(|row| row != &omitted_row) {
            for col in (0..DIM).filter(|col| col != &omitted_col) {
                array[array_index] = self[(row, col)];
                array_index += 1;
            }
        }
        // for row in 0..DIM {
        //     match row {
        //         _ if row == omitted_row => continue,
        //         _ => {
        //             for col in 0..DIM {
        //                 match col {
        //                     _ if col == omitted_col => continue,
        //                     _ => {
        //                         array[array_index] = self[(row, col)];
        //                         array_index += 1;
        //                     }
        //                 }
        //             }
        //         }
        //     }
        // }

        Self::SubmatrixOutput::new(array)
    }
}

impl Submatrix<3, 2> for Mat3 {
    type SubmatrixOutput = Mat2;
}

impl Submatrix<4, 3> for Mat4 {
    type SubmatrixOutput = Mat3;
    // #[inline]
    // fn submatrix(&self, omitted_row: usize, omitted_col: usize) -> Self::SubmatrixOutput {
    //     let mut array = [0.0; 9];
    //     let mut i = 0;
    //     for row in 0..4 {
    //         if row == omitted_row {
    //             continue;
    //         }
    //         for col in 0..4 {
    //             if col == omitted_col {
    //                 continue;
    //             }
    //             array[i] = self[(row, col)];
    //             i += 1;
    //         }
    //     }
    //     Mat3::new(array)
    // }
}

#[cfg(test)]
mod tests {
    use super::*;
    mod mat3 {
        use super::{Mat2, Mat3, Matrix, Submatrix};
        #[test]
        fn submat() {
            let a = Mat3::new([1.0, 5.0, 0.0, -3.0, 2.0, 7.0, 0.0, 6.0, -3.0]);
            assert_eq!(a.submatrix(0, 2), Mat2::new([-3.0, 2.0, 0.0, 6.0]))
        }
    }
    mod mat4 {
        use super::{Mat3, Mat4, Matrix, Submatrix};
        #[test]
        fn submat() {
            let a = Mat4::new([
                -6.0, 1.0, 1.0, 6.0, -8.0, 5.0, 8.0, 6.0, -1.0, 0.0, 8.0, 2.0, -7.0, 1.0, -1.0, 1.0,
            ]);
            assert_eq!(
                a.submatrix(2, 1),
                Mat3::new([-6.0, 1.0, 6.0, -8.0, 8.0, 6.0, -7.0, -1.0, 1.0])
            )
        }
    }
}
