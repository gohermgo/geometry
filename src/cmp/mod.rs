//! Comparisons between geometry
use crate::ops::AbsDiff;
/// SortaEq: Sort of equal, just takes some epsilon
pub trait SortaEq: PartialOrd<Self> + AbsDiff<Self>
where
    Self: Sized,
{
    type Rhs;
    fn ehh_maybe(&self, rhs: &Self::Rhs) -> bool;
}
