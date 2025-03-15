#![allow(incomplete_features)]
#![feature(const_trait_impl)]
#![feature(generic_const_exprs)]
#![feature(portable_simd)]
pub mod matrix;
pub use matrix::{Cofactor, Determinant, Mat2, Mat3, Matr4, Matrix, Minor, Submatrix};
pub mod cmp;
pub mod ops;
#[macro_use]
pub mod macros;
pub mod vertex;
pub use vertex::{Vert2, Vert3, Vert4, Vertex};

pub enum TupleType {
    Point,
    Vector,
}
mod vector;
pub use vector::Vector;
pub mod point;
pub use point::Point;

pub trait PointType {
    fn new(x: f32, y: f32, z: f32) -> Self;
    fn is_vector(&self) -> bool;
    #[inline]
    fn is_point(&self) -> bool {
        !self.is_vector()
    }
}
impl PointType for Vector {
    #[inline]
    fn new(x: f32, y: f32, z: f32) -> Self {
        Self(Vert4::new(x, y, z, 0.0_f32))
    }
    #[inline]
    fn is_vector(&self) -> bool {
        true
    }
}
impl PointType for Point {
    #[inline]
    fn new(x: f32, y: f32, z: f32) -> Self {
        Self(Vert4::new(x, y, z, 1.0_f32))
    }
    #[inline]
    fn is_vector(&self) -> bool {
        false
    }
}
