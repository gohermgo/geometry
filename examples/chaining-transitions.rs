use core::f32::consts::PI;
use geometry::{Matr4, Vert4};
#[inline(never)]
fn initial_point() -> Vert4 {
    Vert4::point(1., 0., 1.)
}
#[inline(never)]
fn a_matr() -> Matr4 {
    Matr4::rotation_x_rad(PI / 2.)
}
#[inline(never)]
const fn b_matr() -> Matr4 {
    Matr4::scaling(5., 5., 5.)
}
#[inline(never)]
const fn c_matr() -> Matr4 {
    Matr4::translation(10., 5., 7.)
}
#[inline(never)]
fn expensive_calculation() {
    for _ in 0..10000 {
        let p_a = a_matr() * initial_point();
        let p_b = b_matr() * p_a;
        let _p_c = c_matr() * p_b;
    }
}
fn main() {
    #[expect(clippy::unit_arg)]
    core::hint::black_box(expensive_calculation())
}
