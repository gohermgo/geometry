use crate::{Tuple, Vector};
use std::ops::{Add, AddAssign, Sub, SubAssign};
pub struct Point(pub(crate) Tuple);
// impl Pointlike for Point {
//     #[inline]
//     fn new(x: f32, y: f32, z: f32) -> Self {
//         Self(Tuple::new(x, y, z, 1.0_f32))
//     }
//     #[inline]
//     fn is_vector(&self) -> bool {
//         false
//     }
// }
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
impl PartialEq<Tuple> for Point {
    #[inline]
    fn eq(&self, other: &Tuple) -> bool {
        self.0.eq(other)
    }
}
#[cfg(test)]
mod tests {
    use crate::{Point, Pointlike, Tuple, Vector};

    #[test]
    fn point_constructor_w_eq_one() {
        let p = Point::new(4.0, -4.0, 3.0);
        assert!(p == Tuple::new(4.0, -4.0, 3.0, 1.0));
    }

    #[test]
    fn a_tuple_with_w_eq_one_is_a_point() {
        let a = Point::new(4.3, -4.2, 3.1);
        assert!(a == Tuple::new(4.3, -4.2, 3.1, 1.0));
        assert!(a.is_point());
        assert!(!a.is_vector());
    }

    mod sub {
        use super::*;
        #[test]
        fn two_points() {
            let p1 = point!(3.0, 2.0, 1.0);
            let p2 = point!(5.0, 6.0, 7.0);
            assert!((p1 - p2) == vector!(-2.0, -4.0, -6.0))
        }

        #[test]
        fn vector_from_point() {
            let p = point!(3.0, 2.0, 1.0);
            let v = vector!(5.0, 6.0, 7.0);
            assert!((p - v) == point!(-2.0, -4.0, -6.0))
        }
    }
}
