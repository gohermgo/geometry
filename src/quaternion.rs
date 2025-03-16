use core::simd::{Simd, SimdElement};

use core::ops::{Add, AddAssign};
use core::ops::{Mul, MulAssign};
use core::ops::{Sub, SubAssign};

use crate::Vert3;

#[derive(Debug)]
pub struct Quaternion<T: SimdElement>(Simd<T, 4>);
impl<T: SimdElement> From<Simd<T, 4>> for Quaternion<T> {
    fn from(value: Simd<T, 4>) -> Self {
        Quaternion(value)
    }
}
impl<T: SimdElement> From<[T; 4]> for Quaternion<T> {
    fn from(value: [T; 4]) -> Self {
        Simd::from_array(value).into()
    }
}
impl<T: SimdElement> Add<T> for Quaternion<T>
where
    Simd<T, 4>: Add,
    Quaternion<T>: From<<Simd<T, 4> as Add>::Output>,
{
    type Output = Quaternion<T>;
    fn add(self, rhs: T) -> Self::Output {
        let rhs = Simd::splat(rhs);
        let Quaternion(lhs) = self;
        Simd::add(lhs, rhs).into()
    }
}
impl<T: SimdElement> Add for Quaternion<T>
where
    Simd<T, 4>: Add,
    Quaternion<T>: From<<Simd<T, 4> as Add>::Output>,
{
    type Output = Quaternion<T>;
    fn add(self, Quaternion(rhs): Quaternion<T>) -> Self::Output {
        let Quaternion(lhs) = self;
        Simd::add(lhs, rhs).into()
    }
}
impl<T: SimdElement> Sub<T> for Quaternion<T>
where
    Simd<T, 4>: Sub,
    Quaternion<T>: From<<Simd<T, 4> as Sub>::Output>,
{
    type Output = Quaternion<T>;
    fn sub(self, rhs: T) -> Self::Output {
        let rhs = Simd::splat(rhs);
        let Quaternion(lhs) = self;
        Simd::sub(lhs, rhs).into()
    }
}
impl<T: SimdElement> Sub for Quaternion<T>
where
    Simd<T, 4>: Sub,
    Quaternion<T>: From<<Simd<T, 4> as Sub>::Output>,
{
    type Output = Quaternion<T>;
    fn sub(self, Quaternion(rhs): Quaternion<T>) -> Self::Output {
        let Quaternion(lhs) = self;
        Simd::sub(lhs, rhs).into()
    }
}
fn quaternion_vector_component<T: SimdElement>(Quaternion(s): &Quaternion<T>) -> Vert3<T> {
    Vert3::from(core::array::from_fn(|idx| s[idx + 1].into()))
}
// q1q2 = (a1,v1)(a2,v2)
//      = (a1a2 - v1v2, a1v2 + a2v1 + v1 `cross` v2)
// Thus, scalar is (a1a2 - v1v2)
//       vector is (a1v2 + a2v1 + v1 `cross` v2)

// S <- Sa * Sb - Xa * Xb - Ya * Yb - Za * Zb
// X <- Sa * Xb + Sb * Ya
impl<T: SimdElement> Mul<T> for Quaternion<T>
where
    Simd<T, 4>: Mul,
    Quaternion<T>: From<<Simd<T, 4> as Mul>::Output>,
{
    type Output = Quaternion<T>;
    fn mul(self, rhs: T) -> Self::Output {
        let rhs = Simd::splat(rhs);
        let Quaternion(lhs) = self;
        Simd::mul(lhs, rhs).into()
    }
}
impl<T: SimdElement + Mul<Output = T> + Add<Output = T> + Sub<Output = T>> Mul for Quaternion<T>
where
    Simd<T, 4>: Mul,
    Quaternion<T>: From<<Simd<T, 4> as Mul>::Output>,
{
    type Output = Quaternion<T>;
    fn mul(self, q2 @ Quaternion(rhs): Quaternion<T>) -> Self::Output {
        let v1 = quaternion_vector_component(&self);
        let v2 = quaternion_vector_component(&q2);

        let self_scalar = self.0[0];
        let rhs_scalar = rhs[0];

        // Scalar = a1a2 - v1v2
        let scalar = (self_scalar * rhs_scalar) - crate::Dot::dot(v1, v2);

        // Vector = a1v2 + a2v1 + v1 `cross` v2
        let vert = (v2 * self_scalar) + (v1 * rhs_scalar) + crate::Cross::cross(v1, v2);

        Simd::from_array([scalar, vert[0], vert[1], vert[2]]).into()
    }
}
// TODO:
// RTQuaternion& operator +=(const RTQuaternion& quat);
// RTQuaternion& operator -=(const RTQuaternion& quat);
// RTQuaternion& operator *=(const RTQuaternion& qb);
// RTQuaternion& operator *=(const RTFLOAT val);
// RTQuaternion& operator -=(const RTFLOAT val);

// TODO:
// RTQuaternion& operator =(const RTQuaternion& quat);
// const RTQuaternion operator *(const RTQuaternion& qb) const;
// const RTQuaternion operator *(const RTFLOAT val) const;
// const RTQuaternion operator -(const RTQuaternion& qb) const;
// const RTQuaternion operator -(const RTFLOAT val) const;

#[rustfmt::skip]
pub fn scalar_of<T: SimdElement>(Quaternion(s): &Quaternion<T>) -> T { s[0] }
#[rustfmt::skip]
pub fn x_of<T: SimdElement>(Quaternion(s): &Quaternion<T>) -> T { s[1] }
#[rustfmt::skip]
pub fn y_of<T: SimdElement>(Quaternion(s): &Quaternion<T>) -> T { s[2] }
#[rustfmt::skip]
pub fn z_of<T: SimdElement>(Quaternion(s): &Quaternion<T>) -> T { s[3] }
#[rustfmt::skip]
pub fn with_scalar_of<T: SimdElement>(Quaternion(s): Quaternion<T>, scalar: T) -> Quaternion<T> { [scalar, s[1], s[2], s[3]].into() }
#[rustfmt::skip]
pub fn with_x_of<T: SimdElement>(Quaternion(s): Quaternion<T>, x: T) -> Quaternion<T> { [s[0], x, s[2], s[3]].into() }
#[rustfmt::skip]
pub fn with_y_of<T: SimdElement>(Quaternion(s): Quaternion<T>, y: T) -> Quaternion<T> { [s[0], s[1], y, s[3]].into() }
#[rustfmt::skip]
pub fn with_z_of<T: SimdElement>(Quaternion(s): Quaternion<T>, z: T) -> Quaternion<T> { [s[0], s[1], s[2], z].into() }
#[rustfmt::skip]
pub fn from_array<T: SimdElement>(array: [T; 4]) -> Quaternion<T> { array.into() }
#[rustfmt::skip]
pub fn to_array<T: SimdElement>(Quaternion(s): Quaternion<T>) -> [T; 4] { s.to_array() }
#[rustfmt::skip]
pub fn default<T: SimdElement + Default>() -> Quaternion<T> { [T::default(); 4].into() }

// TODO:
// void normalize();
// void toEuler(RTVector3& vec);
// void fromEuler(RTVector3& vec);
// RTQuaternion conjugate() const;
// void toAngleVector(RTFLOAT& angle, RTVector3& vec);
// void fromAngleVector(const RTFLOAT& angle, const RTVector3& vec);
