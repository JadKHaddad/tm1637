//! This module contains a demo implementation for the `TM1637` device.

#[cfg(feature = "async")]
pub mod asynch;
#[cfg(feature = "blocking")]
pub mod blocking;
