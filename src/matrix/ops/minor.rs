//! Minor operation

use crate::{
    matrix::{
        ops::{
            minor_3, minor_4,
            submatrix::{array_submatrix, ArraySubmatrix, ConstIndex, Square},
        },
        AsArray,
    },
    Determinant, Matr3, Matr4, Matrix, Submatrix,
};

#[const_trait]
pub trait ArrayMinor<const N: usize, const NS: usize> {
    fn array_minor(&self, row: usize, col: usize) -> f32;
}

impl const ArrayMinor<3, 2> for [f32; 9] {
    #[inline]
    fn array_minor(&self, row: usize, col: usize) -> f32 {
        minor_3(*self, row, col)
    }
}

// #[inline]
// pub const fn minor_3(array: Square<f32, 3>, row: usize, col: usize) -> f32
// {
//     array_submatrix::<3>(array, row, col).determinant()
// }

// impl const ArrayMinor<4, 3> for [f32; 16] {
//     #[inline]
//     fn array_minor(&self, row: usize, col: usize) -> f32 {
//         self.array_submatrix(row, col).determinant()
//     }
// }
// impl<const N: usize> const ArrayMinor<N, { N - 1 }> for [f32; N * N]
// where
//     [f32; N * N]: ~const ArraySubmatrix<{ N * N }, { (N - 1) * (N - 1) }>,

//     <[f32; N * N] as ArraySubmatrix<{ N * N }, { (N - 1) * (N - 1) }>>::Output<
//         { (N - 1) * (N - 1) },
//     >: ~const Determinant,
// {
//     #[inline]
//     fn array_minor(&self, row: usize, col: usize) -> f32 {
//         self.array_submatrix(row, col).determinant()
//     }
// }
/// The determinant of the submatrix
#[const_trait]
pub trait Minor<const DIM: usize> {
    fn minor(&self, row: usize, col: usize) -> f32;
}
impl Minor<3> for Matr3 {
    #[inline]
    fn minor(&self, row: usize, col: usize) -> f32 {
        minor_3(*self.as_array(), row, col)
    }
}
impl Minor<4> for Matr4 {
    #[inline]
    fn minor(&self, row: usize, col: usize) -> f32 {
        let submatrix = self.submatrix(row, col);

        submatrix.determinant()
    }
}
// impl<T, const N: usize> const Minor<N> for T
// where
//     [f32; N * N]: ~const ArrayMinor<N, { N - 1 }>,
//     T: ~const AsArray<f32, { N * N }>,
// {
//     #[inline]
//     fn minor(&self, row: usize, col: usize) -> f32 {
//         self.as_array().array_minor(row, col)
//     }
// }
// impl ArrayMinor<3, 2> for [f32; 9] {}
// impl Minor<3, 2> for Matr3 {}

// impl ArrayMinor<4, 3> for [f32; 16] {}
// impl Minor<4, 3> for Matr4 {}

// impl<'mat, 'submat> Minor<'mat, 'submat, 3, 2> for Mat3 {
//     fn minor(&'mat self, row: usize, col: usize) -> f32 {
//         self.submat(row, col).det()
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    mod mat3 {
        use super::{Determinant, Matr3, Matrix, Minor, Submatrix};
        #[test]
        fn calc_minor() {
            let a = Matr3::new([3.0, 5.0, 0.0, 2.0, -1.0, -7.0, 6.0, -1.0, 5.0]);
            let b = a.submatrix(1, 0);
            assert_eq!(b.determinant(), a.minor(1, 0))
        }
    }
}
