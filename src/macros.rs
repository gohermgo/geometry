#![allow(unused_macros)]
macro_rules! simd_n {
    ($($x:expr),+) => {
        Simd::from_array([$($x),+])
    };
}
macro_rules! simd_4 {
    ($x:expr) => {
        simd_n!($x, $x, $x, $x)
    };
}
macro_rules! simd {
    ($x:expr, $y:expr, $z:expr, $w:expr) => {
        simd_n!($x, $y, $z, $w)
    };
}
macro_rules! tuple {
    ($x:expr, $y:expr, $z:expr, $w:expr) => {
        Tuple::new($x, $y, $z, $w)
    };
}
macro_rules! tuple_4 {
    ($x:expr) => {
        tuple!($x, $x, $x, $x)
    };
}
macro_rules! point {
    ($x:expr, $y:expr, $z:expr) => {
        Point::new($x, $y, $z)
    };
}
macro_rules! point_3 {
    ($x:expr) => {
        point!($x, $x, $x)
    };
}
macro_rules! vector {
    ($x:expr, $y:expr, $z:expr) => {
        Vector::new($x, $y, $z)
    };
}
macro_rules! vector_3 {
    ($x:expr) => {
        vector!($x, $x, $x)
    };
}
