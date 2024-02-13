use crate::{Mat2, Matrix};

pub trait Cofact<'mat, const DIM: usize>: Matrix<'mat, DIM>
where
    &'mat Self: Into<[Self::Vert; DIM]>,
    Self: 'mat + Into<[Self::Vert; DIM]>,
{
    fn cofact(&'mat self, row: usize, col: usize) -> f32;
}
