//! A platform agnostic driver to interface with the TM1637 (4-digit 7-segment display) using the [`embedded-hal`](embedded_hal) and [`embedded-hal-async`](embedded_hal_async) traits.
//!
//! # Features
//! The following features are available:
//! - `blocking`: enables blocking functionality.
//! - `async`: enables asynchronous functionality.
//! - `mappings`: enables the mappings module.
//! - `demo`: enables the demo module.
//! - `disable-checks`: disables bound checks while writing to the display. When enabled, positions greater than available positions on the display will be written to the display regardless, causing more delay than needed. Enable this feature only if you are sure about the positions you are writing to.

#![no_std]
#![deny(unsafe_code)]
#![deny(missing_docs)]

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

#[cfg(feature = "demo")]
pub mod demo;
pub mod device;
pub mod functionality;
#[cfg(feature = "mappings")]
pub mod mappings;
