#![feature(associated_type_bounds)]
#![feature(portable_simd)]
use std::simd::{f32x4, Simd};
pub mod traits;
pub use traits::Pointlike;
pub mod ops;
#[macro_use]
pub mod macros;
pub mod tuple;
pub use tuple::Tuple;

pub enum TupleType {
    Point,
    Vector,
}
pub mod vector;
pub use vector::Vector;
pub mod point;
pub use point::Point;
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    //! TODO TEsts kek
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
