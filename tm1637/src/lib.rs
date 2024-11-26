//! A platform agnostic driver to interface with the `TM1637` (7-segment display) using the [`embedded-hal`](embedded_hal) and [`embedded-hal-async`](embedded_hal_async) traits.
//!
//! ## Features
//! The following features are available:
//! - `blocking`: enables blocking functionality.
//! - `async`: enables asynchronous functionality.
//! - `defmt`: implements `defmt::Format` for structs and enums.
//! - `demo`: enables the demo module.
//! - `disable-checks`: disables bound checks while writing to the display. When enabled, positions greater than available positions on the display will be written to the display regardless, causing more delay than needed. Enable this feature only if you are sure about the positions you are writing to.

#![no_std]
#![deny(unsafe_code)]
#![deny(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(any(feature = "async", feature = "blocking"))]
/// Our custom `try!` macro aka `?`, to get rid of [`core::convert::From`]/[`core::convert::Into`] used by the `?` operator.
macro_rules! tri {
    ($e:expr $(,)?) => {
        match $e {
            core::result::Result::Ok(value) => value,
            core::result::Result::Err(err) => {
                return core::result::Result::Err(err);
            }
        }
    };
}

mod brightness;
mod device;

pub use brightness::Brightness;

#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
pub use crate::device::asynch;

#[cfg(feature = "blocking")]
#[cfg_attr(docsrs, doc(cfg(feature = "blocking")))]
pub use crate::device::blocking;

#[cfg(feature = "demo")]
#[cfg_attr(docsrs, doc(cfg(feature = "demo")))]
pub mod demo;

#[cfg_attr(docsrs, doc(cfg(feature = "mappings")))]
pub mod mappings;

#[cfg_attr(docsrs, doc(cfg(feature = "formatters")))]
pub mod formatters;
