mod vector;

#[test]
fn add_two_vertices() {
    let a1 = geometry::vertex!(3.0, -2.0, 5.0, 1.0);
    let a2 = geometry::vertex!(-2.0, 3.0, 1.0, 0.0);
    assert!((a1 + a2) == geometry::vertex!(1.0, 1.0, 6.0, 1.0));
}
#[test]
fn negating_vertices() {
    let a = geometry::vertex!(1.0, -2.0, 3.0, -4.0);
    assert!(-a == geometry::vertex!(-1.0, 2.0, -3.0, 4.0))
}
#[test]
fn multiplying_vertex_by_scalar() {
    let a = geometry::vertex!(1.0, -2.0, 3.0, -4.0);
    assert!((a * 3.5) == geometry::vertex!(3.5, -7.0, 10.5, -14.0))
}
#[test]
fn multiplying_vertex_by_fraction() {
    let a = geometry::vertex!(1.0, -2.0, 3.0, -4.0);
    assert!((a * 0.5) == geometry::vertex!(0.5, -1.0, 1.5, -2.0))
}
#[test]
fn dividing_vertex_by_scalar() {
    let a = geometry::vertex!(1.0, -2.0, 3.0, -4.0);
    assert!((a / 2.0) == geometry::vertex!(0.5, -1.0, 1.5, -2.0))
}

mod point {
    #[test]
    fn constructor_w_eq_one() {
        let p = geometry::point!(4.0, -4.0, 3.0);
        assert_eq!(p, geometry::vertex!(4.0, -4.0, 3.0, 1.0));
    }

    #[test]
    fn a_vertex_with_w_eq_one_is_a_point() {
        let a = geometry::point!(4.3, -4.2, 3.1);
        assert_eq!(a, geometry::vertex!(4.3, -4.2, 3.1, 1.0));
        assert!(a.is_point());
        assert!(!a.is_vector());
    }

    mod subtracting {
        #[test]
        fn two_points() {
            let p1 = geometry::point!(3.0, 2.0, 1.0);
            let p2 = geometry::point!(5.0, 6.0, 7.0);
            assert_eq!((p1 - p2), geometry::vector!(-2.0, -4.0, -6.0))
        }

        #[test]
        fn vector_from_point() {
            let p = geometry::point!(3.0, 2.0, 1.0);
            let v = geometry::vector!(5.0, 6.0, 7.0);
            assert_eq!((p - v), geometry::point!(-2.0, -4.0, -6.0))
        }
    }
}
