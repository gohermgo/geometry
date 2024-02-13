use crate::{Det, Mat2, Mat3, Matrix, Submat};

pub trait Minor<'mat, const DIM: usize>: Matrix<'mat, DIM>
where
    &'mat Self: Into<[Self::Vert; DIM]>,
    Self: 'mat + Into<[Self::Vert; DIM]> + Submat<'mat, DIM>,
{
    fn minor(&'mat self, row: usize, col: usize) -> f32;
}

impl<'mat> Minor<'mat, 3> for Mat3 {
    fn minor(&'mat self, row: usize, col: usize) -> f32 {
        self.submat(row, col).det()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    mod mat3 {
        use super::{Det, Mat3, Matrix, Minor, Submat};
        #[test]
        fn calc_minor() {
            let a = Mat3::new([3.0, 5.0, 0.0, 2.0, -1.0, -7.0, 6.0, -1.0, 5.0]);
            let b = a.submat(1, 0);
            assert_eq!(b.det(), a.minor(1, 0))
        }
    }
}
