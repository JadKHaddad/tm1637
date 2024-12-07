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

mod brightness;
mod builder;
mod conditional;
#[doc(hidden)]
pub mod mock;

// #[cfg(feature = "demo")]
// #[cfg_attr(docsrs, doc(cfg(feature = "demo")))]
// pub mod demo;
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

use crate::exact_size::ExactSizeChainExt;

pub fn align(
    position: usize,
    iter: impl DoubleEndedIterator<Item = u8> + ExactSizeIterator,
) -> (usize, impl Iterator<Item = u8>) {
    #[auto_enums::enum_derive(Iterator)]
    enum I<A, B> {
        A(A),
        B(B),
    }

    if position > 6 {
        return (0, I::A(core::iter::empty()));
    }

    let iter = padding(iter).take(6 - position).rev();

    // 3 is a magic number to make the alignment work
    (3, I::B(iter))
}

fn padding(
    iter: impl DoubleEndedIterator<Item = u8> + ExactSizeIterator,
) -> impl DoubleEndedIterator<Item = u8> + ExactSizeIterator {
    #[auto_enums::enum_derive(DoubleEndedIterator)]
    enum I<A, B> {
        A(A),
        B(B),
    }

    impl<A: ExactSizeIterator<Item = u8>, B: ExactSizeIterator<Item = u8>> ExactSizeIterator
        for I<A, B>
    {
        fn len(&self) -> usize {
            match self {
                I::A(a) => a.len(),
                I::B(b) => b.len(),
            }
        }
    }

    let len = iter.len();

    // less than 6 digits will never align whithout overwriting the last digits, so we pad with zeros
    if len < 6 {
        return I::A(iter.exact_size_chain(core::iter::repeat(0).take(6 - len)));
    };

    I::B(iter)
}
