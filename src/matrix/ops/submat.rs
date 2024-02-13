use crate::{Mat2, Mat3, Mat4, Matrix};

pub trait Submat<'mat, const DIM: usize>: Matrix<'mat, DIM>
where
    &'mat Self: Into<[Self::Vert; DIM]>,
    Self: 'mat + Into<[Self::Vert; DIM]>,
{
    type Submat;
    fn submat(&'mat self, omitted_row: usize, omitted_col: usize) -> Self::Submat;
}

impl<'mat> Submat<'mat, 3> for Mat3 {
    type Submat = Mat2;
    #[inline]
    fn submat(&'mat self, omitted_row: usize, omitted_col: usize) -> Self::Submat {
        let mut array = [0.0; 4];
        let mut i = 0;
        for row in 0..3 {
            if row == omitted_row {
                continue;
            }
            for col in 0..3 {
                if col == omitted_col {
                    continue;
                }
                array[i] = self[(row, col)];
                i += 1;
            }
        }
        Mat2::new(array)
    }
}

impl<'mat> Submat<'mat, 4> for Mat4 {
    type Submat = Mat3;
    #[inline]
    fn submat(&'mat self, omitted_row: usize, omitted_col: usize) -> Self::Submat {
        let mut array = [0.0; 9];
        let mut i = 0;
        for row in 0..4 {
            if row == omitted_row {
                continue;
            }
            for col in 0..4 {
                if col == omitted_col {
                    continue;
                }
                array[i] = self[(row, col)];
                i += 1;
            }
        }
        Mat3::new(array)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    mod mat3 {
        use super::{Mat2, Mat3, Matrix, Submat};
        #[test]
        fn submat() {
            let a = Mat3::new([1.0, 5.0, 0.0, -3.0, 2.0, 7.0, 0.0, 6.0, -3.0]);
            assert_eq!(a.submat(0, 2), Mat2::new([-3.0, 2.0, 0.0, 6.0]))
        }
    }
    mod mat4 {
        use super::{Mat3, Mat4, Matrix, Submat};
        #[test]
        fn submat() {
            let a = Mat4::new([
                -6.0, 1.0, 1.0, 6.0, -8.0, 5.0, 8.0, 6.0, -1.0, 0.0, 8.0, 2.0, -7.0, 1.0, -1.0, 1.0,
            ]);
            assert_eq!(
                a.submat(2, 1),
                Mat3::new([-6.0, 1.0, 6.0, -8.0, 8.0, 6.0, -7.0, -1.0, 1.0])
            )
        }
    }
}
