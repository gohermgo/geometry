pub trait Pointlike {
    fn new(x: f32, y: f32, z: f32) -> Self;
    fn is_vector(&self) -> bool;
    #[inline]
    fn is_point(&self) -> bool {
        !self.is_vector()
    }
}
