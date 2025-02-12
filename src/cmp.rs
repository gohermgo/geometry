use crate::Vert4;
pub trait SortaEq<Rhs = Self> {
    fn ehh_maybe(&self, rhs: &Rhs) -> bool;
}
impl SortaEq for f32 {
    fn ehh_maybe(&self, rhs: &Self) -> bool {
        (self - rhs).abs() < 1e-4
    }
}
impl SortaEq for Vert4 {
    #[inline(always)]
    fn ehh_maybe(&self, rhs: &Self) -> bool {
        let abs_diff = std::simd::num::SimdFloat::abs(self.0 - rhs.0);
        println!("AD {abs_diff:?}");
        let ad = self - &rhs;
        println!("AD {ad:?}");
        let x_sorta = ad.x() < 1e-4;
        let y_sorta = ad.y() < 1e-4;
        let z_sorta = ad.z() < 1e-4;
        let w_sorta = ad.w() < 1e-4;
        x_sorta && y_sorta && z_sorta && w_sorta
    }
}
