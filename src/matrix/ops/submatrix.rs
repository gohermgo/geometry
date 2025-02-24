//! Submatrix-ing operation

use crate::{
    matrix::{AsArray, FromArray},
    Matr2, Matr3, Matr4,
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

#[inline]
pub fn array_submatrix<
    const N: usize,
    const NS: usize,
    const N_ROWS: usize,
    const N_COLS: usize,
>(
    in_array: [f32; N],
    omitted_row: usize,
    omitted_col: usize,
) -> [f32; NS] {
    // Make square array
    let mut out_array = [0.0; NS];

    // Make index counter, so as not to overindex, and write correctly
    let mut array_index = 0;

    let mut row = 0;
    'rows: loop {
        if row >= N_ROWS {
            break 'rows;
        }
        if row == omitted_row {
            row += 1;
            continue 'rows;
        }
        let mut col = 0;
        let offset = row * N_COLS;
        'cols: loop {
            if col >= N_COLS {
                break 'cols;
            }
            if col == omitted_col {
                col += 1;
                continue 'cols;
            }
            // index.1 + (4 * index.0)
            let index = col + offset;
            out_array[array_index] = in_array[index];
            array_index += 1;
            col += 1;
        }

        row += 1;
    }

    out_array
}
pub trait Submatrix<const N: usize> {
    type Output;
    fn submatrix(&self, omitted_row: usize, omitted_col: usize) -> Self::Output;
}
impl Submatrix<3> for Matr3 {
    type Output = Matr2;
    #[inline]
    fn submatrix(&self, omitted_row: usize, omitted_col: usize) -> Self::Output {
        Matr2::from_array(array_submatrix::<9, 4, 3, 3>(
            *self.as_array(),
            omitted_row,
            omitted_col,
        ))
        // let sub = self.as_array().array_submatrix(omitted_row, omitted_col);
        // Matr2::from_array(sub)
    }
}
// impl ArraySubmatrix<3, 2> for [f32; 9] {}
// impl ArraySubmatrix<4, 3> for [f32; 16] {}

impl Submatrix<4> for Matr4 {
    type Output = Matr3;
    #[inline]
    fn submatrix(&self, omitted_row: usize, omitted_col: usize) -> Self::Output {
        Matr3::from_array(array_submatrix::<16, 9, 4, 4>(
            *self.as_array(),
            omitted_row,
            omitted_col,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    mod mat3 {
        use crate::matrix::FromArray;

        use super::{Matr2, Matr3, Submatrix};
        #[test]
        fn submat() {
            let a = Matr3::from_array([1.0, 5.0, 0.0, -3.0, 2.0, 7.0, 0.0, 6.0, -3.0]);
            assert_eq!(a.submatrix(0, 2), Matr2::from_array([-3.0, 2.0, 0.0, 6.0]))
        }
    }
    mod mat4 {
        use crate::matrix::FromArray;

        use super::{Matr3, Matr4, Submatrix};
        #[test]
        fn submat() {
            let a = Matr4::from_array([
                -6.0, 1.0, 1.0, 6.0, -8.0, 5.0, 8.0, 6.0, -1.0, 0.0, 8.0, 2.0, -7.0, 1.0, -1.0, 1.0,
            ]);
            assert_eq!(
                a.submatrix(2, 1),
                Matr3::from_array([-6.0, 1.0, 6.0, -8.0, 8.0, 6.0, -7.0, -1.0, 1.0])
            )
        }
    }
}
