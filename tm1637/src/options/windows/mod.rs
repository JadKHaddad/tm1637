//! Create windows from iterators and slices.

mod circular;
mod functions;
mod linear;
mod reverse;

pub use circular::{CircularWindows, CircularWindowsReversed};
pub use functions::*;
pub use linear::LinearWindows;
