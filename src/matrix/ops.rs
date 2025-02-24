use core::ops::Index;

pub use crate::matrix::ops::submatrix::ConstIndex;
use crate::matrix::{
    ops::{
        cofactor::ArrayCofactor,
        submatrix::{array_submatrix, submatrix_4, Square},
    },
    AsArray, FromArray, Matr3, Matr4, Matrix,
};
mod cofactor;
mod determinant;
mod inverse;
mod minor;
mod submatrix;

pub use cofactor::Cofactor;
pub use determinant::Determinant;
const fn array_inverse<const N: usize>(in_arr: [f32; N * N]) -> Option<[f32; N * N]>
where
    [f32; N * N]: ~const Determinant + ~const ArrayCofactor<N, { N - 1 }>,
{
    if in_arr.is_invertible() {
        let det = in_arr.determinant();
        let mut out_arr = [0f32; N * N];

        let mut row = 0;
        while row < N {
            let mut col = 0;
            while col < N {
                // Transpose by flipping indexing
                out_arr[row + (col * N)] = in_arr.array_cofactor(row, col) / det;
                col += 1;
            }
            row += 1;
        }
        Some(out_arr)
    } else {
        None
    }
}
#[const_trait]
pub trait Inverse {
    fn inverse(&self) -> Option<Self>
    where
        Self: Sized;
}
impl const Inverse for Matr3 {
    #[inline]
    fn inverse(&self) -> Option<Self> {
        if let Some(array) = inverse_3(*self.as_array()) {
            Some(Matr3::from_array(array))
        } else {
            None
        }
        // if self.is_invertible() {
        //     let determinant = self.determinant();
        //     let mut array = [0.0; DIM * DIM];
        //     let mut row = 0;
        //     while row < DIM {
        //         let mut col = 0;
        //         while col < DIM {
        //             // Transpose by flipping indexing
        //             array[row + (col * DIM)] =
        //                 self.as_array().array_cofactor(row, col) / determinant;
        //             col += 1;
        //         }
        //         row += 1;
        //     }
        //     Some(Self::new(array))
        // } else {
        //     None
        // }
    }
}
impl Inverse for Matr4 {
    #[inline]
    fn inverse(&self) -> Option<Self> {
        let det = self.determinant();
        if det != 0. {
            let mut out_arr = [0f32; 4 * 4];
            for row in 0..4 {
                for col in 0..4 {
                    // Transpose by flipping indexing
                    out_arr[row + (col * 4)] = self.cofactor(row, col) / det;
                }
            }
            Some(Matr4::from_array(out_arr))
        } else {
            None
        }
    }
}

pub use minor::Minor;
pub use submatrix::Submatrix;

#[inline]
pub const fn determinant_2(array: Square<f32, 2>) -> f32 {
    (array[0] * array[3]) - (array[1] * array[2])
}
#[inline]
pub const fn minor_3(in_array: Square<f32, 3>, row: usize, col: usize) -> f32 {
    let out_array: Square<f32, 2> = array_submatrix::<3>(in_array, row, col);
    determinant_2(out_array)
}
#[inline]
pub const fn cofactor_3(array: Square<f32, 3>, row: usize, col: usize) -> f32 {
    if (row + col) % 2 != 0 {
        -minor_3(array, row, col)
    } else {
        minor_3(array, row, col)
    }
}
#[inline]
pub const fn determinant_3(array: Square<f32, 3>) -> f32 {
    let mut tally = 0.0;
    let row = 0;
    let mut col = 0;
    while col < 3 {
        let src = 3 * col;
        tally += array[src] * cofactor_3(array, row, col);
        col += 1;
    }
    tally
}
#[inline]
pub const fn is_invertible_3(in_array: Square<f32, 3>) -> bool {
    determinant_3(in_array) != 0.
}
#[inline]
pub const fn inverse_3(in_array: Square<f32, 3>) -> Option<Square<f32, 3>> {
    let det = determinant_3(in_array);
    if det != 0. {
        let mut out_arr = [0f32; 3 * 3];

        let mut row = 0;
        while row < 3 {
            let mut col = 0;
            while col < 3 {
                // Transpose by flipping indexing
                out_arr[row + (col * 3)] = cofactor_3(in_array, row, col) / det;
                col += 1;
            }
            row += 1;
        }
        Some(out_arr)
    } else {
        None
    }
}
#[inline]
pub const fn minor_4(in_array: Square<f32, 4>, row: usize, col: usize) -> f32 {
    let out_array: Square<f32, 3> = array_submatrix::<4>(in_array, row, col);
    determinant_3(out_array)
}
#[inline]
pub const fn cofactor_4(array: Square<f32, 4>, row: usize, col: usize) -> f32 {
    if (row + col) % 2 != 0 {
        -minor_4(array, row, col)
    } else {
        minor_4(array, row, col)
    }
}
#[inline]
pub const fn determinant_4(array: Square<f32, 4>) -> f32 {
    let mut tally = 0.0;
    let row = 0;
    let mut col = 0;
    while col < 3 {
        let src = 3 * col;
        tally += array[src] * cofactor_4(array, row, col);
        col += 1;
    }
    tally
}
#[inline]
pub const fn is_invertible_4(in_array: Square<f32, 4>) -> bool {
    determinant_4(in_array) != 0.
}
#[inline]
pub const fn inverse_4(in_array: Square<f32, 4>) -> Option<Square<f32, 4>> {
    let det = determinant_4(in_array);
    if det != 0. {
        let mut out_arr = [0f32; 4 * 4];

        let mut row = 0;
        while row < 4 {
            let mut col = 0;
            while col < 4 {
                // Transpose by flipping indexing
                out_arr[row + (col * 4)] = cofactor_4(in_array, row, col) / det;
                col += 1;
            }
            row += 1;
        }
        Some(out_arr)
    } else {
        None
    }
}
