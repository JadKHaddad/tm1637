//! The most advanced platform agnostic driver to interface with the `TM1637` (7-segment display) using the [`embedded-hal`](embedded_hal) and [`embedded-hal-async`](embedded_hal_async) traits.
//!
//! # Features
//!
//! The following features are available:
//! - `ack`: enables the driver to use the `InputPin` trait for the `DIO` pin and wait for the acknowledgment signal from the display.
//! - `defmt`: implements `defmt::Format` for structs and enums.
//! - `demo`: enables the demo module.

#![no_std]
// #![deny(unsafe_code, missing_docs, missing_debug_implementations)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[doc(hidden)]
pub mod mock;
// #[cfg(feature = "demo")]
// #[cfg_attr(docsrs, doc(cfg(feature = "demo")))]
// pub mod demo;

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
mod mode;
mod numbers;
mod options;
mod rotating_circle;
pub mod scroll;
mod str_parser;
pub mod tokens;
pub mod windows;

pub use brightness::Brightness;
pub use builder::TM1637Builder;
pub(crate) use conditional::ConditionalInputPin;
pub use device::TM1637;
pub use error::Error;
pub(crate) use identity::Identity;
pub use maybe_flipped::MaybeFlipped;
pub use options::{DisplayOptions, ScrollDisplayOptions};
