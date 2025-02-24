use crate::{Matr3, Matr4, Minor};

pub trait Cofactor<const N: usize> {
    fn cofactor(&self, row: usize, col: usize) -> f32;
}
impl Cofactor<3> for Matr3 {
    #[inline]
    fn cofactor(&self, row: usize, col: usize) -> f32 {
        let minor = self.minor(row, col);

        if (row + col) % 2 != 0 {
            -minor
        } else {
            minor
        }
        // generic_cofactor(self, row, col)
    }
}
impl Cofactor<4> for Matr4 {
    #[inline]
    fn cofactor(&self, row: usize, col: usize) -> f32 {
        let minor = self.minor(row, col);

        if (row + col) % 2 != 0 {
            -minor
        } else {
            minor
        }
        // generic_cofactor(self, row, col)
    }
}
