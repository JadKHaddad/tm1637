//! A platform agnostic driver to interface with the `TM1637` (7-segment display) using the [`embedded-hal`](embedded_hal) and [`embedded-hal-async`](embedded_hal_async) traits.
//!
//! # Features
//!
//! The following features are available:
//! - `ack`: enables the driver to use the `InputPin` trait for the `DIO` pin and wait for the acknowledgment signal from the display.
//! - `defmt`: implements `defmt::Format` for structs and enums.

#![no_std]
#![deny(unsafe_code, missing_docs, missing_debug_implementations)]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod align;
mod brightness;
mod builder;
mod conditional;
mod device;
mod error;
mod exact_size;
pub mod formatters;
mod identity;
pub mod mappings;
mod maybe_flipped;
#[doc(hidden)]
pub mod mock;
mod mode;
pub mod numbers;
pub mod options;
pub mod str;
pub mod tokens;

pub use brightness::Brightness;
pub use builder::TM1637Builder;
pub use conditional::ConditionalInputPin;
pub use device::TM1637;
pub use error::Error;
pub(crate) use identity::Identity;
