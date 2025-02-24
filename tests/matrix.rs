use geometry::matrix::FromArray;
use geometry::{Matr2, Matr3, Matr4, Matrix};
#[test]
fn constructing_mat4() {
    let m = Matr4::from_array([
        1.0, 2.0, 3.0, 4.0, 5.5, 6.5, 7.5, 8.5, 9.0, 10.0, 11.0, 12.0, 13.5, 14.5, 15.5, 16.5,
    ]);
    assert_eq!(m[(0, 0)], 1.0);
    assert_eq!(m[(0, 3)], 4.0);
    assert_eq!(m[(1, 0)], 5.5);
    assert_eq!(m[(1, 2)], 7.5);
    assert_eq!(m[(2, 2)], 11.0);
    assert_eq!(m[(3, 0)], 13.5);
    assert_eq!(m[(3, 2)], 15.5);
}
#[test]
fn constructing_mat3() {
    let m = Matr3::from_array([-3.0, 5.0, 0.0, 1.0, -2.0, -7.0, 0.0, 1.0, 1.0]);
    assert_eq!(m[(0, 0)], -3.0);
    assert_eq!(m[(1, 1)], -2.0);
    assert_eq!(m[(2, 2)], 1.0);
}
#[test]
fn constructing_mat2() {
    let m = Matr2::from_array([-3.0, 5.0, 1.0, -2.0]);
    assert_eq!(m[(0, 0)], -3.0);
    assert_eq!(m[(0, 1)], 5.0);
    assert_eq!(m[(1, 0)], 1.0);
    assert_eq!(m[(1, 1)], -2.0);
}
mod mat2 {
    mod determinant {
        use geometry::matrix::Determinant;
        #[test]
        fn calculating_determinant_1() {
            let a = geometry::mat2! {1.0, 5.0, -3.0, 2.0};
            assert_eq!(a.determinant(), 17.0)
        }
    }
}
mod mat3 {
    mod cofactor {
        use geometry::matrix::{Cofactor, Minor};
        #[test]
        fn calculating_cofactor_1() {
            let a = geometry::mat3! {3.0, 5.0, 0.0, 2.0, -1.0, -7.0, 6.0, -1.0, 5.0};

            assert_eq!(a.minor(0, 0), -12.0);
            assert_eq!(a.cofactor(0, 0), -12.0);

            assert_eq!(a.minor(1, 0), 25.0);
            assert_eq!(a.cofactor(1, 0), -25.0);
        }
    }
}
mod mat4 {
    use super::*;
    #[test]
    fn equality() {
        let a = Matr4::from_array([
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
        ]);
        let b = Matr4::from_array([
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
        ]);
        assert_eq!(a, b)
    }
    #[test]
    fn inequality() {
        let a = Matr4::from_array([
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
        ]);
        let b = Matr4::from_array([
            2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0,
        ]);
        assert_ne!(a, b)
    }
    mod mul {
        use super::*;
        #[test]
        fn multiplication_by_matrix() {
            let a = Matr4::from_array([
                1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
            ]);
            let b = Matr4::from_array([
                -2.0, 1.0, 2.0, 3.0, 3.0, 2.0, 1.0, -1.0, 4.0, 3.0, 6.0, 5.0, 1.0, 2.0, 7.0, 8.0,
            ]);
            assert_eq!(
                a * b,
                Matr4::from_array([
                    20.0, 22.0, 50.0, 48.0, 44.0, 54.0, 114.0, 108.0, 40.0, 58.0, 110.0, 102.0,
                    16.0, 26.0, 46.0, 42.0
                ])
            )
        }
        #[test]
        fn multiplication_by_vertex() {
            let a = Matr4::from_array([
                1.0, 2.0, 3.0, 4.0, 2.0, 4.0, 4.0, 2.0, 8.0, 6.0, 4.0, 1.0, 0.0, 0.0, 0.0, 1.0,
            ]);
            let b = geometry::vertex!(1.0, 2.0, 3.0, 1.0);
            assert_eq!(a * b, geometry::vertex!(18.0, 24.0, 33.0, 1.0))
        }
        #[test]
        fn multiplication_by_ident() {
            let a = geometry::mat4! {
                0.0, 1.0, 2.0, 4.0, 1.0, 2.0, 4.0, 8.0, 2.0, 4.0, 8.0, 16.0, 4.0, 8.0, 16.0, 32.0,
            };
            assert_eq!(&a * Matr4::identity(), a)
        }
        #[test]
        fn multiplication_of_vertex_by_ident() {
            let a = geometry::vertex!(1.0, 2.0, 3.0, 4.0);
            assert_eq!(Matr4::identity() * &a, a)
        }
    }
    #[test]
    fn transposition() {
        let a = geometry::mat4! {
            0.0, 9.0, 3.0, 0.0, 9.0, 8.0, 0.0, 8.0, 1.0, 8.0, 5.0, 3.0, 0.0, 0.0, 5.0, 8.0,
        };
        assert_eq!(
            a.transpose(),
            geometry::mat4! {
                0.0, 9.0, 1.0, 0.0, 9.0, 8.0, 8.0, 0.0, 3.0, 0.0, 5.0, 5.0, 0.0, 8.0, 3.0, 8.0
            }
        )
    }
    #[test]
    fn transposition_of_ident() {
        let a = Matr4::identity();
        assert_eq!(a.transpose(), Matr4::identity())
    }
    mod determinant {
        use geometry::matrix::{Cofactor, Determinant};
        #[test]
        fn calculate_determinant_1() {
            let a = geometry::mat4! {
                -2.0, -8.0, 3.0, 5.0, -3.0, 1.0, 7.0, 3.0, 1.0, 2.0, -9.0, 6.0, -6.0, 7.0, 7.0,
                -9.0,
            };
            assert_eq!(a.cofactor(0, 0), 690.0);
            assert_eq!(a.cofactor(0, 1), 447.0);
            assert_eq!(a.cofactor(0, 2), 210.0);
            assert_eq!(a.cofactor(0, 3), 51.0);
            assert_eq!(a.determinant(), -4071.0);
        }
    }
    mod inverse {
        use geometry::matrix::{AsColumns, Cofactor, Determinant, Inverse, Matrix};
        use geometry::Vert4;

        use super::*;
        #[test]
        fn invertible() {
            let a = geometry::mat4! {
                6.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 6.0, 4.0, -9.0, 3.0, -7.0, 9.0, 1.0, 7.0, -6.0,
            };
            assert_eq!(a.determinant(), -2120.0);
            assert!(a.is_invertible())
        }
        #[test]
        fn non_invertible() {
            let a = geometry::mat4! {
                -4.0, 2.0, -2.0, -3.0, 9.0, 6.0, 2.0, 6.0, 0.0, -5.0, 1.0, -5.0, 0.0, 0.0, 0.0,
                0.0,
            };
            assert_eq!(a.determinant(), 0.0);
            assert!(!a.is_invertible())
        }
        #[test]
        fn calculating_inverse_1() {
            let a = geometry::mat4! {
                -5.0, 2.0, 6.0, -8.0, 1.0, -5.0, 1.0, 8.0, 7.0, 7.0, -6.0, -7.0, 1.0, -3.0, 7.0,
                4.0,
            };
            // Assert determinants and cofactors
            assert_eq!(a.determinant(), 532.0);
            assert_eq!(a.cofactor(2, 3), -160.0);
            assert_eq!(a.cofactor(3, 2), 105.0);

            let i = a.inverse().unwrap();

            // Assert manual calculation gives same results
            assert_eq!(i[(3, 2)], -160.0 / 532.0);
            assert_eq!(i[(2, 3)], 105.0 / 532.0);

            // Assert row-by-row equality
            let rows = i.as_row_vectors();
            assert_eq!(rows[0], Vert4::new(0.21805, 0.45113, 0.24060, -0.04511));
            assert_eq!(rows[1], Vert4::new(-0.80827, -1.45677, -0.44361, 0.52068));
            assert_eq!(rows[2], Vert4::new(-0.07895, -0.22368, -0.05263, 0.19737));
            assert_eq!(rows[3], Vert4::new(-0.52256, -0.81391, -0.30075, 0.30639));
        }
        #[test]
        fn calculating_inverse_2() {
            let a = geometry::mat4! {
                8.0, -5.0, 9.0, 2.0, 7.0, 5.0, 6.0, 1.0, -6.0, 0.0, 9.0, 6.0, -3.0, 0.0, -9.0,
                -4.0,
            };

            let i = a.inverse().unwrap();

            // Assert row-by-row equality
            let rows = i.as_row_vectors();
            assert_eq!(rows[0], Vert4::new(-0.15385, -0.15385, -0.28205, -0.53846));
            assert_eq!(rows[1], Vert4::new(-0.07692, 0.12308, 0.02564, 0.03077));
            assert_eq!(rows[2], Vert4::new(0.35897, 0.35897, 0.43590, 0.92308));
            assert_eq!(rows[3], Vert4::new(-0.69231, -0.69231, -0.76923, -1.92308));
        }
        #[test]
        fn calculating_inverse_3() {
            let m = geometry::mat4! {
                9.0, 3.0, 0.0, 9.0, -5.0, -2.0, -6.0, -3.0, -4.0, 9.0, 6.0, 4.0, -7.0, 6.0, 6.0,
                2.0,
            };

            let i = m.inverse().unwrap();

            // Assert row-by-row equality
            let rows = i.as_row_vectors();
            assert_eq!(rows[0], Vert4::new(-0.04074, -0.07778, 0.14444, -0.22222));
            assert_eq!(rows[1], Vert4::new(-0.07778, 0.03333, 0.36667, -0.33333));
            assert_eq!(rows[2], Vert4::new(-0.02901, -0.14630, -0.10926, 0.12963));
            assert_eq!(rows[3], Vert4::new(0.17778, 0.06667, -0.26667, 0.33333));
        }
        #[test]
        fn matrix_product_and_product_with_inverse_of_second_gives_original() {
            let m1 = geometry::mat4! {
                3.0, -9.0, 7.0, 3.0, 3.0, -8.0, 2.0, -9.0, -4.0, 4.0, 4.0, 1.0, -6.0, 5.0, -1.0,
                1.0,
            };
            let m2 = geometry::mat4! {
                8.0, 2.0, 2.0, 2.0, 3.0, -1.0, 7.0, 0.0, 7.0, 0.0, 5.0, 4.0, 6.0, -2.0, 0.0, 5.0,
            };
            let m1_m2_prod = &m1 * &m2;
            let i2 = m2.inverse().unwrap();
            let m1_m2_prod_i2_prod = m1_m2_prod * &i2;
            assert_eq!(m1_m2_prod_i2_prod, m1);
        }
    }
}
