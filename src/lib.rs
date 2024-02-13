#![feature(associated_type_bounds)]
#![feature(const_trait_impl)]
#![feature(effects)]
#![feature(generic_const_exprs)]
#![feature(portable_simd)]
pub mod matrix;
pub use matrix::{Det, Mat2, Mat3, Mat4, Matrix, Submat};
pub mod traits;
pub use traits::Pointlike;
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
