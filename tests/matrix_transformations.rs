use geometry::matrix::Inverse;
use geometry::matrix::ShearingProportion;
use geometry::{Matr4, Vert4};
//
// Translation tests
//
#[test]
fn multiplying_by_a_translation_matrix() {
    let transform = Matr4::translation(5., -3., 2.);
    let p = Vert4::new(-3., 4., 5., 1.);

    let res = transform * p;
    assert_eq!(res, Vert4::new(2., 1., 7., 1.))
}
#[test]
fn multiplying_by_the_inverse_of_a_translation_matrix() {
    let transform = Matr4::translation(5., -3., 2.);
    let p = Vert4::point(-3., 4., 5.);

    let inv = transform.inverse().unwrap();

    let res = inv * p;
    assert_eq!(res, Vert4::point(-8., 7., 3.))
}
#[test]
fn translation_does_not_affect_vectors() {
    let transform = Matr4::translation(5., -3., 2.);
    let v = Vert4::vector(-3., 4., 5.);
    let res = transform * &v;
    assert_eq!(res, v)
}
//
// Scaling tests
//
#[test]
fn a_scaling_matrix_applied_to_a_point() {
    let transform = Matr4::scaling(2., 3., 4.);
    let p = Vert4::point(-4., 6., 8.);
    let res = transform * p;
    assert_eq!(res, Vert4::point(-8., 18., 32.))
}
#[test]
fn a_scaling_matrix_applied_to_a_vector() {
    let transform = Matr4::scaling(2., 3., 4.);
    let v = Vert4::vector(-4., 6., 8.);
    let res = transform * v;
    assert_eq!(res, Vert4::vector(-8., 18., 32.))
}
#[test]
fn multiplying_by_the_inverse_of_a_scaling_matrix() {
    let transform = Matr4::scaling(2., 3., 4.);
    let inv = transform.inverse().unwrap();
    let v = Vert4::vector(-4., 6., 8.);
    let res = inv * v;
    assert_eq!(res, Vert4::vector(-2., 2., 2.))
}
#[test]
fn reflection_is_scaling_by_a_negative_value() {
    let transform = Matr4::scaling(-1., 1., 1.);
    let p = Vert4::point(2., 3., 4.);
    let res = transform * p;
    assert_eq!(res, Vert4::point(-2., 3., 4.))
}
//
// Rotation tests
//
#[test]
fn rotating_a_point_around_the_x_axis() {
    use core::f32::consts::PI;

    let p = Vert4::point(0., 1., 0.);

    let half_quarter = Matr4::rotation_x_rad(PI / 4.);
    let half_rotp = half_quarter * &p;
    assert_eq!(
        half_rotp,
        Vert4::point(0., 2_f32.sqrt() / 2., 2_f32.sqrt() / 2.)
    );

    let full_quarter = Matr4::rotation_x_rad(PI / 2.);
    let full_rotp = full_quarter * &p;
    assert_eq!(full_rotp, Vert4::point(0., 0., 1.))
}
#[test]
fn the_inverse_of_an_x_rotation_rotates_in_the_oppsite_direction() {
    use core::f32::consts::PI;

    let p = Vert4::point(0., 1., 0.);

    let half_quarter = Matr4::rotation_x_rad(PI / 4.);
    let inv = half_quarter.inverse().unwrap();
    let half_rotp = inv * &p;
    assert_eq!(
        half_rotp,
        Vert4::point(0., 2_f32.sqrt() / 2., -(2_f32.sqrt() / 2.))
    );
}
#[test]
fn rotating_a_point_around_the_y_axis() {
    use core::f32::consts::PI;

    let p = Vert4::point(0., 0., 1.);

    let half_quarter = Matr4::rotation_y_rad(PI / 4.);
    let half_rotp = half_quarter * &p;
    assert_eq!(
        half_rotp,
        Vert4::point(2_f32.sqrt() / 2., 0., 2_f32.sqrt() / 2.)
    );

    let full_quarter = Matr4::rotation_y_rad(PI / 2.);
    let full_rotp = full_quarter * &p;
    assert_eq!(full_rotp, Vert4::point(1., 0., 0.))
}
#[test]
fn rotating_a_point_around_the_z_axis() {
    use core::f32::consts::PI;

    let p = Vert4::point(0., 1., 0.);

    let half_quarter = Matr4::rotation_z_rad(PI / 4.);
    let half_rotp = half_quarter * &p;
    assert_eq!(
        half_rotp,
        Vert4::point(-2_f32.sqrt() / 2., 2_f32.sqrt() / 2., 0.)
    );

    let full_quarter = Matr4::rotation_z_rad(PI / 2.);
    let full_rotp = full_quarter * &p;
    assert_eq!(full_rotp, Vert4::point(-1., 0., 0.))
}
//
// Shearing tests
//
#[test]
fn a_shearing_transformation_moves_x_in_proportion_to_y() {
    let transform = Matr4::shearing(
        ShearingProportion::new(1., 0.),
        ShearingProportion::default(),
        ShearingProportion::default(),
    );
    let p = Vert4::point(2., 3., 4.);
    assert_eq!(transform * p, Vert4::point(5., 3., 4.))
}
#[test]
fn a_shearing_transformation_moves_x_in_proportion_to_z() {
    let transform = Matr4::shearing(
        ShearingProportion::new(0., 1.),
        ShearingProportion::default(),
        ShearingProportion::default(),
    );
    let p = Vert4::point(2., 3., 4.);
    assert_eq!(transform * p, Vert4::point(6., 3., 4.))
}
#[test]
fn a_shearing_transformation_moves_y_in_proportion_to_x() {
    let transform = Matr4::shearing(
        ShearingProportion::default(),
        ShearingProportion::new(1., 0.),
        ShearingProportion::default(),
    );
    let p = Vert4::point(2., 3., 4.);
    assert_eq!(transform * p, Vert4::point(2., 5., 4.))
}
#[test]
fn a_shearing_transformation_moves_y_in_proportion_to_z() {
    let transform = Matr4::shearing(
        ShearingProportion::default(),
        ShearingProportion::new(0., 1.),
        ShearingProportion::default(),
    );
    let p = Vert4::point(2., 3., 4.);
    assert_eq!(transform * p, Vert4::point(2., 7., 4.))
}
#[test]
fn a_shearing_transformation_moves_z_in_proportion_to_x() {
    let transform = Matr4::shearing(
        ShearingProportion::default(),
        ShearingProportion::default(),
        ShearingProportion::new(1., 0.),
    );
    let p = Vert4::point(2., 3., 4.);
    assert_eq!(transform * p, Vert4::point(2., 3., 6.))
}
#[test]
fn a_shearing_transformation_moves_z_in_proportion_to_y() {
    let transform = Matr4::shearing(
        ShearingProportion::default(),
        ShearingProportion::default(),
        ShearingProportion::new(0., 1.),
    );
    let p = Vert4::point(2., 3., 4.);
    assert_eq!(transform * p, Vert4::point(2., 3., 7.))
}
