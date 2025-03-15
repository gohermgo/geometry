use core::ops::Index;

pub use crate::matrix::ops::submatrix::ConstIndex;
use crate::matrix::{FromArray, Matr2, Matr3, Matr4};

mod cofactor;
mod inverse;
mod submatrix;

pub use cofactor::Cofactor;

pub trait Determinant {
    fn determinant(&self) -> f32;
    fn is_invertible(&self) -> bool {
        self.determinant() != 0.
    }
}
impl Determinant for [f32; 4] {
    #[inline]
    fn determinant(&self) -> f32 {
        (self[0] * self[3]) - (self[1] * self[2])
    }
}

impl Determinant for Matr2 {
    #[inline]
    fn determinant(&self) -> f32 {
        (self[(0, 0)] * self[(1, 1)]) - (self[(0, 1)] * self[(1, 0)])
    }
}
fn generic_determinant<T, const N: usize>(inval: &T) -> f32
where
    T: ?Sized + Cofactor<N> + Index<(usize, usize), Output = f32>,
{
    let mut tally = 0.0;
    let row = 0;
    for col in 0..N {
        tally += inval[(row, col)] * inval.cofactor(row, col);
    }
    tally
}
impl Determinant for Matr3 {
    #[inline]
    fn determinant(&self) -> f32 {
        generic_determinant::<Matr3, 3>(self)
    }
}
impl Determinant for Matr4 {
    #[inline]
    fn determinant(&self) -> f32 {
        generic_determinant::<Matr4, 4>(self)
    }
}

/// The determinant of the submatrix
pub trait Minor<const DIM: usize> {
    fn minor(&self, row: usize, col: usize) -> f32;
}
#[inline]
fn generic_minor<T, const N: usize, const NS: usize>(inval: &T, row: usize, col: usize) -> f32
where
    T: Submatrix<N>,
    <T as Submatrix<N>>::Output: Determinant,
{
    let sm = inval.submatrix(row, col);
    sm.determinant()
}
impl Minor<3> for Matr3 {
    #[inline]
    fn minor(&self, row: usize, col: usize) -> f32 {
        generic_minor::<Matr3, 3, 2>(self, row, col)
    }
}
impl Minor<4> for Matr4 {
    #[inline]
    fn minor(&self, row: usize, col: usize) -> f32 {
        generic_minor::<Matr4, 4, 3>(self, row, col)
    }
}

pub trait Inverse {
    type Inverted;
    fn inverse(&self) -> Option<Self::Inverted>;
}
fn generic_inverse<T, const DIM: usize, const N: usize>(inval: &T) -> Option<[f32; DIM]>
where
    T: Determinant + Cofactor<N>,
{
    let det = inval.determinant();
    if det != 0. {
        let mut out_arr = [0f32; DIM];

        let mut row = 0;
        while row < N {
            let mut col = 0;
            while col < N {
                // Transpose by flipping indexing
                out_arr[row + (col * N)] = inval.cofactor(row, col) / det;
                col += 1;
            }
            row += 1;
        }
        Some(out_arr)
    } else {
        None
    }
}
impl Inverse for Matr3 {
    type Inverted = Matr3;
    #[inline]
    fn inverse(&self) -> Option<Self> {
        generic_inverse(self).map(Matr3::from_array)
    }
}
impl Inverse for Matr4 {
    type Inverted = Matr4;
    #[inline]
    fn inverse(&self) -> Option<Self> {
        generic_inverse(self).map(Matr4::from_array)
    }
}

pub use submatrix::Submatrix;

// #[inline]
// pub const fn determinant_2(array: Square<f32, 2>) -> f32 {
//     (array[0] * array[3]) - (array[1] * array[2])
// }
// #[inline]
// pub const fn minor_3(in_array: Square<f32, 3>, row: usize, col: usize) -> f32 {
//     let out_array: Square<f32, 2> = array_submatrix::<3, 2>(in_array, row, col);
//     determinant_2(out_array)
// }
// #[inline]
// pub const fn cofactor_3(array: Square<f32, 3>, row: usize, col: usize) -> f32 {
//     if (row + col) % 2 != 0 {
//         -minor_3(array, row, col)
//     } else {
//         minor_3(array, row, col)
//     }
// }
// #[inline]
// pub const fn determinant_3(array: Square<f32, 3>) -> f32 {
//     let mut tally = 0.0;
//     let row = 0;
//     let mut col = 0;
//     while col < 3 {
//         let src = 3 * col;
//         tally += array[src] * cofactor_3(array, row, col);
//         col += 1;
//     }
//     tally
// }
// #[inline]
// pub const fn is_invertible_3(in_array: Square<f32, 3>) -> bool {
//     determinant_3(in_array) != 0.
// }
// #[inline]
// pub const fn inverse_3(in_array: Square<f32, 3>) -> Option<Square<f32, 3>> {
//     let det = determinant_3(in_array);
//     if det != 0. {
//         let mut out_arr = [0f32; 3 * 3];

//         let mut row = 0;
//         while row < 3 {
//             let mut col = 0;
//             while col < 3 {
//                 // Transpose by flipping indexing
//                 out_arr[row + (col * 3)] = cofactor_3(in_array, row, col) / det;
//                 col += 1;
//             }
//             row += 1;
//         }
//         Some(out_arr)
//     } else {
//         None
//     }
// }
// #[inline]
// pub const fn minor_4(in_array: Square<f32, 4>, row: usize, col: usize) -> f32 {
//     let out_array: Square<f32, 3> = array_submatrix::<4, 3>(in_array, row, col);
//     determinant_3(out_array)
// }
// #[inline]
// pub const fn cofactor_4(array: Square<f32, 4>, row: usize, col: usize) -> f32 {
//     if (row + col) % 2 != 0 {
//         -minor_4(array, row, col)
//     } else {
//         minor_4(array, row, col)
//     }
// }
// #[inline]
// pub const fn determinant_4(array: Square<f32, 4>) -> f32 {
//     let mut tally = 0.0;
//     let row = 0;
//     let mut col = 0;
//     while col < 3 {
//         let src = 3 * col;
//         tally += array[src] * cofactor_4(array, row, col);
//         col += 1;
//     }
//     tally
// }
