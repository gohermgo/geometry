use crate::{
    matrix::{
        ops::{cofactor_3, cofactor_4, minor::ArrayMinor, submatrix::ConstIndex},
        AsArray,
    },
    Determinant, Matr3, Matr4, Matrix, Minor,
};
use std::ops::Index;
#[const_trait]
pub trait ArrayCofactor<const N: usize, const NS: usize>: ~const ArrayMinor<N, NS> {
    fn array_cofactor(&self, row: usize, col: usize) -> f32;
}
impl<const N: usize> const ArrayCofactor<N, { N - 1 }> for [f32; N * N]
where
    [f32; N * N]: ~const ArrayMinor<N, { N - 1 }>,
{
    #[inline]
    fn array_cofactor(&self, row: usize, col: usize) -> f32 {
        if (row + col) % 2 != 0 {
            -self.array_minor(row, col)
        } else {
            self.array_minor(row, col)
        }
    }
}
#[const_trait]
pub trait Cofactor<const N: usize, const NS: usize> {
    fn cofactor(&self, row: usize, col: usize) -> f32;
    // {
    //     if (row + col) % 2 != 0 {
    //         -self.as_array().array_cofactor(row, col)
    //     } else {
    //         self.as_array().array_cofactor(row, col)
    //     }
    // }
}
// impl const ArrayCofactor<3, 2> for [f32; 9] {}
impl Cofactor<3, 2> for Matr3 {
    #[inline]
    fn cofactor(&self, row: usize, col: usize) -> f32 {
        let minor = self.minor(row, col);

        if (row + col) % 2 != 0 {
            -minor
        } else {
            minor
        }
    }
}

// impl const ArrayCofactor<16, 9> for [f32; 16] {}
impl Cofactor<4, 3> for Matr4 {
    #[inline]
    fn cofactor(&self, row: usize, col: usize) -> f32 {
        let minor = self.minor(row, col);
        if (row + col) % 2 != 0 {
            -minor
        } else {
            minor
        }
    }
}
