use core::marker::PhantomData;
use core::ops::{Index, IndexMut};
use core::ops::Mul;

use std::simd::{f32x16, f32x4, simd_swizzle, LaneCount, Simd, SimdElement, SupportedLaneCount};

use crate::{matrix::ops::ConstIndex, Vert2, Vert3, Vert4};

mod ops;
pub use ops::{Cofactor, Determinant, Minor, Submatrix, Inverse};


pub trait Matrix<const DIM: usize> {
    type Vert;
    fn identity() -> Self;
    fn transpose(&self) -> Self;
}
pub trait AsColumns<const N: usize>: Matrix<N> {
    #[inline]
    fn as_column_vectors<'a>(&'a self) -> [Self::Vert; N]
    where
        Self: 'a + Into<[Self::Vert; N]>,
    {
        self.transpose().into()
    }
    #[inline]
    fn as_row_vectors<'a>(&'a self) -> [Self::Vert; N]
    where
        Self: 'a + Into<[Self::Vert; N]>,
        &'a Self: Into<[Self::Vert; N]>,
    {
        self.into()
    }

}
pub trait FromArray<T, const N: usize> {
    fn from_array(array: [T; N]) -> Self;
}
pub trait AsArray<T, const N: usize> {
    fn as_array(&self) -> &[T; N];
}
impl<T, const N: usize> AsArray<T, N> for [T; N] {
    fn as_array(&self) -> &[T; N] {
        self
    }
}
impl<T, const N: usize> AsArray<T, N> for Simd<T, N> where LaneCount<N>: SupportedLaneCount, T: SimdElement {
    fn as_array(&self) -> &[T; N] {
        self.as_array()
    }
}
pub trait FromSlice<T> {
    fn from_slice(slice: &[T]) -> Self;
}

#[derive(Debug, PartialEq)]
pub struct Matr2(pub(crate) f32x4);
impl FromArray<f32, 4> for Matr2 {
    #[inline]
    fn from_array(array: [f32; 4]) -> Self {
        Matr2(f32x4::from_array(array))
    }
}
impl AsArray<f32, 4> for Matr2 {
    #[inline]
    fn as_array(&self) -> &[f32; 4] {
        self.0.as_array()
    }
}
impl FromSlice<f32> for Matr2 {
    #[inline]
    fn from_slice(slice: &[f32]) -> Self {
        Matr2(f32x4::from_slice(slice))
    }
}
pub(crate) const T_SWIZZLE_2: [usize; 4] = [0, 2, 1, 3];
impl Matrix<2> for Matr2 {
    type Vert = Vert2;
    
    #[inline]
    fn identity() -> Self {
        Matr2::from_array([1.0, 0.0, 0.0, 1.0])
    }
    #[inline]
    fn transpose(&self) -> Self {
        Matr2(simd_swizzle!(self.0, T_SWIZZLE_2))
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
impl Index<(usize, usize)> for Matr2 {
    type Output = f32;
    #[inline]
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.0[index.1 + (2 * index.0)]
    }
}
impl ConstIndex<usize> for Matr2 {
    type Output = f32;
    #[inline]
    fn const_index(&self, index: usize) -> &Self::Output {
        &self.0.as_array()[index]
    }
}
impl ConstIndex<(usize, usize)> for Matr2 {
    type Output = f32;
    #[inline]
    fn const_index(&self, index: (usize, usize)) -> &Self::Output {
        self.const_index(index.1 + (2 * index.0))
    }
}
impl From<Matr2> for [Vert2; 2] {
    #[inline]
    fn from(value: Matr2) -> Self {
        let v0 = Vert2::new(value[(0, 0)], value[(0, 1)]);
        let v1 = Vert2::new(value[(1, 0)], value[(1, 1)]);
        [v0, v1]
    }
}
impl From<&Matr2> for [Vert2; 2] {
    #[inline]
    fn from(value: &Matr2) -> Self {
        let v0 = Vert2::new(value[(0, 0)], value[(0, 1)]);
        let v1 = Vert2::new(value[(1, 0)], value[(1, 1)]);
        [v0, v1]
    }
}
#[derive(Debug, PartialEq)]
pub struct Matr3([f32; 9]);
impl FromArray<f32, 9> for Matr3 {

    #[inline]
    fn from_array(array: [f32; 9]) -> Matr3 {
        Matr3(array)
    }
}
impl FromSlice<f32> for Matr3 {
    #[inline]
    fn from_slice(slice: &[f32]) -> Matr3 {
        assert!(
            slice.len() >= 9,
            "slice length must be at least the number of elements"
        );
        // SAFETY: We just checked that the slice contains
        // at least `N` elements.
        unsafe { Matr3::load(slice.as_ptr().cast()) }
    }
}
impl Matr3 {
    #[inline]
    const unsafe fn load(ptr: *const [f32; 9]) -> Matr3 {
        let mut tmp = core::mem::MaybeUninit::<Matr3>::uninit();
        // SAFETY: `Mat3` always contains `9` elements of type `f32`.  It may have padding
        // which does not need to be initialized.  The safety of reading `ptr` is ensured by the
        // caller.
        unsafe {
            core::ptr::copy_nonoverlapping(ptr, tmp.as_mut_ptr().cast(), 1);
            tmp.assume_init()
        }
    }
}
impl  AsArray<f32, 9> for Matr3 {
    fn as_array(&self) -> &[f32; 9] {
        &self.0
    }
}
impl  Matrix<3> for Matr3 {
    type Vert = Vert3;
    #[inline]
    fn identity() -> Self {
        Self::from_array([1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0])
    }
    #[inline]
    fn transpose(&self) -> Self {
        Self([
            *self.const_index((0, 0)),
            *self.const_index((1, 0)),
            *self.const_index((2, 0)),
            *self.const_index((0, 1)),
            *self.const_index((1, 1)),
            *self.const_index((2, 1)),
            *self.const_index((0, 2)),
            *self.const_index((1, 2)),
            *self.const_index((2, 2)),
        ])
    }
    // #[inline]
    // fn as_row_vectors(&self) -> [Self::Vert; 3] {
    //     self.into()
    // }
}
impl ConstIndex<(usize, usize)> for Matr3 {
    type Output = f32;
    #[inline]
    fn const_index(&self, index: (usize, usize)) -> &Self::Output {
        &self.0[index.1 + (3 * index.0)]
    }
}
impl Index<(usize, usize)> for Matr3 {
    type Output = f32;
    #[inline]
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        self.const_index(index)
    }
}
impl  FromArray<Vert3, 3> for Matr3 {
    #[inline]
    fn from_array(arr: [Vert3; 3]) -> Self {
        let mut out_arr = [0.; 9];
        let mut idx = 0;
        while idx <9 {
            let src_idx = match idx {
                0..3 => 0,
                3..6 => 1,
                6.. => 2
            };

            out_arr[idx] = arr[src_idx][idx % 3];
            idx += 1;
        }
        Matr3::from_array(out_arr)
    }
}
impl From<Matr3> for [Vert3; 3] {
    #[inline]
    fn from(value: Matr3) -> Self {
        let v0 = Vert3::new(value[(0, 0)], value[(0, 1)], value[(0, 2)]);
        let v1 = Vert3::new(value[(1, 0)], value[(1, 1)], value[(1, 2)]);
        let v2 = Vert3::new(value[(2, 0)], value[(2, 1)], value[(2, 2)]);
        [v0, v1, v2]
    }
}
impl From<&Matr3> for [Vert3; 3] {
    #[inline]
    fn from(value: &Matr3) -> Self {
        let v0 = Vert3::new(value[(0, 0)], value[(0, 1)], value[(0, 2)]);
        let v1 = Vert3::new(value[(1, 0)], value[(1, 1)], value[(1, 2)]);
        let v2 = Vert3::new(value[(2, 0)], value[(2, 1)], value[(2, 2)]);
        [v0, v1, v2]
    }
}
pub trait Axis {
    type Primary;
    type Secondary;
}
pub struct X;
impl Axis for X {
    type Primary = Y;
    type Secondary = Z;
}
pub struct Y;
impl Axis for Y {
    type Primary = X;
    type Secondary = Z;
}
pub struct Z;
impl Axis for Z {
    type Primary = X;
    type Secondary = Y;
}
pub struct RotationAxes<T, U>{
    rad: f32,
    axes: PhantomData<(T, U)>
}

impl<T, U> RotationAxes<T, U> {
    pub fn new_rad(rad: f32) -> RotationAxes<T, U> {
        RotationAxes { rad, axes: PhantomData }
    }
    pub fn new_deg(deg: f32) -> RotationAxes<T, U> {
        RotationAxes::new_rad(deg.to_radians())
    }
}
impl RotationAxes<Y, Z> {
    #[inline]
    pub fn yy(&self) -> f32 {
        self.rad.cos()
    }
    #[inline]
    pub fn yz(&self) -> f32 {
        -self.rad.sin()
    }
    #[inline]
    pub fn zy(&self) -> f32 {
        self.rad.sin()
    }
}
impl RotationAxes<X, Z> {
    #[inline]
    fn xz(&self) -> f32 {
        self.rad.sin()
    }
    #[inline]
    fn zx(&self) -> f32 {
        -self.rad.sin()
    }

}
impl RotationAxes<X, Y> {
    #[inline]
    fn yy(&self) -> f32 {
        self.rad.cos()
    }
    #[inline]
    fn xy(&self) -> f32 {
        -self.rad.sin()   
    }
    #[inline]
    fn yx(&self) -> f32 {
        self.rad.sin()
    }
}
impl<T> RotationAxes<X, T> {
    #[inline]
    fn xx(&self) -> f32 {
        self.rad.cos()
    }
}
impl<T> RotationAxes<T, Z> {
    #[inline]
    fn zz(&self) -> f32 {
        self.rad.cos()
    }
}
// impl<T, U> RotationAxes<T, U> where T: RotationAxis, U: RotationAxis {
//     #[inline]
//     fn yy(&self) -> f32 {
//         self.rad.cos()
//     }
// }
pub struct RotationAbout<T> {
    rad: f32,
    rotation_axis_marker:PhantomData<T>
}
impl<T> RotationAbout<T> {
    #[inline]
    pub fn new_rad(rad: f32) -> RotationAbout<T> where T: Axis {
        RotationAbout { rad , rotation_axis_marker:PhantomData  }
    }
    pub fn new_deg(deg: f32) -> RotationAbout<T> where T: Axis {
        RotationAbout::new_rad(deg.to_radians())
    }
}
pub trait IntoAxes<T> where T: Axis {
    fn into_axes(self) -> RotationAxes<T::Primary, T::Secondary>;
}
impl<T> IntoAxes<T> for RotationAbout<T> where T: Axis {
    fn into_axes(self) -> RotationAxes<T::Primary, T::Secondary> {
        RotationAxes::new_rad(self.rad)
    }
}
pub struct ShearOf<T, Ax>(pub T, PhantomData<Ax>);
impl<T, Ax> ShearOf<T, Ax> where Ax: Axis {
    #[inline]
    pub const fn new(value: T) -> ShearOf<T, Ax> {
        ShearOf(value, PhantomData)
    }
}
pub struct ShearingProportion<Ax: Axis> {
    ratio_to_primary: ShearOf<f32, Ax::Primary>,
    ratio_to_secondary: ShearOf<f32, Ax::Secondary>,
    axis_marker: PhantomData<Ax>
}
impl<Ax: Axis> Default for ShearingProportion<Ax> where Ax::Primary: Axis, Ax::Secondary: Axis {
    fn default() -> Self {
        ShearingProportion::new(f32::default(), f32::default())
    }
}
impl<Ax: Axis> ShearingProportion<Ax> {
    pub fn new(to_primary: f32, to_secondary: f32) -> ShearingProportion<Ax>
    where 
        Ax::Primary: Axis, 
        Ax::Secondary: Axis
    {
        ShearingProportion {
            ratio_to_primary: ShearOf::new(to_primary),
            ratio_to_secondary: ShearOf::new(to_secondary),
            axis_marker: PhantomData
        }
    }
}
#[derive(Debug)]
pub struct Matr4(f32x16);
impl AsArray<f32, 16> for Matr4 {
    fn as_array(&self) -> &[f32; 16] {
        self.0.as_array()
    }
}
impl FromArray<f32, 16> for Matr4 {
    #[inline]
    fn from_array(array: [f32; 16]) -> Matr4 {
        Matr4(f32x16::from_array(array))
    }
}
impl FromSlice<f32> for Matr4 {
    #[inline]
    fn from_slice(slice: &[f32]) -> Matr4 {
        Matr4(f32x16::from_slice(slice))
    }
}
impl Matr4 {
    #[inline]
    pub const fn translation(tx: f32, ty: f32, tz: f32) -> Matr4 {
        #[rustfmt::skip]
        mod inner {
            use super::{Matr4, f32x16};
            pub const fn create(tx: f32, ty: f32, tz: f32) -> Matr4 {
                Matr4(f32x16::from_array([
                    1., 0., 0., tx, 
                    0., 1., 0., ty,
                    0., 0., 1., tz,
                    0., 0., 0., 1.
                ]))
            }
        }
        inner::create(tx, ty, tz)
    }
    #[inline]
    pub const fn scaling(sx: f32, sy: f32, sz: f32) -> Matr4 {

        #[rustfmt::skip]
        mod inner {
            use super::{Matr4, f32x16};
            pub const fn create(sx: f32, sy: f32, sz: f32) -> Matr4 {
                Matr4(f32x16::from_array([
                    sx, 0., 0., 0., 
                    0., sy, 0., 0.,
                    0., 0., sz, 0.,
                    0., 0., 0., 1.
                ]))
            }
        }
        inner::create(sx, sy, sz)
    }
    #[inline]
    pub fn rotation_x_rad(rad: f32) -> Matr4 {
        #[rustfmt::skip]
        mod inner {
            use super::{Matr4, f32x16};
            pub const fn create(yy: f32, yz: f32, zy: f32, zz: f32) -> Matr4 {
                Matr4(f32x16::from_array([
                    1., 0., 0., 0., 
                    0., yy, yz, 0.,
                    0., zy, zz, 0.,
                    0., 0., 0., 1.
                ]))
            }
        }
        let rotation: RotationAbout<X> = RotationAbout::new_rad(rad);
        let axes = rotation.into_axes();
        inner::create(axes.yy(), axes.yz(), axes.zy(), axes.zz())
    }
    #[inline]
    pub fn rotation_x_deg(deg: f32) -> Matr4 {
        Matr4::rotation_x_rad(deg.to_radians())
    }

    #[inline]
    pub fn rotation_y_rad(rad: f32) -> Matr4 {
        let rotation: RotationAbout<Y> = RotationAbout::new_rad(rad);
        let axes = rotation.into_axes();

        #[rustfmt::skip]
        mod inner {
            use super::{Matr4, f32x16};
            #[inline]
            pub fn create(xx: f32, xz: f32, zx: f32, zz: f32) -> Matr4 {
                let array = 
                [
                    xx, 0., xz, 0., 
                    0., 1., 0., 0.,
                    zx, 0., zz, 0.,
                    0., 0., 0., 1.
                ];
                Matr4(f32x16::from_array(array))
            }
        }
        inner::create(axes.xx(), axes.xz(), axes.zx(), axes.zz())
    }
    #[inline]
    pub fn rotation_y_deg(deg: f32) -> Matr4 {
        Matr4::rotation_y_rad(deg.to_radians())
    }
    #[inline]
    pub fn rotation_z_rad(rad: f32) -> Matr4 {
        let rotation: RotationAbout<Z> = RotationAbout::new_rad(rad);
        let axes = rotation.into_axes();

        #[rustfmt::skip]
        mod inner {
            use super::{Matr4, f32x16};
            #[inline]
            pub fn create(xx: f32, xy: f32, yx: f32, yy: f32) -> Matr4 {
                let array = 
                [
                    xx, xy, 0., 0., 
                    yx, yy, 0., 0.,
                    0., 0., 1., 0.,
                    0., 0., 0., 1.
                ];
                Matr4(f32x16::from_array(array))
            }
        }
        inner::create(axes.xx(), axes.xy(), axes.yx(), axes.yy())
    }
    #[inline]
    pub fn rotation_z_deg(deg: f32) -> Matr4 {
        Matr4::rotation_z_rad(deg.to_radians())
    }
    #[inline]
    pub const fn shearing(
        ShearingProportion {
            ratio_to_primary: ShearOf(xy, _xy),
            ratio_to_secondary: ShearOf(xz, _xz),
            ..
        }: ShearingProportion<X>, 
        ShearingProportion {
            ratio_to_primary: ShearOf(yx, _yx),
            ratio_to_secondary: ShearOf(yz, _yz),
            ..
        }: ShearingProportion<Y>,
        ShearingProportion {
            ratio_to_primary: ShearOf(zx, _zx),
            ratio_to_secondary: ShearOf(zy, _zy),
            ..
        }: ShearingProportion<Z>) -> Matr4 {
        #[rustfmt::skip]
        mod inner {
            use super::{Matr4, f32x16};
            #[inline]
            pub const fn create(xy: f32, xz: f32, yx: f32, yz: f32, zx: f32, zy: f32) -> Matr4 {
                let array = [
                    1., xy, xz, 0., 
                    yx, 1., yz, 0.,
                    zx, zy, 1., 0.,
                    0., 0., 0., 1.
                ];
                Matr4(f32x16::from_array(array))
            }
        }
        inner::create(xy, xz, yx, yz, zx, zy)
    }
}
pub(crate) const T_SWIZZLE_4: [usize; 16] = [0, 4, 8, 12, 1, 5, 9, 13, 2, 6, 10, 14, 3, 7, 11, 15];
impl Matrix<4> for Matr4 {
    type Vert = Vert4;
    
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
impl Index<(usize, usize)> for Matr4 {
    type Output = f32;
    #[inline]
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.0[index.1 + (4 * index.0)]
    }
}
impl IndexMut<(usize, usize)> for Matr4 {
    #[inline]
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.0[index.1 + (4 * index.0)]
    }
}
// impl From<Mat4> for [Vert4; 4] {}
impl From<Matr4> for [Vert4; 4] {
    #[inline]
    fn from(value: Matr4) -> Self {
        let v0 = Vert4::new(value[(0, 0)], value[(0, 1)], value[(0, 2)], value[(0, 3)]);
        let v1 = Vert4::new(value[(1, 0)], value[(1, 1)], value[(1, 2)], value[(1, 3)]);
        let v2 = Vert4::new(value[(2, 0)], value[(2, 1)], value[(2, 2)], value[(2, 3)]);
        let v3 = Vert4::new(value[(3, 0)], value[(3, 1)], value[(3, 2)], value[(3, 3)]);
        [v0, v1, v2, v3]
    }
}
impl From<&Matr4> for [Vert4; 4] {
    #[inline]
    fn from(value: &Matr4) -> Self {
        let v0 = Vert4::new(value[(0, 0)], value[(0, 1)], value[(0, 2)], value[(0, 3)]);
        let v1 = Vert4::new(value[(1, 0)], value[(1, 1)], value[(1, 2)], value[(1, 3)]);
        let v2 = Vert4::new(value[(2, 0)], value[(2, 1)], value[(2, 2)], value[(2, 3)]);
        let v3 = Vert4::new(value[(3, 0)], value[(3, 1)], value[(3, 2)], value[(3, 3)]);
        [v0, v1, v2, v3]
    }
}
impl AsColumns<4> for Matr4 {}
impl Mul<Matr4> for Matr4 {
    type Output = Matr4;
    #[inline]
    fn mul(self, rhs: Matr4) -> Self::Output {
        // M[(r, c)] = A[(r, 0)] * B[(0, c)] (= B[0 + 4c])
        //           + A[(r, 1)] * B[(1, c)]
        //           + A[(r, 2)] * B[(2, c)]
        //           + A[(r, 3)] * B[(3, c)]
        let mut output: Matr4 = Matr4::identity();
        let rows = self.as_row_vectors();
        let cols = rhs.as_column_vectors();
        for (row_idx, row) in rows.iter().enumerate() {
            for (col_idx, col) in cols.iter().enumerate() {
                output[(row_idx, col_idx)] = (row * col).reduce_sum();
            }
        }
        output
    }
}
impl Mul<&Matr4> for Matr4 {
    type Output = Matr4;
    #[inline]
    fn mul(self, rhs: &Matr4) -> Self::Output {
        // M[(r, c)] = A[(r, 0)] * B[(0, c)] (= B[0 + 4c])
        //           + A[(r, 1)] * B[(1, c)]
        //           + A[(r, 2)] * B[(2, c)]
        //           + A[(r, 3)] * B[(3, c)]
        let mut output: Matr4 = Matr4::identity();
        let rows = self.as_row_vectors();
        let cols = rhs.as_column_vectors();
        for (row_idx, row) in rows.iter().enumerate() {
            for (col_idx, col) in cols.iter().enumerate() {
                output[(row_idx, col_idx)] = (row * col).reduce_sum();
            }
        }
        output
    }
}
impl Mul<Matr4> for &Matr4 {
    type Output = Matr4;
    #[inline]
    fn mul(self, rhs: Matr4) -> Self::Output {
        // M[(r, c)] = A[(r, 0)] * B[(0, c)] (= B[0 + 4c])
        //           + A[(r, 1)] * B[(1, c)]
        //           + A[(r, 2)] * B[(2, c)]
        //           + A[(r, 3)] * B[(3, c)]
        let mut output: Matr4 = Matr4::identity();
        let rows = self.as_row_vectors();
        let cols = rhs.as_column_vectors();
        for (row_idx, row) in rows.iter().enumerate() {
            for (col_idx, col) in cols.iter().enumerate() {
                output[(row_idx, col_idx)] = (row * col).reduce_sum();
            }
        }
        output
    }
}
impl Mul<&Matr4> for &Matr4 {
    type Output = Matr4;
    #[inline]
    fn mul(self, rhs: &Matr4) -> Self::Output {
        // M[(r, c)] = A[(r, 0)] * B[(0, c)] (= B[0 + 4c])
        //           + A[(r, 1)] * B[(1, c)]
        //           + A[(r, 2)] * B[(2, c)]
        //           + A[(r, 3)] * B[(3, c)]
        let mut output: Matr4 = Matr4::identity();
        let rows = self.as_row_vectors();
        let cols = rhs.as_column_vectors();
        for (row_idx, row) in rows.iter().enumerate() {
            for (col_idx, col) in cols.iter().enumerate() {
                output[(row_idx, col_idx)] = (row * col).reduce_sum();
            }
        }
        output
    }
}
impl Mul<Vert4> for Matr4 {
    type Output = Vert4;
    #[inline]
    fn mul(self, rhs: Vert4) -> Self::Output {
        // let array = core::array::from_fn(|y_idx| 
        //     {
        //         let y_offset = y_idx * 4;
        //         // let row_slice: &[f32] = unsafe { <f32x16 as AsRef<[f32]>>::as_ref(&self.0).get_unchecked(y_offset..y_offset+4) };
        //         let p_elt = unsafe { self.0.as_array().as_ptr().add(y_offset) };
        //         // let row_slice = unsafe {core::slice::from_raw_parts(p_elt, 4)};
        //         let row_array = unsafe {[
        //             p_elt.add(0).read() * rhs.0[0],
        //             p_elt.add(1).read() * rhs.0[1],
        //             p_elt.add(2).read() * rhs.0[2],
        //             p_elt.add(3).read() * rhs.0[3],
        //         ]};
        //         // let sum_of_rows_array = core::array::from_fn(|x_idx| {
        //         //     self.0[y_offset + x_idx]
        //         // });
        //         f32x4::from_array(row_array).reduce_sum()
        //     }
        // );
        let arr: &[f32; 16] = self.0.as_array();

        let arr_0 =Vert4::from_slice(unsafe {arr.get_unchecked(0..4)}) * &rhs;
        let arr_1 = Vert4::from_slice(unsafe {arr.get_unchecked(4..8)}) * &rhs;
        let arr_2 = Vert4::from_slice(unsafe {arr.get_unchecked(8..12)}) * &rhs;
        let arr_3 = Vert4::from_slice(unsafe {arr.get_unchecked(12..16)}) * &rhs;
        Vert4::from_array([
                    arr_0.reduce_sum(),
                    arr_1.reduce_sum(),
                    arr_2.reduce_sum(),
                    arr_3.reduce_sum(),
        ])
    }
}
impl Mul<&Vert4> for Matr4 {
    type Output = Vert4;
    #[inline]
    fn mul(self, rhs: &Vert4) -> Self::Output {
        let mut output: Vert4 = Vert4::new(0.0, 0.0, 0.0, 0.0);
        let rows = self.as_row_vectors();
        for (idx, row) in rows.iter().enumerate() {
            output[idx] = (row * rhs).reduce_sum();
        }
        output
    }
}
impl Mul<Vert4> for &Matr4 {
    type Output = Vert4;
    #[inline]
    fn mul(self, rhs: Vert4) -> Self::Output {
        let mut output: Vert4 = Vert4::new(0.0, 0.0, 0.0, 0.0);
        let rows = self.as_row_vectors();
        for (idx, row) in rows.iter().enumerate() {
            output[idx] = (row * &rhs).reduce_sum();
        }
        output
    }
}
impl Mul<&Vert4> for &Matr4 {
    type Output = Vert4;
    #[inline]
    fn mul(self, rhs: &Vert4) -> Self::Output {
        let mut output: Vert4 = Vert4::new(0.0, 0.0, 0.0, 0.0);
        let rows = self.as_row_vectors();
        for (idx, row) in rows.iter().enumerate() {
            output[idx] = (row * rhs).reduce_sum();
        }
        output
    }
}
impl PartialEq for Matr4 {
    fn eq(&self, other: &Self) -> bool {
        self.as_row_vectors()
            .iter()
            .zip(other.as_row_vectors().iter())
            .all(|(lhs, rhs)|PartialEq::eq(lhs, rhs))
    }
}
