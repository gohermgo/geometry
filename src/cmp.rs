use std::simd::{cmp::SimdPartialEq, LaneCount, Simd, SimdElement, SupportedLaneCount};

use crate::Vert4;
pub trait SortaEq<Rhs = Self> {
    fn ehh_maybe(&self, rhs: &Rhs) -> bool;
}
impl SortaEq for f32 {
    fn ehh_maybe(&self, rhs: &Self) -> bool {
        (self - rhs).abs() < 1e-4
    }
}
impl<const N: usize> SortaEq for Simd<f32, N>
where
    LaneCount<N>: SupportedLaneCount,
{
    fn ehh_maybe(&self, rhs: &Self) -> bool {
        println!();
        self.as_array()
            .iter()
            .zip(rhs.as_array().iter())
            .all(|(lhs, rhs)| {
                let abs_diff = f32::max(*lhs, *rhs) - f32::min(*lhs, *rhs);
                let cond = abs_diff < 1e-4;
                println!("A = {lhs}, B = {rhs}, AD = {abs_diff} -> {cond}");
                cond
            })
    }
}
