use crate::{Vector, Vert4};
use std::ops::{Add, AddAssign, Sub, SubAssign};
pub struct Point(pub(crate) Vert4);
impl Point {
    #[inline]
    pub fn x(&self) -> f32 {
        self.0.x()
    }
    #[inline]
    pub fn y(&self) -> f32 {
        self.0.y()
    }
}
impl Add<Vector> for Point {
    type Output = Point;
    #[inline]
    fn add(self, rhs: Vector) -> Self::Output {
        Self(self.0.add(rhs.0))
    }
}
impl AddAssign<Vector> for Point {
    #[inline]
    fn add_assign(&mut self, rhs: Vector) {
        self.0 .0.add_assign(rhs.0 .0)
    }
}
impl Add<&Vector> for Point {
    type Output = Point;
    #[inline]
    fn add(self, rhs: &Vector) -> Self::Output {
        Self(self.0 .0.add(rhs.0 .0).into())
    }
}
impl AddAssign<&Vector> for Point {
    #[inline]
    fn add_assign(&mut self, rhs: &Vector) {
        self.0 .0.add_assign(rhs.0 .0)
    }
}
impl AddAssign<&mut Vector> for Point {
    #[inline]
    fn add_assign(&mut self, rhs: &mut Vector) {
        self.0 .0.add_assign(rhs.0 .0)
    }
}
impl AddAssign<&&mut Vector> for Point {
    #[inline]
    fn add_assign(&mut self, rhs: &&mut Vector) {
        self.0 .0.add_assign(rhs.0 .0)
    }
}
impl Add<Vector> for &Point {
    type Output = Point;
    #[inline]
    fn add(self, rhs: Vector) -> Self::Output {
        Point(self.0 .0.add(rhs.0 .0).into())
    }
}
impl Add<&Vector> for &Point {
    type Output = Point;
    #[inline]
    fn add(self, rhs: &Vector) -> Self::Output {
        Point(self.0 .0.add(rhs.0 .0).into())
    }
}
// Point - Point = Vector
impl Sub for Point {
    type Output = Vector;
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Vector(self.0.sub(rhs.0))
    }
}
impl Sub<&Self> for Point {
    type Output = Vector;
    #[inline]
    fn sub(self, rhs: &Self) -> Self::Output {
        Vector(self.0 .0.sub(rhs.0 .0).into())
    }
}
impl Sub<Point> for &Point {
    type Output = Vector;
    #[inline]
    fn sub(self, rhs: Point) -> Self::Output {
        Vector(self.0 .0.sub(rhs.0 .0).into())
    }
}
impl Sub for &Point {
    type Output = Vector;
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Vector(self.0 .0.sub(rhs.0 .0).into())
    }
}
impl SubAssign for Point {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.0.sub_assign(rhs.0)
    }
}
impl SubAssign<&Self> for Point {
    #[inline]
    fn sub_assign(&mut self, rhs: &Self) {
        self.0 .0.sub_assign(rhs.0 .0)
    }
}
// Point - Vector = Point
impl Sub<Vector> for Point {
    type Output = Self;
    #[inline]
    /// `Point` - `Vector` = `Point`
    fn sub(self, rhs: Vector) -> Self::Output {
        Self(self.0.sub(rhs.0))
    }
}
impl Sub<&Vector> for Point {
    type Output = Self;
    #[inline]
    /// `Point` - `&Vector` = `Point`
    fn sub(self, rhs: &Vector) -> Self::Output {
        Self(self.0 .0.sub(rhs.0 .0).into())
    }
}
impl Sub<Vector> for &Point {
    type Output = Point;
    #[inline]
    /// `&Point` - `Vector` = `Point`
    fn sub(self, rhs: Vector) -> Self::Output {
        Point(self.0 .0.sub(rhs.0 .0).into())
    }
}
impl Sub<&Vector> for &Point {
    type Output = Point;
    /// `&Point` - `&Vector` = `Point`
    #[inline]
    fn sub(self, rhs: &Vector) -> Self::Output {
        Point(self.0 .0.sub(rhs.0 .0).into())
    }
}
impl SubAssign<Vector> for Point {
    #[inline]
    fn sub_assign(&mut self, rhs: Vector) {
        self.0.sub_assign(rhs.0)
    }
}
impl SubAssign<&Vector> for Point {
    #[inline]
    fn sub_assign(&mut self, rhs: &Vector) {
        self.0 .0.sub_assign(rhs.0 .0)
    }
}
impl PartialEq for Point {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(other)
    }
}
impl PartialEq<Vert4> for Point {
    #[inline]
    fn eq(&self, other: &Vert4) -> bool {
        self.0.eq(other)
    }
}
