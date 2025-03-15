use core::f32::consts::PI;
use geometry::{Matr4, Vert4};
#[inline]
fn initial_point() -> Vert4 {
    Vert4::point(1., 0., 1.)
}
#[inline]
fn a_matr() -> Matr4 {
    Matr4::rotation_x_rad(PI / 2.)
}
#[inline]
const fn b_matr() -> Matr4 {
    Matr4::scaling(5., 5., 5.)
}
#[inline]
const fn c_matr() -> Matr4 {
    Matr4::translation(10., 5., 7.)
}
#[test]
fn individual_transformations_are_applied_in_sequence() {
    let p_a = a_matr() * initial_point();
    assert_eq!(p_a, Vert4::point(1., -1., 0.));

    let p_b = b_matr() * p_a;
    assert_eq!(p_b, Vert4::point(5., -5., 0.));

    let p_c = c_matr() * p_b;
    assert_eq!(p_c, Vert4::point(15., 0., 7.));
}
#[test]
fn chained_transformations_must_be_applied_in_reverse_order() {
    let t = c_matr() * b_matr() * a_matr() * initial_point();
    assert_eq!(t, Vert4::point(15., 0., 7.));
}
