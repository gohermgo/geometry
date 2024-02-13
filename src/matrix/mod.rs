use std::{
    ops::{Index, IndexMut, Mul},
    simd::{f32x16, f32x4, num::SimdFloat, simd_swizzle},
};

use crate::{Vert2, Vert3, Vert4};

mod ops;
pub use ops::{Det, Submat};

pub trait Matrix<'mat, const DIM: usize>
where
    &'mat Self: Into<[Self::Vert; DIM]>,
    Self: 'mat + Into<[Self::Vert; DIM]>,
{
    type Vert;
    fn new(array: [f32; DIM * DIM]) -> Self;
    fn identity() -> Self;
    fn transpose(&self) -> Self;
    #[inline]
    fn as_column_vectors(&self) -> [Self::Vert; DIM] {
        self.transpose().into()
    }
    #[inline]
    fn as_row_vectors(&'mat self) -> [Self::Vert; DIM] {
        self.into()
    }
}

#[derive(Debug, PartialEq)]
pub struct Mat2(pub(crate) f32x4);
pub(crate) const T_SWIZZLE_2: [usize; 4] = [0, 2, 1, 3];
impl<'mat> Matrix<'mat, 2> for Mat2 {
    type Vert = Vert2;
    #[inline]
    fn new(array: [f32; 2 * 2]) -> Self {
        Self(f32x4::from_array(array))
    }
    #[inline]
    fn identity() -> Self {
        Self(f32x4::from_array([1.0, 0.0, 0.0, 1.0]))
    }
    #[inline]
    fn transpose(&self) -> Self {
        Self(simd_swizzle!(self.0, T_SWIZZLE_2))
    }
    // #[inline]
    // fn as_column_vectors(&self) -> [Self::Vert; 2] {
    //     self.transpose().into()
    // }
    // #[inline]
    // fn as_row_vectors(&self) -> [Self::Vert; 2] {
    //     self.into()
    // }
}
impl Index<(usize, usize)> for Mat2 {
    type Output = f32;
    #[inline]
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.0[index.1 + (2 * index.0)]
    }
}
impl From<Mat2> for [Vert2; 2] {
    #[inline]
    fn from(value: Mat2) -> Self {
        let v0 = Vert2::new(value[(0, 0)], value[(0, 1)]);
        let v1 = Vert2::new(value[(1, 0)], value[(1, 1)]);
        [v0, v1]
    }
}
impl From<&Mat2> for [Vert2; 2] {
    #[inline]
    fn from(value: &Mat2) -> Self {
        let v0 = Vert2::new(value[(0, 0)], value[(0, 1)]);
        let v1 = Vert2::new(value[(1, 0)], value[(1, 1)]);
        [v0, v1]
    }
}
#[derive(Debug, PartialEq)]
pub struct Mat3([f32; 9]);
impl<'mat> Matrix<'mat, 3> for Mat3 {
    type Vert = Vert3;
    #[inline]
    fn new(array: [f32; 3 * 3]) -> Self {
        Self(array)
    }
    #[inline]
    fn identity() -> Self {
        Self([1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0])
    }
    #[inline]
    fn transpose(&self) -> Self {
        Self([
            self[(0, 0)],
            self[(1, 0)],
            self[(2, 0)],
            self[(0, 1)],
            self[(1, 1)],
            self[(2, 1)],
            self[(0, 2)],
            self[(1, 2)],
            self[(2, 2)],
        ])
    }
    // #[inline]
    // fn as_row_vectors(&self) -> [Self::Vert; 3] {
    //     self.into()
    // }
}
impl Index<(usize, usize)> for Mat3 {
    type Output = f32;
    #[inline]
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.0[index.1 + (3 * index.0)]
    }
}
impl From<Mat3> for [Vert3; 3] {
    #[inline]
    fn from(value: Mat3) -> Self {
        let v0 = Vert3::new(value[(0, 0)], value[(0, 1)], value[(0, 2)]);
        let v1 = Vert3::new(value[(1, 0)], value[(1, 1)], value[(1, 2)]);
        let v2 = Vert3::new(value[(2, 0)], value[(2, 1)], value[(2, 2)]);
        [v0, v1, v2]
    }
}
impl From<&Mat3> for [Vert3; 3] {
    #[inline]
    fn from(value: &Mat3) -> Self {
        let v0 = Vert3::new(value[(0, 0)], value[(0, 1)], value[(0, 2)]);
        let v1 = Vert3::new(value[(1, 0)], value[(1, 1)], value[(1, 2)]);
        let v2 = Vert3::new(value[(2, 0)], value[(2, 1)], value[(2, 2)]);
        [v0, v1, v2]
    }
}
#[derive(Debug, PartialEq)]
pub struct Mat4(f32x16);
pub(crate) const T_SWIZZLE_4: [usize; 16] = [0, 4, 8, 12, 1, 5, 9, 13, 2, 6, 10, 14, 3, 7, 11, 15];
impl<'mat> Matrix<'mat, 4> for Mat4 {
    type Vert = Vert4;
    #[inline]
    fn new(array: [f32; 4 * 4]) -> Self {
        Self(f32x16::from_array(array))
    }
    #[inline]
    fn identity() -> Self {
        Self(f32x16::from_array([
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        ]))
    }
    #[inline]
    fn transpose(&self) -> Self {
        Self(simd_swizzle!(self.0, T_SWIZZLE_4))
    }
}
impl Index<(usize, usize)> for Mat4 {
    type Output = f32;
    #[inline]
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.0[index.1 + (4 * index.0)]
    }
}
impl IndexMut<(usize, usize)> for Mat4 {
    #[inline]
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.0[index.1 + (4 * index.0)]
    }
}
// impl From<Mat4> for [Vert4; 4] {}
impl From<Mat4> for [Vert4; 4] {
    #[inline]
    fn from(value: Mat4) -> Self {
        let v0 = Vert4::new(value[(0, 0)], value[(0, 1)], value[(0, 2)], value[(0, 3)]);
        let v1 = Vert4::new(value[(1, 0)], value[(1, 1)], value[(1, 2)], value[(1, 3)]);
        let v2 = Vert4::new(value[(2, 0)], value[(2, 1)], value[(2, 2)], value[(2, 3)]);
        let v3 = Vert4::new(value[(3, 0)], value[(3, 1)], value[(3, 2)], value[(3, 3)]);
        [v0, v1, v2, v3]
    }
}
impl From<&Mat4> for [Vert4; 4] {
    #[inline]
    fn from(value: &Mat4) -> Self {
        let v0 = Vert4::new(value[(0, 0)], value[(0, 1)], value[(0, 2)], value[(0, 3)]);
        let v1 = Vert4::new(value[(1, 0)], value[(1, 1)], value[(1, 2)], value[(1, 3)]);
        let v2 = Vert4::new(value[(2, 0)], value[(2, 1)], value[(2, 2)], value[(2, 3)]);
        let v3 = Vert4::new(value[(3, 0)], value[(3, 1)], value[(3, 2)], value[(3, 3)]);
        [v0, v1, v2, v3]
    }
}
impl Mul<Mat4> for Mat4 {
    type Output = Mat4;
    #[inline]
    fn mul(self, rhs: Mat4) -> Self::Output {
        // M[(r, c)] = A[(r, 0)] * B[(0, c)] (= B[0 + 4c])
        //           + A[(r, 1)] * B[(1, c)]
        //           + A[(r, 2)] * B[(2, c)]
        //           + A[(r, 3)] * B[(3, c)]
        let mut output: Mat4 = Mat4::identity();
        let rows = self.as_row_vectors();
        let cols = rhs.as_column_vectors();
        for (row_idx, row) in rows.iter().enumerate() {
            for (col_idx, col) in cols.iter().enumerate() {
                output[(row_idx, col_idx)] = (row.0 * col.0).reduce_sum();
            }
        }
        output
    }
}
impl Mul<&Mat4> for Mat4 {
    type Output = Mat4;
    #[inline]
    fn mul(self, rhs: &Mat4) -> Self::Output {
        // M[(r, c)] = A[(r, 0)] * B[(0, c)] (= B[0 + 4c])
        //           + A[(r, 1)] * B[(1, c)]
        //           + A[(r, 2)] * B[(2, c)]
        //           + A[(r, 3)] * B[(3, c)]
        let mut output: Mat4 = Mat4::identity();
        let rows = self.as_row_vectors();
        let cols = rhs.as_column_vectors();
        for (row_idx, row) in rows.iter().enumerate() {
            for (col_idx, col) in cols.iter().enumerate() {
                output[(row_idx, col_idx)] = (row.0 * col.0).reduce_sum();
            }
        }
        output
    }
}
impl Mul<Mat4> for &Mat4 {
    type Output = Mat4;
    #[inline]
    fn mul(self, rhs: Mat4) -> Self::Output {
        // M[(r, c)] = A[(r, 0)] * B[(0, c)] (= B[0 + 4c])
        //           + A[(r, 1)] * B[(1, c)]
        //           + A[(r, 2)] * B[(2, c)]
        //           + A[(r, 3)] * B[(3, c)]
        let mut output: Mat4 = Mat4::identity();
        let rows = self.as_row_vectors();
        let cols = rhs.as_column_vectors();
        for (row_idx, row) in rows.iter().enumerate() {
            for (col_idx, col) in cols.iter().enumerate() {
                output[(row_idx, col_idx)] = (row.0 * col.0).reduce_sum();
            }
        }
        output
    }
}
impl Mul<&Mat4> for &Mat4 {
    type Output = Mat4;
    #[inline]
    fn mul(self, rhs: &Mat4) -> Self::Output {
        // M[(r, c)] = A[(r, 0)] * B[(0, c)] (= B[0 + 4c])
        //           + A[(r, 1)] * B[(1, c)]
        //           + A[(r, 2)] * B[(2, c)]
        //           + A[(r, 3)] * B[(3, c)]
        let mut output: Mat4 = Mat4::identity();
        let rows = self.as_row_vectors();
        let cols = rhs.as_column_vectors();
        for (row_idx, row) in rows.iter().enumerate() {
            for (col_idx, col) in cols.iter().enumerate() {
                output[(row_idx, col_idx)] = (row.0 * col.0).reduce_sum();
            }
        }
        output
    }
}
impl Mul<Vert4> for Mat4 {
    type Output = Vert4;
    #[inline]
    fn mul(self, rhs: Vert4) -> Self::Output {
        let mut output: Vert4 = Vert4::new(0.0, 0.0, 0.0, 0.0);
        let rows = self.as_row_vectors();
        for (idx, row) in rows.iter().enumerate() {
            output.0[idx] = (row.0 * rhs.0).reduce_sum();
        }
        output
    }
}
impl Mul<&Vert4> for Mat4 {
    type Output = Vert4;
    #[inline]
    fn mul(self, rhs: &Vert4) -> Self::Output {
        let mut output: Vert4 = Vert4::new(0.0, 0.0, 0.0, 0.0);
        let rows = self.as_row_vectors();
        for (idx, row) in rows.iter().enumerate() {
            output.0[idx] = (row.0 * rhs.0).reduce_sum();
        }
        output
    }
}
impl Mul<Vert4> for &Mat4 {
    type Output = Vert4;
    #[inline]
    fn mul(self, rhs: Vert4) -> Self::Output {
        let mut output: Vert4 = Vert4::new(0.0, 0.0, 0.0, 0.0);
        let rows = self.as_row_vectors();
        for (idx, row) in rows.iter().enumerate() {
            output.0[idx] = (row.0 * rhs.0).reduce_sum();
        }
        output
    }
}
impl Mul<&Vert4> for &Mat4 {
    type Output = Vert4;
    #[inline]
    fn mul(self, rhs: &Vert4) -> Self::Output {
        let mut output: Vert4 = Vert4::new(0.0, 0.0, 0.0, 0.0);
        let rows = self.as_row_vectors();
        for (idx, row) in rows.iter().enumerate() {
            output.0[idx] = (row.0 * rhs.0).reduce_sum();
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn constructing_mat4() {
        let m = Mat4(f32x16::from_array([
            1.0, 2.0, 3.0, 4.0, 5.5, 6.5, 7.5, 8.5, 9.0, 10.0, 11.0, 12.0, 13.5, 14.5, 15.5, 16.5,
        ]));
        assert_eq!(m[(0, 0)], 1.0);
        assert_eq!(m[(0, 3)], 4.0);
        assert_eq!(m[(1, 0)], 5.5);
        assert_eq!(m[(1, 2)], 7.5);
        assert_eq!(m[(2, 2)], 11.0);
        assert_eq!(m[(3, 0)], 13.5);
        assert_eq!(m[(3, 2)], 15.5);
    }
    #[test]
    fn constructing_mat3() {
        let m = Mat3([-3.0, 5.0, 0.0, 1.0, -2.0, -7.0, 0.0, 1.0, 1.0]);
        assert_eq!(m[(0, 0)], -3.0);
        assert_eq!(m[(1, 1)], -2.0);
        assert_eq!(m[(2, 2)], 1.0);
    }
    #[test]
    fn constructing_mat2() {
        let m = Mat2(f32x4::from_array([-3.0, 5.0, 1.0, -2.0]));
        assert_eq!(m[(0, 0)], -3.0);
        assert_eq!(m[(0, 1)], 5.0);
        assert_eq!(m[(1, 0)], 1.0);
        assert_eq!(m[(1, 1)], -2.0);
    }
    mod mat4 {
        use super::*;
        #[test]
        fn equality() {
            let a = Mat4(f32x16::from_array([
                1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
            ]));
            let b = Mat4(f32x16::from_array([
                1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
            ]));
            assert_eq!(a, b)
        }
        #[test]
        fn inequality() {
            let a = Mat4(f32x16::from_array([
                1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
            ]));
            let b = Mat4(f32x16::from_array([
                2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0,
            ]));
            assert_ne!(a, b)
        }
        mod mul {
            use super::*;
            #[test]
            fn multiplication_by_matrix() {
                let a = Mat4(f32x16::from_array([
                    1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
                ]));
                let b = Mat4(f32x16::from_array([
                    -2.0, 1.0, 2.0, 3.0, 3.0, 2.0, 1.0, -1.0, 4.0, 3.0, 6.0, 5.0, 1.0, 2.0, 7.0,
                    8.0,
                ]));
                assert_eq!(
                    a * b,
                    Mat4(f32x16::from_array([
                        20.0, 22.0, 50.0, 48.0, 44.0, 54.0, 114.0, 108.0, 40.0, 58.0, 110.0, 102.0,
                        16.0, 26.0, 46.0, 42.0
                    ]))
                )
            }
            #[test]
            fn multiplication_by_vertex() {
                let a = Mat4(f32x16::from_array([
                    1.0, 2.0, 3.0, 4.0, 2.0, 4.0, 4.0, 2.0, 8.0, 6.0, 4.0, 1.0, 0.0, 0.0, 0.0, 1.0,
                ]));
                let b = Vert4::new(1.0, 2.0, 3.0, 1.0);
                assert_eq!(a * b, Vert4::new(18.0, 24.0, 33.0, 1.0))
            }
            #[test]
            fn multiplication_by_ident() {
                let a = Mat4(f32x16::from_array([
                    0.0, 1.0, 2.0, 4.0, 1.0, 2.0, 4.0, 8.0, 2.0, 4.0, 8.0, 16.0, 4.0, 8.0, 16.0,
                    32.0,
                ]));
                assert_eq!(&a * Mat4::identity(), a)
            }
            #[test]
            fn multiplication_of_vertex_by_ident() {
                let a = Vert4::new(1.0, 2.0, 3.0, 4.0);
                assert_eq!(Mat4::identity() * &a, a)
            }
        }
        #[test]
        fn transposition() {
            let a = Mat4(f32x16::from_array([
                0.0, 9.0, 3.0, 0.0, 9.0, 8.0, 0.0, 8.0, 1.0, 8.0, 5.0, 3.0, 0.0, 0.0, 5.0, 8.0,
            ]));
            assert_eq!(
                a.transpose(),
                Mat4(f32x16::from_array([
                    0.0, 9.0, 1.0, 0.0, 9.0, 8.0, 8.0, 0.0, 3.0, 0.0, 5.0, 5.0, 0.0, 8.0, 3.0, 8.0
                ]))
            )
        }
        #[test]
        fn transposition_of_ident() {
            let a = Mat4::identity();
            assert_eq!(a.transpose(), Mat4::identity())
        }
    }
}
