#![feature(portable_simd)]
pub mod matrix;
pub use matrix::{Cofactor, Determinant, Matr2, Matr3, Matr4, Matrix, Minor, Submatrix};
#[macro_use]
pub mod macros;
pub mod vertex;
pub use vertex::{Cross, Dot, Mag, Norm, Vert2, Vert3, Vert4};

#[macro_export]
macro_rules! vertex {
    ($x:literal, $y:literal, $z:literal, $w:literal) => {
        $crate::vertex::Vert4::new($x as f32, $y as f32, $z as f32, $w as f32)
    };
    ($x:expr, $y:expr, $z:expr, $w:expr) => {
        $crate::vertex::Vert4::new($x as f32, $y as f32, $z as f32, $w as f32)
    };
}
#[macro_export]
macro_rules! point {
    ($x:literal, $y:literal, $z:literal) => {{ $crate::vertex::Vert4::point($x as f32, $y as f32, $z as f32) }};
    ($x:expr, $y:expr, $z:expr) => {{ $crate::vertex::Vert4::point($x as f32, $y as f32, $z as f32) }};
}
#[macro_export]
macro_rules! vector {
    ($x:literal, $y:literal, $z:literal) => {{ $crate::vertex::Vert4::vector($x as f32, $y as f32, $z as f32) }};
    ($x:expr, $y:expr, $z:expr) => {{ $crate::vertex::Vert4::vector($x as f32, $y as f32, $z as f32) }};
}
/// Returns a unit-vector for X
#[macro_export]
macro_rules! vux {
    () => {
        $crate::vector!(1, 0, 0)
    };
}
/// Returns a unit-vector for Y
#[macro_export]
macro_rules! vuy {
    () => {
        $crate::vector!(0, 1, 0)
    };
}
/// Returns a unit-vector for Z
#[macro_export]
macro_rules! vuz {
    () => {
        $crate::vector!(0, 0, 1)
    };
}
#[macro_export]
macro_rules! mat2 {
    ($arr:literal) => {{
        <$crate::matrix::Matr2 as $crate::matrix::FromArray<f32, 4>>::from_array($arr)
    }};
    ($($arr:literal),*$(,)?) => {{
        <$crate::matrix::Matr2 as $crate::matrix::FromArray<f32, 4>>::from_array([$($arr),*])
    }};
    ($arr:expr) => {{
        <$crate::matrix::Matr2 as $crate::matrix::FromArray<f32, 4>>::from_array($arr)
    }};
}
#[macro_export]
macro_rules! mat3 {

    ($arr:literal) => {{
        <$crate::matrix::Matr3 as $crate::matrix::FromArray<f32, 9>>::from_array($arr)
    }};
    ($($arr:literal),*$(,)?) => {{
        <$crate::matrix::Matr3 as $crate::matrix::FromArray<f32, 9>>::from_array([$($arr),*])
    }};
    ($arr:expr) => {{
        <$crate::matrix::Matr3 as $crate::matrix::FromArray<f32, 9>>::from_array($arr)
    }};
}
#[macro_export]
macro_rules! mat4 {
    ($arr:literal) => {{
        <$crate::matrix::Matr4 as $crate::matrix::FromArray<f32, 16>>::from_array($arr)
    }};
    ($($arr:literal),*$(,)?) => {{
        <$crate::matrix::Matr4 as $crate::matrix::FromArray<f32, 16>>::from_array([$($arr),*])
    }};
    ($arr:expr) => {{
        <$crate::matrix::Matr4 as $crate::matrix::FromArray<f32, 16>>::from_array($arr)
    }};
}

pub enum TupleType {
    Point,
    Vector,
}

pub trait PointType {
    fn new(x: f32, y: f32, z: f32) -> Self;
    fn is_vector(&self) -> bool;
    #[inline]
    fn is_point(&self) -> bool {
        !self.is_vector()
    }
}
/// # Pure
pub fn clock(twelve: Vert4) -> impl Iterator<Item = Vert4> {
    use core::f32::consts::PI;
    (0..12).map(move |idx| Matr4::rotation_y_rad((idx as f32 * (2. * PI)) / 12.0) * &twelve)
}
