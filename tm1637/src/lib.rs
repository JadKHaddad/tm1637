//! A platform agnostic driver to interface with the `TM1637` (7-segment display) using the [`embedded-hal`](embedded_hal) and [`embedded-hal-async`](embedded_hal_async) traits.
//!
//! ## Features
//! The following features are available:
//! - `blocking`: enables blocking functionality.
//! - `async`: enables asynchronous functionality.
//! - `defmt`: implements `defmt::Format` for structs and enums.
//! - `demo`: enables the demo module.
//! - `disable-checks`: disables bound checks while writing to the display. When enabled, positions greater than available positions on the display will be written to the display regardless, causing more delay than needed. Enable this feature only if you are sure about the positions you are writing to.

// TODO: ack feature

#![no_std]
#![deny(unsafe_code, missing_docs, missing_debug_implementations)]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod brightness;
mod conditional;
#[cfg(feature = "demo")]
#[cfg_attr(docsrs, doc(cfg(feature = "demo")))]
pub mod demo;
mod device;
mod error;
pub mod formatters;
pub mod mappings;

pub use brightness::Brightness;
pub use conditional::ConditionalInputPin;
pub use error::Error;

#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
pub use crate::device::asynch;

#[cfg(feature = "blocking")]
#[cfg_attr(docsrs, doc(cfg(feature = "blocking")))]
pub use crate::device::blocking;
