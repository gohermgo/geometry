//! Submatrix-ing operation
use std::ops::Index;

use crate::{
    matrix::{AsArray, FromArray},
    Matr2, Matr3, Matr4, Matrix,
};
#[const_trait]
pub trait ConstIndex<Idx: ?Sized> {
    /// The returned type after indexing.
    type Output: ?Sized;

    /// Performs the indexing (`container[index]`) operation.
    ///
    /// # Panics
    ///
    /// May panic if the index is out of bounds.
    #[track_caller]
    fn const_index(&self, index: Idx) -> &Self::Output;
}
impl<T, const N: usize> const ConstIndex<usize> for [T; N] {
    type Output = T;
    #[inline]
    fn const_index(&self, index: usize) -> &Self::Output {
        &self[index]
    }
}
#[const_trait]
pub trait ArraySubmatrix<const N: usize, const NS: usize> {
    type Output;
    fn array_submatrix(&self, omitted_row: usize, omitted_col: usize) -> Self::Output;
}
#[inline]
pub const fn submatrix_3(in_matrix: &Matr3, omitted_row: usize, omitted_col: usize) -> Matr2 {
    // Make square array
    let mut out_array = [0f32; 4];

    // Make index counter, so as not to overindex, and write correctly
    let mut array_index = 0;

    // Iterate over the `Self` matrix
    let mut row = 0;
    'rows: loop {
        if row >= 3 {
            break 'rows;
        }
        if row == omitted_row {
            row += 1;
            continue 'rows;
        }
        let mut col = 0;
        'cols: loop {
            if col >= 3 {
                break 'cols;
            }
            if col == omitted_col {
                col += 1;
                continue 'cols;
            }
            out_array[array_index] = *in_matrix.const_index((row, col));
            array_index += 1;
            col += 1;
        }

        row += 1;
    }

    Matr2::from_array(out_array)
}
#[inline]
pub const fn submatrix_4(in_matrix: &Matr4, omitted_row: usize, omitted_col: usize) -> Matr3 {
    // Make square array
    let mut out_array = [0f32; 9];

    // Make index counter, so as not to overindex, and write correctly
    let mut array_index = 0;

    // Iterate over the `Self` matrix
    let mut row = 0;
    'rows: loop {
        if row >= 4 {
            break 'rows;
        }
        if row == omitted_row {
            row += 1;
            continue 'rows;
        }
        let mut col = 0;
        'cols: loop {
            if col >= 4 {
                break 'cols;
            }
            if col == omitted_col {
                col += 1;
                continue 'cols;
            }
            out_array[array_index] = *in_matrix.const_index((row, col));
            array_index += 1;
            col += 1;
        }

        row += 1;
    }

    Matr3::from_array(out_array)
}
#[inline]
pub const fn array_submatrix<const N: usize>(
    in_array: [f32; N * N],
    omitted_row: usize,
    omitted_col: usize,
) -> [f32; (N - 1) * (N - 1)] {
    // Make square array
    let mut array = [0.0; { N - 1 } * { N - 1 }];

    // Make index counter, so as not to overindex, and write correctly
    let mut array_index = 0;

    // Iterate over the `Self` matrix
    let mut row = 0;
    'rows: loop {
        if row >= N {
            break 'rows;
        }
        if row == omitted_row {
            row += 1;
            continue 'rows;
        }
        let row_offset = row * N;
        let mut col = 0;
        'cols: loop {
            if col >= N {
                break 'cols;
            }
            if col == omitted_col {
                col += 1;
                continue 'cols;
            }
            let src = row_offset + col;
            array[array_index] = *in_array.const_index(src);
            array_index += 1;
            col += 1;
        }

        row += 1;
    }

    array
}
impl const ArraySubmatrix<3, 2> for [f32; 9] {
    type Output = [f32; 4];
    #[inline]
    fn array_submatrix(&self, omitted_row: usize, omitted_col: usize) -> Self::Output {
        array_submatrix::<3>(*self, omitted_row, omitted_col)
    }
}
impl const ArraySubmatrix<16, 9> for [f32; 16] {
    type Output = [f32; 9];
    #[inline]
    fn array_submatrix(&self, omitted_row: usize, omitted_col: usize) -> Self::Output {
        array_submatrix::<4>(*self, omitted_row, omitted_col)
    }
}
// impl<const N: usize> const ArraySubmatrix<N> for [f32; N * N]
// where
//     [f32; N * N]: ~const ConstIndex<usize, Output = f32>,
//     [(); (N + 1) * (N + 1)]:,
// {
//     type Output = [f32; (N - 1) * (N - 1)];
// }
pub type Square<T, const N: usize> = [T; N * N];
#[const_trait]
pub trait Submatrix<const N: usize> {
    type SubmatrixOutput;
    fn submatrix(&self, omitted_row: usize, omitted_col: usize) -> Self::SubmatrixOutput;
    // where
    //     Self: ~const AsArray<f32, { N * N }>,
    //     Square<f32, { N }>: ~const ArraySubmatrix<N, { N - 1 }, Output = [f32; (N - 1) * (N - 1)]>
    //         + ~const ConstIndex<usize, Output = f32>,
    //     Self::SubmatrixOutput: ~const FromArray<f32, { (N - 1) * (N - 1) }>,
    //     [(); N * N]:,
    // {
    //     let sma = self.as_array().array_submatrix(omitted_row, omitted_col);
    //     Self::SubmatrixOutput::from_array(sma)
    // }
}
impl const Submatrix<3> for Matr3 {
    type SubmatrixOutput = Matr2;
    #[inline]
    fn submatrix(&self, omitted_row: usize, omitted_col: usize) -> Self::SubmatrixOutput {
        submatrix_3(self, omitted_row, omitted_col)
        // let sub = self.as_array().array_submatrix(omitted_row, omitted_col);
        // Matr2::from_array(sub)
    }
}
// impl ArraySubmatrix<3, 2> for [f32; 9] {}
// impl ArraySubmatrix<4, 3> for [f32; 16] {}

impl const Submatrix<4> for Matr4 {
    type SubmatrixOutput = Matr3;
    #[inline]
    fn submatrix(&self, omitted_row: usize, omitted_col: usize) -> Self::SubmatrixOutput {
        submatrix_4(self, omitted_row, omitted_col)
        // let sub = self.as_array().array_submatrix(omitted_row, omitted_col);
        // Matr3::from_array(sub)
    }
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
        use super::{Matr2, Matr3, Matrix, Submatrix};
        #[test]
        fn submat() {
            let a = Matr3::new([1.0, 5.0, 0.0, -3.0, 2.0, 7.0, 0.0, 6.0, -3.0]);
            assert_eq!(a.submatrix(0, 2), Matr2::new([-3.0, 2.0, 0.0, 6.0]))
        }
    }
    mod mat4 {
        use super::{Matr3, Matr4, Matrix, Submatrix};
        #[test]
        fn submat() {
            let a = Matr4::new([
                -6.0, 1.0, 1.0, 6.0, -8.0, 5.0, 8.0, 6.0, -1.0, 0.0, 8.0, 2.0, -7.0, 1.0, -1.0, 1.0,
            ]);
            assert_eq!(
                a.submatrix(2, 1),
                Matr3::new([-6.0, 1.0, 6.0, -8.0, 8.0, 6.0, -7.0, -1.0, 1.0])
            )
        }
    }
}
