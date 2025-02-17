use core::marker::PhantomData;
use core::ops::Deref;

use crate::{Mat4, Matrix};

pub struct TranslationMatrix(pub Mat4);
impl TranslationMatrix {
    pub fn new(x: f32, y: f32, z: f32) -> TranslationMatrix {
        TranslationMatrix(Mat4::new([
            1.0, 0.0, 0.0, x, 0.0, 1.0, 0.0, y, 0.0, 0.0, 1.0, z, 0.0, 0.0, 0.0, 1.0,
        ]))
    }
}
impl Deref for TranslationMatrix {
    type Target = Mat4;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[test]
fn multiplying_by_a_translation_matrix() {
    use crate::cmp::SortaEq;
    use crate::Vert4;

    let transform = TranslationMatrix::new(5., -3., 2.);
    let p = Vert4::new(-3., 4., 5., 1.);

    let res = transform.deref() * p;
    assert!(res.ehh_maybe(&Vert4::new(2., 1., 7., 1.)))
}
#[test]
fn multiplying_by_the_inverse_of_a_translation_matrix() {
    use crate::cmp::SortaEq;
    use crate::matrix::ops::Inverse;
    use crate::Vert4;

    let transform = TranslationMatrix::new(5., -3., 2.);
    let p = Vert4::new(-3., 4., 5., 1.);
    let inv = transform.inverse().unwrap();
    let res = inv * p;
    assert!(res.ehh_maybe(&Vert4::new(-8., 7., 3., 1.)))
}
#[test]
fn translation_does_not_affect_vectors() {
    use crate::cmp::SortaEq;
    use crate::Vert4;

    let transform = TranslationMatrix::new(5., -3., 2.);
    let v = Vert4::new(-3., 4., 5., 0.);
    let res = transform.deref() * &v;
    assert!(res.ehh_maybe(&v))
}
