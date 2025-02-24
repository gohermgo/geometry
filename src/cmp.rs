use std::simd::{num::SimdFloat, LaneCount, Simd, SupportedLaneCount};

pub trait SortaEq<Rhs = Self> {
    fn ehh_maybe(&self, rhs: &Rhs) -> bool;
}
impl SortaEq for f32 {
    fn ehh_maybe(&self, rhs: &Self) -> bool {
        (self - rhs).abs() < 1e-4
    }
}
impl<const N: usize> SortaEq for [f32; N] {
    fn ehh_maybe(&self, rhs: &Self) -> bool {
        self.iter().zip(rhs.iter()).all(|(lhs, rhs)| {
            let abs_diff = f32::max(*lhs, *rhs) - f32::min(*lhs, *rhs);
            abs_diff < 1e-4
        })
    }
}
impl<const N: usize> SortaEq for Simd<f32, N>
where
    LaneCount<N>: SupportedLaneCount,
{
    fn ehh_maybe(&self, rhs: &Self) -> bool {
        Simd::lt(&SimdFloat::abs(self - rhs), &Simd::splat(1e-4))
    }
}
