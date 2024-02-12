use crate::{Point, Vector, Vert4};
pub trait Pointlike {
    fn new(x: f32, y: f32, z: f32) -> Self;
    fn is_vector(&self) -> bool;
    #[inline]
    fn is_point(&self) -> bool {
        !self.is_vector()
    }
}
impl Pointlike for Vector {
    #[inline]
    fn new(x: f32, y: f32, z: f32) -> Self {
        Self(Vert4::new(x, y, z, 0.0_f32))
    }
    #[inline]
    fn is_vector(&self) -> bool {
        true
    }
}
impl Pointlike for Point {
    #[inline]
    fn new(x: f32, y: f32, z: f32) -> Self {
        Self(Vert4::new(x, y, z, 1.0_f32))
    }
    #[inline]
    fn is_vector(&self) -> bool {
        false
    }
}
