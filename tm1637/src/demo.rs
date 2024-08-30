//! This module contains a demo implementation for the `TM1637` device.
//!
//! This module is only available when the `demo` feature of this
//! library is activated.

#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
pub mod asynch;
#[cfg(feature = "blocking")]
#[cfg_attr(docsrs, doc(cfg(feature = "blocking")))]
pub mod blocking;
