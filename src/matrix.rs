use core::marker::PhantomData;
use core::ops::{Index, IndexMut};
use core::ops::Mul;

use std::simd::{f32x16, f32x4, num::SimdFloat, simd_swizzle};

use crate::{Vert2, Vert3, Vert4};

mod ops;
pub use ops::{Cofactor, Determinant, Minor, Submatrix};

pub trait Matrix<const DIM: usize> {
    type Vert;
    fn new(array: [f32; DIM * DIM]) -> Self;
    fn identity() -> Self;
    fn transpose(&self) -> Self;
    #[inline]
    fn as_column_vectors<'a>(&'a self) -> [Self::Vert; DIM]
    where
        Self: 'a + Into<[Self::Vert; DIM]>,
    {
        self.transpose().into()
    }
    #[inline]
    fn as_row_vectors<'a>(&'a self) -> [Self::Vert; DIM]
    where
        Self: 'a + Into<[Self::Vert; DIM]>,
        &'a Self: Into<[Self::Vert; DIM]>,
    {
        self.into()
    }
}

#[derive(Debug, PartialEq)]
pub struct Mat2(pub(crate) f32x4);
pub(crate) const T_SWIZZLE_2: [usize; 4] = [0, 2, 1, 3];
impl Matrix<2> for Mat2 {
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
impl Matrix<3> for Mat3 {
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
#[cfg(test)]
mod transformation_tests {
    use super::*;
    //
    // Translation tests
    //
    #[test]
    fn multiplying_by_a_translation_matrix() {
        let transform = Matr4::translation(5., -3., 2.);
        let p = Vert4::new(-3., 4., 5., 1.);

        let res = transform * p;
        assert_eq!(res, Vert4::new(2., 1., 7., 1.))
    }
    #[test]
    fn multiplying_by_the_inverse_of_a_translation_matrix() {
        use crate::matrix::ops::Inverse;

        let transform = Matr4::translation(5., -3., 2.);
        let p = Vert4::point(-3., 4., 5.);

        let inv = transform.inverse().unwrap();

        let res = inv * p;
        assert_eq!(res, Vert4::point(-8., 7., 3.))
    }
    #[test]
    fn translation_does_not_affect_vectors() {
        use crate::Vert4;

        let transform = Matr4::translation(5., -3., 2.);
        let v = Vert4::vector(-3., 4., 5.);
        let res = transform * &v;
        assert_eq!(res, v)
    }
    //
    // Scaling tests
    //
    #[test]
    fn a_scaling_matrix_applied_to_a_point() {
        let transform = Matr4::scaling(2., 3., 4.);
        let p = Vert4::point(-4., 6., 8.);
        let res = transform * p;
        assert_eq!(res, Vert4::point(-8., 18., 32.))
    }
    #[test]
    fn a_scaling_matrix_applied_to_a_vector() {
        let transform = Matr4::scaling(2., 3., 4.);
        let v = Vert4::vector(-4., 6., 8.);
        let res = transform * v;
        assert_eq!(res, Vert4::vector(-8., 18., 32.))
    }
    #[test]
    fn multiplying_by_the_inverse_of_a_scaling_matrix() {
        use crate::matrix::ops::Inverse;

        let transform = Matr4::scaling(2., 3., 4.);
        let inv = transform.inverse().unwrap();
        let v = Vert4::vector(-4., 6., 8.);
        let res = inv * v;
        assert_eq!(res,Vert4::vector(-2., 2., 2.))
    }
    #[test]
    fn reflection_is_scaling_by_a_negative_value() {
        let transform = Matr4::scaling(-1., 1., 1.);
        let p = Vert4::point(2., 3., 4.);
        let res = transform * p;
        assert_eq!(res, Vert4::point(-2., 3., 4.))
    }
    //
    // Rotation tests
    //
    #[test]
    fn rotating_a_point_around_the_x_axis() {
        use core::f32::consts::PI;

        let p = Vert4::point(0., 1., 0.);

        let half_quarter = Matr4::rotation_x_rad(PI / 4.);
        let half_rotp = half_quarter * &p;
        assert_eq!(half_rotp, Vert4::point(0., 2_f32.sqrt() / 2., 2_f32.sqrt() / 2.));

        let full_quarter = Matr4::rotation_x_rad(PI / 2.);
        let full_rotp = full_quarter *&p;
        assert_eq!(full_rotp, Vert4::point(0., 0., 1.))
    }
    #[test]
    fn the_inverse_of_an_x_rotation_rotates_in_the_oppsite_direction() {
        use crate::matrix::ops::Inverse;
        use core::f32::consts::PI;

        let p = Vert4::point(0., 1., 0.);

        let half_quarter = Matr4::rotation_x_rad(PI / 4.);
        let inv = half_quarter.inverse().unwrap();
        let half_rotp = inv * &p;
        assert_eq!(half_rotp, Vert4::point(0., 2_f32.sqrt() / 2., -(2_f32.sqrt() / 2.)));
    }
    #[test]
    fn rotating_a_point_around_the_y_axis() {
        use core::f32::consts::PI;

        let p = Vert4::point(0., 0., 1.);

        let half_quarter = Matr4::rotation_y_rad(PI / 4.);
        let half_rotp = half_quarter * &p;
        assert_eq!(half_rotp, Vert4::point(2_f32.sqrt() / 2., 0., 2_f32.sqrt() / 2.));

        let full_quarter = Matr4::rotation_y_rad(PI / 2.);
        let full_rotp = full_quarter *&p;
        assert_eq!(full_rotp, Vert4::point(1., 0., 0.))
    }
    #[test]
    fn rotating_a_point_around_the_z_axis() {
        use core::f32::consts::PI;

        let p = Vert4::point(0., 1., 0.);

        let half_quarter = Matr4::rotation_z_rad(PI / 4.);
        let half_rotp = half_quarter * &p;
        assert_eq!(half_rotp, Vert4::point(-2_f32.sqrt() / 2., 2_f32.sqrt() / 2., 0.));

        let full_quarter = Matr4::rotation_z_rad(PI / 2.);
        let full_rotp = full_quarter *&p;
        assert_eq!(full_rotp, Vert4::point(-1., 0., 0.))
    }
    //
    // Shearing tests
    //
    #[test]
    fn a_shearing_transformation_moves_x_in_proportion_to_y() {
        let transform = Matr4::shearing(ShearingProportion::new(1., 0.), ShearingProportion::default(), ShearingProportion::default());
        let p = Vert4::point(2., 3., 4.);
        assert_eq!(transform * p, Vert4::point(5., 3., 4.))
    }
    #[test]
    fn a_shearing_transformation_moves_x_in_proportion_to_z() {
        let transform = Matr4::shearing(ShearingProportion::new(0., 1.), ShearingProportion::default(), ShearingProportion::default());
        let p = Vert4::point(2., 3., 4.);
        assert_eq!(transform * p, Vert4::point(6., 3., 4.))
    }
    #[test]
    fn a_shearing_transformation_moves_y_in_proportion_to_x() {
        let transform = Matr4::shearing(ShearingProportion::default(), ShearingProportion::new(1., 0.), ShearingProportion::default());
        let p = Vert4::point(2., 3., 4.);
        assert_eq!(transform * p, Vert4::point(2., 5., 4.))
    }
    #[test]
    fn a_shearing_transformation_moves_y_in_proportion_to_z() {
        let transform = Matr4::shearing(ShearingProportion::default(), ShearingProportion::new(0., 1.), ShearingProportion::default());
        let p = Vert4::point(2., 3., 4.);
        assert_eq!(transform * p, Vert4::point(2., 7., 4.))
    }
    #[test]
    fn a_shearing_transformation_moves_z_in_proportion_to_x() {
        let transform = Matr4::shearing(ShearingProportion::default(), ShearingProportion::default(), ShearingProportion::new(1., 0.));
        let p = Vert4::point(2., 3., 4.);
        assert_eq!(transform * p, Vert4::point(2., 3., 6.))
    }
    #[test]
    fn a_shearing_transformation_moves_z_in_proportion_to_y() {
        let transform = Matr4::shearing(ShearingProportion::default(), ShearingProportion::default(), ShearingProportion::new(0., 1.));
        let p = Vert4::point(2., 3., 4.);
        assert_eq!(transform * p, Vert4::point(2., 3., 7.))
    }
}
pub(crate) const T_SWIZZLE_4: [usize; 16] = [0, 4, 8, 12, 1, 5, 9, 13, 2, 6, 10, 14, 3, 7, 11, 15];
impl Matrix<4> for Matr4 {
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
                output[(row_idx, col_idx)] = (row.0 * col.0).reduce_sum();
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
                output[(row_idx, col_idx)] = (row.0 * col.0).reduce_sum();
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
                output[(row_idx, col_idx)] = (row.0 * col.0).reduce_sum();
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
                output[(row_idx, col_idx)] = (row.0 * col.0).reduce_sum();
            }
        }
        output
    }
}
impl Mul<Vert4> for Matr4 {
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
impl Mul<&Vert4> for Matr4 {
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
impl Mul<Vert4> for &Matr4 {
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
impl Mul<&Vert4> for &Matr4 {
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
impl PartialEq for Matr4 {
    fn eq(&self, other: &Self) -> bool {
        self.as_row_vectors()
            .iter()
            .zip(other.as_row_vectors().iter())
            .all(|(lhs, rhs)|PartialEq::eq(lhs, rhs))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn constructing_mat4() {
        let m = Matr4(f32x16::from_array([
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
            let a = Matr4(f32x16::from_array([
                1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
            ]));
            let b = Matr4(f32x16::from_array([
                1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
            ]));
            assert_eq!(a, b)
        }
        #[test]
        fn inequality() {
            let a = Matr4(f32x16::from_array([
                1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
            ]));
            let b = Matr4(f32x16::from_array([
                2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0,
            ]));
            assert_ne!(a, b)
        }
        mod mul {
            use super::*;
            #[test]
            fn multiplication_by_matrix() {
                let a = Matr4(f32x16::from_array([
                    1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
                ]));
                let b = Matr4(f32x16::from_array([
                    -2.0, 1.0, 2.0, 3.0, 3.0, 2.0, 1.0, -1.0, 4.0, 3.0, 6.0, 5.0, 1.0, 2.0, 7.0,
                    8.0,
                ]));
                assert_eq!(
                    a * b,
                    Matr4(f32x16::from_array([
                        20.0, 22.0, 50.0, 48.0, 44.0, 54.0, 114.0, 108.0, 40.0, 58.0, 110.0, 102.0,
                        16.0, 26.0, 46.0, 42.0
                    ]))
                )
            }
            #[test]
            fn multiplication_by_vertex() {
                let a = Matr4(f32x16::from_array([
                    1.0, 2.0, 3.0, 4.0, 2.0, 4.0, 4.0, 2.0, 8.0, 6.0, 4.0, 1.0, 0.0, 0.0, 0.0, 1.0,
                ]));
                let b = Vert4::new(1.0, 2.0, 3.0, 1.0);
                assert_eq!(a * b, Vert4::new(18.0, 24.0, 33.0, 1.0))
            }
            #[test]
            fn multiplication_by_ident() {
                let a = Matr4(f32x16::from_array([
                    0.0, 1.0, 2.0, 4.0, 1.0, 2.0, 4.0, 8.0, 2.0, 4.0, 8.0, 16.0, 4.0, 8.0, 16.0,
                    32.0,
                ]));
                assert_eq!(&a * Matr4::identity(), a)
            }
            #[test]
            fn multiplication_of_vertex_by_ident() {
                let a = Vert4::new(1.0, 2.0, 3.0, 4.0);
                assert_eq!(Matr4::identity() * &a, a)
            }
        }
        #[test]
        fn transposition() {
            let a = Matr4(f32x16::from_array([
                0.0, 9.0, 3.0, 0.0, 9.0, 8.0, 0.0, 8.0, 1.0, 8.0, 5.0, 3.0, 0.0, 0.0, 5.0, 8.0,
            ]));
            assert_eq!(
                a.transpose(),
                Matr4(f32x16::from_array([
                    0.0, 9.0, 1.0, 0.0, 9.0, 8.0, 8.0, 0.0, 3.0, 0.0, 5.0, 5.0, 0.0, 8.0, 3.0, 8.0
                ]))
            )
        }
        #[test]
        fn transposition_of_ident() {
            let a = Matr4::identity();
            assert_eq!(a.transpose(), Matr4::identity())
        }
    }
}
