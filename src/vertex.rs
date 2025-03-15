use core::fmt::{Debug, Formatter, Result as FmtResult};

use core::simd::num::SimdFloat;
use core::simd::{LaneCount, SupportedLaneCount};
use core::simd::{Simd, f32x2, f32x4, simd_swizzle};

use core::ops::{Deref, DerefMut};

use core::ops::Neg;
use core::ops::{Add, AddAssign};
use core::ops::{Div, DivAssign};
use core::ops::{Mul, MulAssign};
use core::ops::{Sub, SubAssign};

const SIMD_2_ZERO: Simd<f32, 2> = Simd::from_array([0.0_f32, 0.0_f32]);
const SIMD_2_X: Simd<f32, 2> = Simd::from_array([1.0_f32, 0.0_f32]);
const SIMD_2_Y: Simd<f32, 2> = Simd::from_array([0.0_f32, 1.0_f32]);

pub const fn float_almost_eq(lhs: &f32, rhs: &f32) -> bool {
    (f32::max(*lhs, *rhs) - f32::min(*lhs, *rhs)) < 1e-4
}
pub fn float_array_almost_eq<const N: usize>(lhs: &[f32; N], rhs: &[f32; N]) -> bool {
    lhs.iter()
        .zip(rhs)
        .all(|(lhs, rhs)| float_almost_eq(lhs, rhs))
}
pub fn simd_almost_eq<const N: usize>(lhs: &Simd<f32, N>, rhs: &Simd<f32, N>) -> bool
where
    LaneCount<N>: SupportedLaneCount,
{
    Simd::lt(&SimdFloat::abs(lhs - rhs), &Simd::splat(1e-4))
}

#[repr(transparent)]
#[derive(Debug)]
pub struct Vert2(f32x2);

impl Deref for Vert2 {
    type Target = [f32];
    fn deref(&self) -> &Self::Target {
        self.0.as_array().as_ref()
    }
}
impl Vert2 {
    pub const ZERO: Self = Self(SIMD_2_ZERO);
    pub const X: Self = Self(SIMD_2_X);
    pub const Y: Self = Self(SIMD_2_Y);
    #[inline]
    pub const fn new(x: f32, y: f32) -> Self {
        Self(f32x2::from_array([x, y]))
    }
}
impl From<f32x2> for Vert2 {
    #[inline]
    fn from(value: f32x2) -> Self {
        Self(value)
    }
}
impl From<&f32x2> for Vert2 {
    #[inline]
    fn from(value: &f32x2) -> Self {
        Self(*value)
    }
}

const SIMD_3_ZERO: [f32; 3] = [0.0_f32, 0.0_f32, 0.0_f32];
const SIMD_3_X: [f32; 3] = [1.0_f32, 0.0_f32, 0.0_f32];
const SIMD_3_Y: [f32; 3] = [0.0_f32, 1.0_f32, 0.0_f32];
const SIMD_3_Z: [f32; 3] = [0.0_f32, 0.0_f32, 1.0_f32];

#[repr(transparent)]
pub struct Vert3([f32; 3]);

impl Deref for Vert3 {
    type Target = [f32];
    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}
impl Debug for Vert3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_tuple("Vert3").field(&self.0).finish()
    }
}
impl PartialEq for Vert3 {
    fn eq(&self, other: &Self) -> bool {
        float_array_almost_eq(&self.0, &other.0)
    }
}
impl Vert3 {
    pub const ZERO: Self = Self(SIMD_3_ZERO);
    pub const X: Self = Self(SIMD_3_X);
    pub const Y: Self = Self(SIMD_3_Y);
    pub const Z: Self = Self(SIMD_3_Z);
    #[inline]
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self([x, y, z])
    }
}
impl From<[f32; 3]> for Vert3 {
    #[inline]
    fn from(value: [f32; 3]) -> Self {
        Self(value)
    }
}
impl From<&[f32; 3]> for Vert3 {
    #[inline]
    fn from(value: &[f32; 3]) -> Self {
        Self(*value)
    }
}

const SIMD_4_ZERO: Simd<f32, 4> = Simd::from_array([0.0_f32, 0.0_f32, 0.0_f32, 0.0_f32]);
const SIMD_4_X: Simd<f32, 4> = Simd::from_array([1.0_f32, 0.0_f32, 0.0_f32, 0.0_f32]);
const SIMD_4_Y: Simd<f32, 4> = Simd::from_array([0.0_f32, 1.0_f32, 0.0_f32, 0.0_f32]);
const SIMD_4_Z: Simd<f32, 4> = Simd::from_array([0.0_f32, 0.0_f32, 1.0_f32, 0.0_f32]);
const SIMD_4_W: Simd<f32, 4> = Simd::from_array([0.0_f32, 0.0_f32, 0.0_f32, 1.0_f32]);

#[repr(transparent)]
#[derive(Debug)]
pub struct Vert4(f32x4);

impl Deref for Vert4 {
    type Target = [f32];
    fn deref(&self) -> &Self::Target {
        self.0.as_array().as_ref()
    }
}
impl DerefMut for Vert4 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.as_mut_array().as_mut()
    }
}
impl Vert4 {
    pub const ZERO: Self = Self(SIMD_4_ZERO);
    pub const X: Self = Self(SIMD_4_X);
    pub const Y: Self = Self(SIMD_4_Y);
    pub const Z: Self = Self(SIMD_4_Z);
    pub const W: Self = Self(SIMD_4_W);
    #[inline]
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self(f32x4::from_array([x, y, z, w]))
    }
    #[inline]
    pub fn x(&self) -> f32 {
        self.0[0]
    }
    #[inline]
    pub fn y(&self) -> f32 {
        self.0[1]
    }
    #[inline]
    pub fn z(&self) -> f32 {
        self.0[2]
    }
    #[inline]
    pub const fn w(&self) -> f32 {
        self.0.as_array()[3]
    }
    #[inline]
    pub const fn point(x: f32, y: f32, z: f32) -> Vert4 {
        Vert4::new(x, y, z, 1.0)
    }
    #[inline]
    pub const fn vector(x: f32, y: f32, z: f32) -> Vert4 {
        Vert4::new(x, y, z, 0.0)
    }
    #[inline]
    pub const fn is_point(&self) -> bool {
        self.w() == 1.
    }
    #[inline]
    pub const fn is_vector(&self) -> bool {
        !self.is_point()
    }
    #[inline]
    pub const fn from_slice(slice: &[f32]) -> Vert4 {
        Vert4(f32x4::from_slice(slice))
    }

    #[inline]
    pub const fn from_array(array: [f32; 4]) -> Vert4 {
        Vert4(f32x4::from_array(array))
    }
    #[inline]
    pub fn reduce_sum(self) -> f32 {
        self.0.reduce_sum()
    }
}
// From impls---------
impl From<f32x4> for Vert4 {
    #[inline]
    fn from(value: f32x4) -> Self {
        Self(value)
    }
}
impl From<&f32x4> for Vert4 {
    #[inline]
    fn from(value: &f32x4) -> Self {
        Self(value.to_owned())
    }
}
#[expect(unused_macros)]
macro_rules! imp {
    (Add, Self = $t:ty $(, Rhs = $rhs:ty)?$(, Output = $o:ty)?$(, Ex = $e:expr)?) => {
        add_impl!(Self = $t$(, Rhs = $rhs)?$(, Output = $o)?$(, Ex = $e)?);
    };
    (AddAssign, Self = $t:ty$(, Rhs = $rhs:ty)?, Ex = $e:expr) => {
        add_assign_impl!(Self = $t$(, Rhs = $rhs)?, Ex = $e);
    };
}
#[expect(unused_macros)]
macro_rules! add_assign_impl {
    (Self = $t:ty, Rhs = $rhs:ty, Ex = $e:expr) => {
        impl AddAssign<$rhs> for $t {
            #[inline]
            fn add_assign(&mut self, rhs: $rhs) {
                $e
            }
        }
    };
    (Self = $t:ty, Ex = $e:expr) => {
        impl AddAssign for $t {
            #[inline]
            fn add_assign(&mut self, rhs: $t) {
                $e
            }
        }
    };
}
// Add impls---------
#[expect(unused_macros)]
macro_rules! add_impl {
    (Self = $t:ty, Ex = $e:expr) => {
        add_impl!(Self = $t, Rhs = $t, Output = $t, Ex = $e);
    };
    (Self = $t:ty) => {
        add_impl!(Self = $t, Rhs = $t, Output = $t);
    };
    (Self = $t:ty, Rhs = $rhs:ty, Ex = $e:expr) => {
        add_impl!(Self = $t, Rhs = $rhs, Output = $t, Ex = $e);
    };
    (Self = $t:ty, Rhs = $rhs:ty) => {
        add_impl!(Self = $t, Rhs = $rhs, Output = $t);
    };
    (Self = $t:ty, Rhs = $rhs:ty, Output = $o:ty) => {
        impl Add<$rhs> for $t {
            type Output = $o;
            #[inline]
            fn add(self, rhs: $rhs) -> Self::Output {
                self.0.add(rhs.0).into()
            }
        }
    };
    (Self = $t:ty, Rhs = $rhs:ty, Output = $o:ty, Ex = $e:expr) => {
        impl Add<$rhs> for $t {
            type Output = $o;
            #[inline]
            fn add(self, rhs: $rhs) -> Self::Output {
                self.0.add(rhs.0).into()
            }
        }
    };
}
impl Add for Vert4 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        self.0.add(rhs.0).into()
    }
}
impl AddAssign for Vert4 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.0.add_assign(rhs.0)
    }
}
impl Add<&Self> for Vert4 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: &Self) -> Self::Output {
        self.0.add(rhs.0).into()
    }
}
impl AddAssign<&Self> for Vert4 {
    #[inline]
    fn add_assign(&mut self, rhs: &Self) {
        self.0.add_assign(rhs.0)
    }
}
impl Add for &Vert4 {
    type Output = Vert4;
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        self.0.add(rhs.0).into()
    }
}
impl Add<&Self> for &Vert4 {
    type Output = Vert4;
    #[inline]
    fn add(self, rhs: &Self) -> Self::Output {
        self.0.add(rhs.0).into()
    }
}
impl Sub for Vert4 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        self.0.sub(rhs.0).into()
    }
}
impl SubAssign for Vert4 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.0.sub_assign(rhs.0)
    }
}
impl Sub<&Self> for Vert4 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: &Self) -> Self::Output {
        self.0.sub(rhs.0).into()
    }
}
impl SubAssign<&Self> for Vert4 {
    #[inline]
    fn sub_assign(&mut self, rhs: &Self) {
        self.0.sub_assign(rhs.0)
    }
}
impl Sub<&Self> for &Vert4 {
    type Output = Vert4;
    #[inline]
    fn sub(self, rhs: &Self) -> Self::Output {
        self.0.sub(rhs.0).into()
    }
}
impl Neg for Vert4 {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self::Output {
        self.0.neg().into()
    }
}
impl Neg for &Vert4 {
    type Output = Vert4;
    #[inline]
    fn neg(self) -> Self::Output {
        self.0.neg().into()
    }
}
impl Mul<f32> for Vert4 {
    type Output = Vert4;
    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        self.0.mul(simd_4!(rhs)).into()
    }
}
impl Mul<f32> for &Vert4 {
    type Output = Vert4;
    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        self.0.mul(simd_4!(rhs)).into()
    }
}
impl Mul<&f32> for Vert4 {
    type Output = Vert4;
    #[inline]
    fn mul(self, rhs: &f32) -> Self::Output {
        self.0.mul(simd_4!(*rhs)).into()
    }
}
impl MulAssign for Vert4 {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        self.0.mul_assign(rhs.0)
    }
}
impl Mul<Vert4> for &Vert4 {
    type Output = Vert4;
    #[inline]
    fn mul(self, rhs: Vert4) -> Self::Output {
        self.0.mul(rhs.0).into()
    }
}
impl Mul<&Self> for Vert4 {
    type Output = Vert4;
    #[inline]
    fn mul(self, rhs: &Self) -> Self::Output {
        self.0.mul(rhs.0).into()
    }
}
impl MulAssign<&Self> for Vert4 {
    #[inline]
    fn mul_assign(&mut self, rhs: &Self) {
        self.0.mul_assign(rhs.0)
    }
}
impl Mul<Self> for &Vert4 {
    type Output = Vert4;
    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        self.0.mul(rhs.0).into()
    }
}
impl Div<f32> for Vert4 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        self.0.div(simd_4!(rhs)).into()
    }
}
impl DivAssign<f32> for Vert4 {
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        self.0.div_assign(simd_4!(rhs))
    }
}
impl Div<f32> for &Vert4 {
    type Output = Vert4;
    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        self.0.div(simd_4!(rhs)).into()
    }
}
impl Div<&f32> for Vert4 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: &f32) -> Self::Output {
        self.0.div(simd_4!(*rhs)).into()
    }
}
impl DivAssign<&f32> for Vert4 {
    #[inline]
    fn div_assign(&mut self, rhs: &f32) {
        self.0.div_assign(simd_4!(*rhs))
    }
}
impl Div<&f32> for &Vert4 {
    type Output = Vert4;
    #[inline]
    fn div(self, rhs: &f32) -> Self::Output {
        self.0.div(simd_4!(*rhs)).into()
    }
}
impl PartialEq for Vert4 {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        simd_almost_eq(&self.0, &other.0)
    }
}
pub trait Dot<Rhs: ?Sized = Self> {
    type Output;
    fn dot(self, rhs: Rhs) -> Self::Output;
}
impl Dot for Vert4 {
    type Output = f32;
    fn dot(self, rhs @ Vert4(rhs_simd): Self) -> Self::Output {
        debug_assert!(self.is_vector(), "Dot: Self is not a vector!");
        debug_assert!(rhs.is_vector(), "Dot: Rhs is not a vector!");
        Simd::mul(self.0, rhs_simd).reduce_sum()
    }
}
impl Dot<&Vert4> for Vert4 {
    type Output = f32;
    fn dot(self, rhs @ Vert4(rhs_simd): &Vert4) -> Self::Output {
        debug_assert!(self.is_vector(), "Dot: Self is not a vector!");
        debug_assert!(rhs.is_vector(), "Dot: Rhs is not a vector!");
        Simd::mul(self.0, rhs_simd).reduce_sum()
    }
}
impl Dot for &Vert4 {
    type Output = f32;
    fn dot(self, rhs @ Vert4(rhs_simd): Self) -> Self::Output {
        debug_assert!(self.is_vector(), "Dot: Self is not a vector!");
        debug_assert!(rhs.is_vector(), "Dot: Rhs is not a vector!");
        Simd::mul(self.0, rhs_simd).reduce_sum()
    }
}
impl Dot<Vert4> for &Vert4 {
    type Output = f32;
    fn dot(self, rhs @ Vert4(rhs_simd): Vert4) -> Self::Output {
        debug_assert!(self.is_vector(), "Dot: Self is not a vector!");
        debug_assert!(rhs.is_vector(), "Dot: Rhs is not a vector!");
        Simd::mul(self.0, rhs_simd).reduce_sum()
    }
}
pub trait Mag {
    type Output;
    fn mag(&self) -> Self::Output;
}
impl Mag for f32x4 {
    type Output = f32;
    #[inline]
    fn mag(&self) -> Self::Output {
        self.mul(self).reduce_sum().sqrt()
    }
}
impl Mag for Vert4 {
    type Output = f32;
    fn mag(&self) -> Self::Output {
        self.mul(self).reduce_sum().sqrt()
    }
}
pub trait Norm {
    type Output;
    fn norm(&self) -> Self::Output;
}
impl Norm for f32x4 {
    type Output = f32x4;
    #[inline]
    fn norm(&self) -> Self::Output {
        self / f32x4::from_array([self.mag(); 4])
    }
}
impl Norm for Vert4 {
    type Output = Vert4;
    #[inline]
    fn norm(&self) -> Self::Output {
        self / self.mag()
        // Vert4(self.0 / f32x4::from_array([self.0.mag(); 4]))
    }
}
pub trait NormAssign {
    fn norm_assign(&mut self);
}
impl NormAssign for f32x4 {
    #[inline]
    fn norm_assign(&mut self) {
        *self /= f32x4::from_array([self.mag(); 4])
    }
}
pub trait Cross<Rhs: ?Sized> {
    type Output;
    fn cross(self, rhs: Rhs) -> Self::Output;
}
pub(crate) const CROSS_SWIZZLE_0: [usize; 4] = [1, 2, 0, 3];
pub(crate) const CROSS_SWIZZLE_1: [usize; 4] = [2, 0, 1, 3];
impl Cross<Vert4> for Vert4 {
    type Output = Vert4;
    #[inline]
    fn cross(self, rhs: Vert4) -> Self::Output {
        let temp_0: f32x4 = simd_swizzle!(self.0, CROSS_SWIZZLE_0);
        let temp_1: f32x4 = simd_swizzle!(rhs.0, CROSS_SWIZZLE_1);
        let prod_0: f32x4 = temp_0 * temp_1;

        let temp_0: f32x4 = simd_swizzle!(self.0, CROSS_SWIZZLE_1);
        let temp_1: f32x4 = simd_swizzle!(rhs.0, CROSS_SWIZZLE_0);
        let prod_1: f32x4 = temp_0 * temp_1;
        Self(prod_0 - prod_1)
    }
}
impl Cross<Vert4> for &Vert4 {
    type Output = Vert4;
    #[inline]
    fn cross(self, rhs: Vert4) -> Self::Output {
        let temp_0: f32x4 = simd_swizzle!(self.0, CROSS_SWIZZLE_0);
        let temp_1: f32x4 = simd_swizzle!(rhs.0, CROSS_SWIZZLE_1);
        let prod_0: f32x4 = temp_0 * temp_1;

        let temp_0: f32x4 = simd_swizzle!(self.0, CROSS_SWIZZLE_1);
        let temp_1: f32x4 = simd_swizzle!(rhs.0, CROSS_SWIZZLE_0);
        let prod_1: f32x4 = temp_0 * temp_1;
        Vert4(prod_0 - prod_1)
    }
}
impl Cross<&Vert4> for Vert4 {
    type Output = Vert4;
    #[inline]
    fn cross(self, rhs: &Vert4) -> Self::Output {
        let temp_0: f32x4 = simd_swizzle!(self.0, CROSS_SWIZZLE_0);
        let temp_1: f32x4 = simd_swizzle!(rhs.0, CROSS_SWIZZLE_1);
        let prod_0: f32x4 = temp_0 * temp_1;

        let temp_0: f32x4 = simd_swizzle!(self.0, CROSS_SWIZZLE_1);
        let temp_1: f32x4 = simd_swizzle!(rhs.0, CROSS_SWIZZLE_0);
        let prod_1: f32x4 = temp_0 * temp_1;
        Self(prod_0 - prod_1)
    }
}
impl Cross<&Vert4> for &Vert4 {
    type Output = Vert4;
    #[inline]
    fn cross(self, rhs: &Vert4) -> Self::Output {
        let temp_0: f32x4 = simd_swizzle!(self.0, CROSS_SWIZZLE_0);
        let temp_1: f32x4 = simd_swizzle!(rhs.0, CROSS_SWIZZLE_1);
        let prod_0: f32x4 = temp_0 * temp_1;

        let temp_0: f32x4 = simd_swizzle!(self.0, CROSS_SWIZZLE_1);
        let temp_1: f32x4 = simd_swizzle!(rhs.0, CROSS_SWIZZLE_0);
        let prod_1: f32x4 = temp_0 * temp_1;
        Vert4(prod_0 - prod_1)
    }
}
