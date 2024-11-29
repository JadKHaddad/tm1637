//! A platform agnostic driver to interface with the `TM1637` (7-segment display) using the [`embedded-hal`](embedded_hal) and [`embedded-hal-async`](embedded_hal_async) traits.
//!
//! ## Features
//! The following features are available:
//! - `ack`: enables the driver to use the `InputPin` trait for the `DIO` pin and wait for the acknowledgment signal from the display.
//! - `defmt`: implements `defmt::Format` for structs and enums.
//! - `demo`: enables the demo module.

#![no_std]
#![deny(unsafe_code, missing_docs, missing_debug_implementations)]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod animitaion;
mod brightness;
mod builder;
mod conditional;
#[cfg(feature = "demo")]
#[cfg_attr(docsrs, doc(cfg(feature = "demo")))]
pub mod demo;
mod device;
mod direction;
mod error;
pub mod formatters;
mod identity;
pub mod mappings;
mod options;

pub use animitaion::AnimationStyle;
pub use brightness::Brightness;
pub use builder::TM1637Builder;
pub(crate) use conditional::ConditionalInputPin;
pub use device::{asynch::TM1637 as AsyncTM1637, blocking::TM1637 as BlockingTM1637};
pub use direction::Direction;
pub use error::Error;
pub(crate) use identity::Identity;
pub use options::{
    asynch::{
        DisplayOptions as AsyncDisplayOptions, Flipped as AsyncFlipped,
        NotFlipped as AsyncNotFlipped,
    },
    blocking::{
        DisplayOptions as BlockingDisplayOptions, Flipped as BlockingFlipped,
        NotFlipped as BlockingNotFlipped,
    },
};
