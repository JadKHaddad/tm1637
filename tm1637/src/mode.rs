//! Async or blocking mode.

/// Async or blocking mode.
///
/// Implemented by [`Async`](crate::tokens::Async) and [`Blocking`](crate::tokens::Blocking) and used by the [`TM1637Builder`](crate::builder::TM1637Builder).
pub trait Mode {}
