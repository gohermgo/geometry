//! Operators for geometry
mod cross;
pub use cross::Cross;
mod dot;
pub use dot::Dot;
mod mag;
pub use mag::Mag;
mod norm;
pub use norm::{Norm, NormAssign};
