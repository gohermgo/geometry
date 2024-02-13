use crate::{Mat2, Matrix};

pub trait Det<'mat, const DIM: usize>: Matrix<'mat, DIM>
where
    &'mat Self: Into<[Self::Vert; DIM]>,
    Self: 'mat + Into<[Self::Vert; DIM]>,
{
    fn det(&'mat self) -> f32;
}

impl<'mat> Det<'mat, 2> for Mat2 {
    fn det(&'mat self) -> f32 {
        (self[(0, 0)] * self[(1, 1)]) - (self[(0, 1)] * self[(1, 0)])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    mod mat2 {
        use super::{Det, Mat2, Matrix};
        #[test]
        fn calc_det() {
            let a = Mat2::new([1.0, 5.0, -3.0, 2.0]);
            assert_eq!(a.det(), 17.0)
        }
    }
}
