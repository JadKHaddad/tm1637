//! A platform agnostic driver to interface with the `TM1637` (7-segment display) using the [`embedded-hal`](embedded_hal) and [`embedded-hal-async`](embedded_hal_async) traits.
//!
//! ```rust
//! use tm1637_embedded_hal::{mock::Noop, Brightness, TM1637Builder};
//!
//! let mut tm = TM1637Builder::new(Noop, Noop, Noop)
//!     .brightness(Brightness::L3)
//!     .delay_us(100)
//!     // Blocking or async mode
//!     .build_blocking::<6>();
//!
//! // Clear the display and set brightness
//! tm.init().ok();
//!
//! // High-Level fluent API
//! tm.options()
//!     .str("HELLO. ruSt.")
//!     .scroll()
//!     .linear()
//!     .finish()
//!     .run();
//!
//! // Or Low-Level API
//! let bytes = &[0b00000110, 0b01011011, 0b01001111, 0b01100110]; // `1234`
//!
//! tm.display_slice(0, bytes).ok();
//! ```
//!
//! # Features
//!
//! - `ack`: Enables the driver to use the [`InputPin`](https://docs.rs/embedded-hal/latest/embedded_hal/digital/trait.InputPin.html) trait for the `DIO` pin and wait for the acknowledgment signal from the display.
//! - `defmt`: Implements [`defmt::Format`](https://docs.rs/defmt/latest/defmt/trait.Format.html) for structs and enums.

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
pub(crate) use conditional::ConditionalInputPin;
pub use device::TM1637;
pub use error::Error;
pub(crate) use identity::Identity;
