#[test]
fn constructor_w_eq_zero() {
    let v = geometry::vector!(4, -4, 3);
    assert_eq!(v, geometry::vector!(4, -4, 3));
}
#[test]
fn a_vertex_with_w_eq_zero_is_a_vector() {
    let a = geometry::vector!(4.3, -4.2, 3.1);
    assert_eq!(a, geometry::vector!(4.3, -4.2, 3.1));
    assert!(!a.is_point() && a.is_vector());
}

mod subtracting {
    #[test]
    fn two_vectors() {
        let v1 = geometry::vector!(3.0, 2.0, 1.0);
        let v2 = geometry::vector!(5.0, 6.0, 7.0);
        assert_eq!((v1 - v2), geometry::vector!(-2.0, -4.0, -6.0))
    }
    #[test]
    fn vector_from_zero_vector() {
        let v = geometry::vector!(1.0, -2.0, 3.0);
        assert_eq!((geometry::Vert4::ZERO - v), geometry::vector!(-1., 2., -3.))
    }
}
mod dot_product {
    #[test]
    fn two_vectors() {
        use geometry::vertex::Dot;
        let a = geometry::vector!(1.0, 2.0, 3.0);
        let b = geometry::vector!(2.0, 3.0, 4.0);
        assert_eq!(a.dot(b), 20.0);
    }
}
mod magnitude {
    use geometry::vertex::Mag;
    #[test]
    fn x_unit_vector() {
        let v = geometry::vux!();
        assert_eq!(v.mag(), 1.0);
    }
    #[test]
    fn y_unit_vector() {
        let v = geometry::vuy!();
        assert_eq!(v.mag(), 1.0);
    }
    #[test]
    fn unit_3() {
        let v = geometry::vuz!();
        assert_eq!(v.mag(), 1.0);
    }
    #[test]
    fn unit_4() {
        let v = geometry::vector!(1.0, 2.0, 3.0);
        assert_eq!(v.mag(), 14.0_f32.sqrt());
    }
    #[test]
    fn unit_5() {
        let v = geometry::vector!(-1.0, -2.0, -3.0);
        assert_eq!(v.mag(), 14.0_f32.sqrt());
    }
}
mod normalizing {
    use geometry::vertex::{Mag, Norm};
    #[test]
    fn unit_1() {
        let v = geometry::vector!(4.0, 0.0, 0.0);
        assert_eq!(v.norm(), geometry::vector!(1.0, 0.0, 0.0));
    }
    #[test]
    fn unit_2() {
        let v = geometry::vector!(1.0, 2.0, 3.0);
        assert_eq!(v.norm(), geometry::vector!(0.26726, 0.53452, 0.80178))
    }
    #[test]
    fn unit_3() {
        let v = geometry::vector!(1.0, 2.0, 3.0);
        let norm = v.norm();
        assert!(geometry::vertex::float_almost_eq(&norm.mag(), &1.0));
    }
}
#[test]
fn the_cross_product_of_two_vectors() {
    use geometry::vertex::Cross;
    let a = geometry::vector!(1., 2., 3.);
    let b = geometry::vector!(2., 3., 4.);

    let v1 = Cross::cross(&a, &b);
    assert_eq!(v1, geometry::vector!(-1., 2., -1.));

    let v2 = Cross::cross(&b, &a);
    assert_eq!(v2, geometry::vector!(1., -2., 1.));
}
