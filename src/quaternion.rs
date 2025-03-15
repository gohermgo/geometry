use core::simd::{Simd, SimdElement};

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
