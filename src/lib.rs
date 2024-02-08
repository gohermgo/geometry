#![feature(associated_type_bounds)]
#![feature(portable_simd)]
use std::simd::{f32x4, Simd};
pub mod cmp;
pub use cmp::SortaEq;
pub mod traits;
use ops::AbsDiff;
pub use traits::Pointlike;
pub mod ops;
const EPSILON: f32 = 0.00001_f32;
impl SortaEq for f32 {
    type Rhs = f32;
    #[inline]
    fn ehh_maybe(&self, rhs: &Self::Rhs) -> bool {
        #[cfg(any(test, debug))]
        println!("ehh_maybe called for f32");
        self.abs_diff(rhs).le(&EPSILON)
    }
}
const SIMD_EPSILON: Simd<f32, 4> = Simd::from_array([EPSILON, EPSILON, EPSILON, EPSILON]);
impl SortaEq for f32x4 {
    type Rhs = f32x4;
    #[inline]
    fn ehh_maybe(&self, rhs: &Self::Rhs) -> bool {
        #[cfg(any(test, debug))]
        println!("ehh_maybe called for f32x4");
        self.abs_diff(rhs).le(&SIMD_EPSILON)
    }
}
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
