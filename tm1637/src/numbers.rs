//! Format signed and unsigned integers into byte arrays.
//!
//! These byte arrays can be sent directly to a TM1637 display.
//!
//! # Example
//!
//! ```
//! # use tm1637_embedded_hal::{formatters::i16_to_4digits, mock::Noop, numbers, TM1637Builder};
//! let mut tm = TM1637Builder::new(Noop, Noop, Noop).build_blocking::<4>();
//!
//! tm.init().ok();
//!
//! tm.display_slice(0, &numbers::i16_3(-123));
//! ```
//!
//! The display will show:
//!
//! ```text
//! +---+ +---+ +---+ +---+
//! | - | | 1 | | 2 | | 3 |
//! +---+ +---+ +---+ +---+
//! ```

/*
TLDR; this module is a mess.

fn is_even(number: i32) -> bool {
    if number == 1 { false }
    else if number == 2 { true }
    else if number == 3 { false }
    else if number == 4 { true }
    else if number == 5 { false }
    else if number == 6 { true }
    else if number == 7 { false }
    else if number == 8 { true }
    else if number == 9 { false }
    else if number == 10 { true }
    else if number == 11 { false }
    else if number == 12 { true }
    else if number == 13 { false }
    else if number == 14 { true }
    else if number == 15 { false }
    else if number == 16 { true }
    else if number == 17 { false }
    else if number == 18 { true }
    else if number == 19 { false }
    else if number == 20 { true }
    else if number == 21 { false }
    else if number == 22 { true }
    else { panic!("Number not supported!") }
}
*/

use crate::mappings::{DigitBits, SpecialCharBits};

/// Format a [`prim@u8`] into its 7-segment byte representation as a `[u8; 1]` aligning the bytes to the left.
///
/// # Example
///
/// ```
/// # use tm1637_embedded_hal::{numbers::u8, mappings::DigitBits};
/// let formatted = u8(1);
///
/// assert_eq!(formatted, [DigitBits::One as u8]);
/// ```
pub fn u8(n: u8) -> [u8; 1] {
    let n = n.clamp(0, 9);

    [DigitBits::from_digit(n) as u8]
}

/// Format a [`prim@u8`] into its 7-segment byte representation as a `[u8; 2]` aligning the bytes to the left.
///
/// # Example
///
/// ```
/// # use tm1637_embedded_hal::{numbers::u8_2, mappings::DigitBits};
/// let formatted = u8_2(1);
/// assert_eq!(formatted, [DigitBits::One as u8, 0]);
///
/// let formatted = u8_2(12);
/// assert_eq!(formatted, [DigitBits::One as u8, DigitBits::Two as u8]);
/// ```
pub fn u8_2(n: u8) -> [u8; 2] {
    let n = n.clamp(0, 99);

    if n > 9 {
        [
            DigitBits::from_digit(n / 10) as u8,
            DigitBits::from_digit(n % 10) as u8,
        ]
    } else {
        [DigitBits::from_digit(n) as u8, 0]
    }
}

/// Format a [`prim@u8`] into its 7-segment byte representation as a `[u8; 2]` aligning the bytes to the right.
///
/// # Example
///
/// ```
/// # use tm1637_embedded_hal::{numbers::r_u8_2, mappings::DigitBits};
/// let formatted = r_u8_2(1);
/// assert_eq!(formatted, [0, DigitBits::One as u8]);
///
/// let formatted = r_u8_2(12);
/// assert_eq!(formatted, [DigitBits::One as u8, DigitBits::Two as u8]);
/// ```
pub fn r_u8_2(n: u8) -> [u8; 2] {
    let n = n.clamp(0, 99);

    if n > 9 {
        [
            DigitBits::from_digit(n / 10) as u8,
            DigitBits::from_digit(n % 10) as u8,
        ]
    } else {
        [0, DigitBits::from_digit(n) as u8]
    }
}

/// Format a [`u16`] into its 7-segment byte representation as a `[u8; 3]` aligning the bytes to the left.
///
/// # Example
///
/// ```
/// # use tm1637_embedded_hal::{numbers::u16_3, mappings::DigitBits};
/// let formatted = u16_3(1);
/// assert_eq!(formatted, [DigitBits::One as u8, 0, 0]);
///
/// let formatted = u16_3(123);
/// assert_eq!(formatted, [DigitBits::One as u8, DigitBits::Two as u8, DigitBits::Three as u8]);
/// ```
pub fn u16_3(n: u16) -> [u8; 3] {
    let n = n.clamp(0, 999);

    if n > 99 {
        [
            DigitBits::from_digit((n / 100) as u8) as u8,
            DigitBits::from_digit(((n / 10) % 10) as u8) as u8,
            DigitBits::from_digit((n % 10) as u8) as u8,
        ]
    } else if n > 9 {
        [
            DigitBits::from_digit((n / 10) as u8) as u8,
            DigitBits::from_digit((n % 10) as u8) as u8,
            0,
        ]
    } else {
        [DigitBits::from_digit((n % 10) as u8) as u8, 0, 0]
    }
}

/// Format a [`u16`] into its 7-segment byte representation as a `[u8; 3]` aligning the bytes to the right.
///
/// # Example
///
/// ```
/// # use tm1637_embedded_hal::{numbers::r_u16_3, mappings::DigitBits};
/// let formatted = r_u16_3(1);
/// assert_eq!(formatted, [0, 0, DigitBits::One as u8]);
///
/// let formatted = r_u16_3(123);
/// assert_eq!(formatted, [DigitBits::One as u8, DigitBits::Two as u8, DigitBits::Three as u8]);
/// ```
pub fn r_u16_3(n: u16) -> [u8; 3] {
    let n = n.clamp(0, 999);

    if n > 99 {
        [
            DigitBits::from_digit((n / 100) as u8) as u8,
            DigitBits::from_digit(((n / 10) % 10) as u8) as u8,
            DigitBits::from_digit((n % 10) as u8) as u8,
        ]
    } else if n > 9 {
        [
            0,
            DigitBits::from_digit((n / 10) as u8) as u8,
            DigitBits::from_digit((n % 10) as u8) as u8,
        ]
    } else {
        [0, 0, DigitBits::from_digit((n % 10) as u8) as u8]
    }
}

/// Format a [`u16`] into its 7-segment byte representation as a `[u8; 4]` aligning the bytes to the left.
///
/// # Example
///
/// ```
/// # use tm1637_embedded_hal::{numbers::u16_4, mappings::DigitBits};
/// let formatted = u16_4(1);
/// assert_eq!(formatted, [DigitBits::One as u8, 0, 0, 0]);
///
/// let formatted = u16_4(1234);
/// assert_eq!(formatted, [DigitBits::One as u8, DigitBits::Two as u8, DigitBits::Three as u8, DigitBits::Four as u8]);
/// ```
pub fn u16_4(n: u16) -> [u8; 4] {
    let n = n.clamp(0, 9999);

    if n > 999 {
        [
            DigitBits::from_digit((n / 1000) as u8) as u8,
            DigitBits::from_digit(((n / 100) % 10) as u8) as u8,
            DigitBits::from_digit(((n / 10) % 10) as u8) as u8,
            DigitBits::from_digit((n % 10) as u8) as u8,
        ]
    } else if n > 99 {
        [
            DigitBits::from_digit((n / 100) as u8) as u8,
            DigitBits::from_digit(((n / 10) % 10) as u8) as u8,
            DigitBits::from_digit((n % 10) as u8) as u8,
            0,
        ]
    } else if n > 9 {
        [
            DigitBits::from_digit((n / 10) as u8) as u8,
            DigitBits::from_digit((n % 10) as u8) as u8,
            0,
            0,
        ]
    } else {
        [DigitBits::from_digit((n % 10) as u8) as u8, 0, 0, 0]
    }
}

/// Format a [`u16`] into its 7-segment byte representation as a `[u8; 4]` aligning the bytes to the right.
///
/// # Example
///
/// ```
/// # use tm1637_embedded_hal::{numbers::r_u16_4, mappings::DigitBits};
/// let formatted = r_u16_4(1);
/// assert_eq!(formatted, [0, 0, 0, DigitBits::One as u8]);
///
/// let formatted = r_u16_4(1234);
/// assert_eq!(formatted, [DigitBits::One as u8, DigitBits::Two as u8, DigitBits::Three as u8, DigitBits::Four as u8]);
/// ```
pub fn r_u16_4(n: u16) -> [u8; 4] {
    let n = n.clamp(0, 9999);

    if n > 999 {
        [
            DigitBits::from_digit((n / 1000) as u8) as u8,
            DigitBits::from_digit(((n / 100) % 10) as u8) as u8,
            DigitBits::from_digit(((n / 10) % 10) as u8) as u8,
            DigitBits::from_digit((n % 10) as u8) as u8,
        ]
    } else if n > 99 {
        [
            0,
            DigitBits::from_digit((n / 100) as u8) as u8,
            DigitBits::from_digit(((n / 10) % 10) as u8) as u8,
            DigitBits::from_digit((n % 10) as u8) as u8,
        ]
    } else if n > 9 {
        [
            0,
            0,
            DigitBits::from_digit((n / 10) as u8) as u8,
            DigitBits::from_digit((n % 10) as u8) as u8,
        ]
    } else {
        [0, 0, 0, DigitBits::from_digit((n % 10) as u8) as u8]
    }
}

/// Format a [`u32`] into its 7-segment byte representation as a `[u8; 5]` aligning the bytes to the left.
///
/// # Example
///
/// ```
/// # use tm1637_embedded_hal::{numbers::u32_5, mappings::DigitBits};
/// let formatted = u32_5(1);
/// assert_eq!(formatted, [DigitBits::One as u8, 0, 0, 0, 0]);
///
/// let formatted = u32_5(12345);
/// assert_eq!(formatted, [DigitBits::One as u8, DigitBits::Two as u8, DigitBits::Three as u8, DigitBits::Four as u8, DigitBits::Five as u8]);
/// ```
pub fn u32_5(n: u32) -> [u8; 5] {
    let n = n.clamp(0, 99999);

    if n > 9999 {
        [
            DigitBits::from_digit((n / 10000) as u8) as u8,
            DigitBits::from_digit(((n / 1000) % 10) as u8) as u8,
            DigitBits::from_digit(((n / 100) % 10) as u8) as u8,
            DigitBits::from_digit(((n / 10) % 10) as u8) as u8,
            DigitBits::from_digit((n % 10) as u8) as u8,
        ]
    } else if n > 999 {
        [
            DigitBits::from_digit((n / 1000) as u8) as u8,
            DigitBits::from_digit(((n / 100) % 10) as u8) as u8,
            DigitBits::from_digit(((n / 10) % 10) as u8) as u8,
            DigitBits::from_digit((n % 10) as u8) as u8,
            0,
        ]
    } else if n > 99 {
        [
            DigitBits::from_digit((n / 100) as u8) as u8,
            DigitBits::from_digit(((n / 10) % 10) as u8) as u8,
            DigitBits::from_digit((n % 10) as u8) as u8,
            0,
            0,
        ]
    } else if n > 9 {
        [
            DigitBits::from_digit((n / 10) as u8) as u8,
            DigitBits::from_digit((n % 10) as u8) as u8,
            0,
            0,
            0,
        ]
    } else {
        [DigitBits::from_digit((n % 10) as u8) as u8, 0, 0, 0, 0]
    }
}

/// Format a [`u32`] into its 7-segment byte representation as a `[u8; 5]` aligning the bytes to the right.
///
/// # Example
///
/// ```
/// # use tm1637_embedded_hal::{numbers::r_u32_5, mappings::DigitBits};
/// let formatted = r_u32_5(1);
/// assert_eq!(formatted, [0, 0, 0, 0, DigitBits::One as u8]);
///
/// let formatted = r_u32_5(12345);
/// assert_eq!(formatted, [DigitBits::One as u8, DigitBits::Two as u8, DigitBits::Three as u8, DigitBits::Four as u8, DigitBits::Five as u8]);
/// ```
pub fn r_u32_5(n: u32) -> [u8; 5] {
    let n = n.clamp(0, 99999);

    if n > 9999 {
        [
            DigitBits::from_digit((n / 10000) as u8) as u8,
            DigitBits::from_digit(((n / 1000) % 10) as u8) as u8,
            DigitBits::from_digit(((n / 100) % 10) as u8) as u8,
            DigitBits::from_digit(((n / 10) % 10) as u8) as u8,
            DigitBits::from_digit((n % 10) as u8) as u8,
        ]
    } else if n > 999 {
        [
            0,
            DigitBits::from_digit((n / 1000) as u8) as u8,
            DigitBits::from_digit(((n / 100) % 10) as u8) as u8,
            DigitBits::from_digit(((n / 10) % 10) as u8) as u8,
            DigitBits::from_digit((n % 10) as u8) as u8,
        ]
    } else if n > 99 {
        [
            0,
            0,
            DigitBits::from_digit((n / 100) as u8) as u8,
            DigitBits::from_digit(((n / 10) % 10) as u8) as u8,
            DigitBits::from_digit((n % 10) as u8) as u8,
        ]
    } else if n > 9 {
        [
            0,
            0,
            0,
            DigitBits::from_digit((n / 10) as u8) as u8,
            DigitBits::from_digit((n % 10) as u8) as u8,
        ]
    } else {
        [0, 0, 0, 0, DigitBits::from_digit((n % 10) as u8) as u8]
    }
}

/// Format a [`u32`] into its 7-segment byte representation as a `[u8; 6]` aligning the bytes to the left.
///
/// # Example
///
/// ```
/// # use tm1637_embedded_hal::{numbers::u32_6, mappings::DigitBits};
/// let formatted = u32_6(1);
/// assert_eq!(formatted, [DigitBits::One as u8, 0, 0, 0, 0, 0]);
///
/// let formatted = u32_6(123456);
/// assert_eq!(formatted, [DigitBits::One as u8, DigitBits::Two as u8, DigitBits::Three as u8, DigitBits::Four as u8, DigitBits::Five as u8, DigitBits::Six as u8]);
/// ```
pub fn u32_6(n: u32) -> [u8; 6] {
    let n = n.clamp(0, 999999);

    if n > 99999 {
        [
            DigitBits::from_digit((n / 100000) as u8) as u8,
            DigitBits::from_digit(((n / 10000) % 10) as u8) as u8,
            DigitBits::from_digit(((n / 1000) % 10) as u8) as u8,
            DigitBits::from_digit(((n / 100) % 10) as u8) as u8,
            DigitBits::from_digit(((n / 10) % 10) as u8) as u8,
            DigitBits::from_digit((n % 10) as u8) as u8,
        ]
    } else if n > 9999 {
        [
            DigitBits::from_digit((n / 10000) as u8) as u8,
            DigitBits::from_digit(((n / 1000) % 10) as u8) as u8,
            DigitBits::from_digit(((n / 100) % 10) as u8) as u8,
            DigitBits::from_digit(((n / 10) % 10) as u8) as u8,
            DigitBits::from_digit((n % 10) as u8) as u8,
            0,
        ]
    } else if n > 999 {
        [
            DigitBits::from_digit((n / 1000) as u8) as u8,
            DigitBits::from_digit(((n / 100) % 10) as u8) as u8,
            DigitBits::from_digit(((n / 10) % 10) as u8) as u8,
            DigitBits::from_digit((n % 10) as u8) as u8,
            0,
            0,
        ]
    } else if n > 99 {
        [
            DigitBits::from_digit((n / 100) as u8) as u8,
            DigitBits::from_digit(((n / 10) % 10) as u8) as u8,
            DigitBits::from_digit((n % 10) as u8) as u8,
            0,
            0,
            0,
        ]
    } else if n > 9 {
        [
            DigitBits::from_digit((n / 10) as u8) as u8,
            DigitBits::from_digit((n % 10) as u8) as u8,
            0,
            0,
            0,
            0,
        ]
    } else {
        [DigitBits::from_digit((n % 10) as u8) as u8, 0, 0, 0, 0, 0]
    }
}

/// Format a [`u32`] into its 7-segment byte representation as a `[u8; 6]` aligning the bytes to the right.
///
/// # Example
///
/// ```
/// # use tm1637_embedded_hal::{numbers::r_u32_6, mappings::DigitBits};
/// let formatted = r_u32_6(1);
/// assert_eq!(formatted, [0, 0, 0, 0, 0, DigitBits::One as u8]);
///
/// let formatted = r_u32_6(123456);
/// assert_eq!(formatted, [DigitBits::One as u8, DigitBits::Two as u8, DigitBits::Three as u8, DigitBits::Four as u8, DigitBits::Five as u8, DigitBits::Six as u8]);
/// ```
pub fn r_u32_6(n: u32) -> [u8; 6] {
    let n = n.clamp(0, 999999);

    if n > 99999 {
        [
            DigitBits::from_digit((n / 100000) as u8) as u8,
            DigitBits::from_digit(((n / 10000) % 10) as u8) as u8,
            DigitBits::from_digit(((n / 1000) % 10) as u8) as u8,
            DigitBits::from_digit(((n / 100) % 10) as u8) as u8,
            DigitBits::from_digit(((n / 10) % 10) as u8) as u8,
            DigitBits::from_digit((n % 10) as u8) as u8,
        ]
    } else if n > 9999 {
        [
            0,
            DigitBits::from_digit((n / 10000) as u8) as u8,
            DigitBits::from_digit(((n / 1000) % 10) as u8) as u8,
            DigitBits::from_digit(((n / 100) % 10) as u8) as u8,
            DigitBits::from_digit(((n / 10) % 10) as u8) as u8,
            DigitBits::from_digit((n % 10) as u8) as u8,
        ]
    } else if n > 999 {
        [
            0,
            0,
            DigitBits::from_digit((n / 1000) as u8) as u8,
            DigitBits::from_digit(((n / 100) % 10) as u8) as u8,
            DigitBits::from_digit(((n / 10) % 10) as u8) as u8,
            DigitBits::from_digit((n % 10) as u8) as u8,
        ]
    } else if n > 99 {
        [
            0,
            0,
            0,
            DigitBits::from_digit((n / 100) as u8) as u8,
            DigitBits::from_digit(((n / 10) % 10) as u8) as u8,
            DigitBits::from_digit((n % 10) as u8) as u8,
        ]
    } else if n > 9 {
        [
            0,
            0,
            0,
            0,
            DigitBits::from_digit((n / 10) as u8) as u8,
            DigitBits::from_digit((n % 10) as u8) as u8,
        ]
    } else {
        [0, 0, 0, 0, 0, DigitBits::from_digit((n % 10) as u8) as u8]
    }
}

/// Format a [`i8`] into its 7-segment byte representation as a `[u8; 2]` aligning the bytes to the left.
///
/// # Example
///
/// ```
/// # use tm1637_embedded_hal::{numbers::i8_2, mappings::{DigitBits, SpecialCharBits}};
/// let formatted = i8_2(1);
/// assert_eq!(formatted, [DigitBits::One as u8, 0]);
///
/// let formatted = i8_2(-1);
/// assert_eq!(formatted, [SpecialCharBits::Minus as u8, DigitBits::One as u8]);
/// ```
pub fn i8_2(n: i8) -> [u8; 2] {
    let n = n.clamp(-9, 99);

    if n > 0 {
        u8_2(n.unsigned_abs())
    } else {
        [
            SpecialCharBits::Minus as u8,
            DigitBits::from_digit((-n) as u8) as u8,
        ]
    }
}

/// Format a [`i16`] into its 7-segment byte representation as a `[u8; 3]` aligning the bytes to the right.
///
/// # Example
///
/// ```
/// # use tm1637_embedded_hal::{numbers::i16_3, mappings::{DigitBits, SpecialCharBits}};
/// let formatted = i16_3(1);
/// assert_eq!(formatted, [DigitBits::One as u8, 0, 0]);
///
/// let formatted = i16_3(-1);
/// assert_eq!(formatted, [SpecialCharBits::Minus as u8, DigitBits::One as u8, 0]);
///
/// let formatted = i16_3(-12);
/// assert_eq!(formatted, [SpecialCharBits::Minus as u8, DigitBits::from_digit(1) as u8, DigitBits::from_digit(2) as u8]);
/// ```
pub fn i16_3(n: i16) -> [u8; 3] {
    let n = n.clamp(-99, 999);

    if n > 0 {
        u16_3(n.unsigned_abs())
    } else if n < -9 {
        [
            SpecialCharBits::Minus as u8,
            DigitBits::from_digit((-n / 10) as u8) as u8,
            DigitBits::from_digit((-n % 10) as u8) as u8,
        ]
    } else {
        [
            SpecialCharBits::Minus as u8,
            DigitBits::from_digit((-n % 10) as u8) as u8,
            0,
        ]
    }
}

/// Format a [`i16`] into its 7-segment byte representation as a `[u8; 3]` aligning the bytes to the right.
///
/// # Example
///
/// ```
/// # use tm1637_embedded_hal::{numbers::r_i16_3, mappings::{DigitBits, SpecialCharBits}};
/// let formatted = r_i16_3(1);
/// assert_eq!(formatted, [0, 0, DigitBits::One as u8]);
///
/// let formatted = r_i16_3(-1);
/// assert_eq!(formatted, [0, SpecialCharBits::Minus as u8, DigitBits::One as u8]);
///
/// let formatted = r_i16_3(-12);
/// assert_eq!(formatted, [SpecialCharBits::Minus as u8, DigitBits::from_digit(1) as u8, DigitBits::from_digit(2) as u8]);
/// ```
pub fn r_i16_3(n: i16) -> [u8; 3] {
    let n = n.clamp(-99, 999);

    if n > 0 {
        r_u16_3(n.unsigned_abs())
    } else if n < -9 {
        [
            SpecialCharBits::Minus as u8,
            DigitBits::from_digit((-n / 10) as u8) as u8,
            DigitBits::from_digit((-n % 10) as u8) as u8,
        ]
    } else {
        [
            0,
            SpecialCharBits::Minus as u8,
            DigitBits::from_digit((-n % 10) as u8) as u8,
        ]
    }
}

/// Format a [`i16`] into its 7-segment byte representation as a `[u8; 4]` aligning the bytes to the left.
///
/// # Example
///
/// ```
/// # use tm1637_embedded_hal::{numbers::i16_4, mappings::{DigitBits, SpecialCharBits}};
/// let formatted = i16_4(1);
/// assert_eq!(formatted, [DigitBits::One as u8, 0, 0, 0]);
///
/// let formatted = i16_4(-1);
/// assert_eq!(formatted, [SpecialCharBits::Minus as u8, DigitBits::One as u8, 0, 0]);
///
/// let formatted = i16_4(-123);
/// assert_eq!(formatted, [SpecialCharBits::Minus as u8, DigitBits::from_digit(1) as u8, DigitBits::from_digit(2) as u8, DigitBits::from_digit(3) as u8]);
/// ```
pub fn i16_4(n: i16) -> [u8; 4] {
    let n = n.clamp(-999, 9999);

    if n > 0 {
        u16_4(n.unsigned_abs())
    } else if n < -99 {
        [
            SpecialCharBits::Minus as u8,
            DigitBits::from_digit((-n / 100) as u8) as u8,
            DigitBits::from_digit(((-n / 10) % 10) as u8) as u8,
            DigitBits::from_digit((-n % 10) as u8) as u8,
        ]
    } else if n < -9 {
        [
            SpecialCharBits::Minus as u8,
            DigitBits::from_digit((-n / 10) as u8) as u8,
            DigitBits::from_digit((-n % 10) as u8) as u8,
            0,
        ]
    } else {
        [
            SpecialCharBits::Minus as u8,
            DigitBits::from_digit((-n % 10) as u8) as u8,
            0,
            0,
        ]
    }
}

/// Format a [`i16`] into its 7-segment byte representation as a `[u8; 4]` aligning the bytes to the right.
///
/// # Example
///
/// ```
/// # use tm1637_embedded_hal::{numbers::r_i16_4, mappings::{DigitBits, SpecialCharBits}};
/// let formatted = r_i16_4(1);
/// assert_eq!(formatted, [0, 0, 0, DigitBits::One as u8]);
///
/// let formatted = r_i16_4(-1);
/// assert_eq!(formatted, [0, 0, SpecialCharBits::Minus as u8, DigitBits::One as u8]);
///
/// let formatted = r_i16_4(-123);
/// assert_eq!(formatted, [SpecialCharBits::Minus as u8, DigitBits::from_digit(1) as u8, DigitBits::from_digit(2) as u8, DigitBits::from_digit(3) as u8]);
/// ```
pub fn r_i16_4(n: i16) -> [u8; 4] {
    let n = n.clamp(-999, 9999);

    if n > 0 {
        r_u16_4(n.unsigned_abs())
    } else if n < -99 {
        [
            SpecialCharBits::Minus as u8,
            DigitBits::from_digit((-n / 100) as u8) as u8,
            DigitBits::from_digit(((-n / 10) % 10) as u8) as u8,
            DigitBits::from_digit((-n % 10) as u8) as u8,
        ]
    } else if n < -9 {
        [
            0,
            SpecialCharBits::Minus as u8,
            DigitBits::from_digit((-n / 10) as u8) as u8,
            DigitBits::from_digit((-n % 10) as u8) as u8,
        ]
    } else {
        [
            0,
            0,
            SpecialCharBits::Minus as u8,
            DigitBits::from_digit((-n % 10) as u8) as u8,
        ]
    }
}

/// Format a [`i32`] into its 7-segment byte representation as a `[u8; 5]` aligning the bytes to the left.
///
/// # Example
///
/// ```
/// # use tm1637_embedded_hal::{numbers::i32_5, mappings::{DigitBits, SpecialCharBits}};
/// let formatted = i32_5(1);
/// assert_eq!(formatted, [DigitBits::One as u8, 0, 0, 0, 0]);
///
/// let formatted = i32_5(-1);
/// assert_eq!(formatted, [SpecialCharBits::Minus as u8, DigitBits::One as u8, 0, 0, 0]);
///
/// let formatted = i32_5(-1234);
/// assert_eq!(formatted, [SpecialCharBits::Minus as u8, DigitBits::from_digit(1) as u8, DigitBits::from_digit(2) as u8, DigitBits::from_digit(3) as u8, DigitBits::from_digit(4) as u8]);
/// ```
pub fn i32_5(n: i32) -> [u8; 5] {
    let n = n.clamp(-9999, 99999);

    if n > 0 {
        u32_5(n.unsigned_abs())
    } else if n < -999 {
        [
            SpecialCharBits::Minus as u8,
            DigitBits::from_digit((-n / 1000) as u8) as u8,
            DigitBits::from_digit(((-n / 100) % 10) as u8) as u8,
            DigitBits::from_digit(((-n / 10) % 10) as u8) as u8,
            DigitBits::from_digit((-n % 10) as u8) as u8,
        ]
    } else if n < -99 {
        [
            SpecialCharBits::Minus as u8,
            DigitBits::from_digit((-n / 100) as u8) as u8,
            DigitBits::from_digit(((-n / 10) % 10) as u8) as u8,
            DigitBits::from_digit((-n % 10) as u8) as u8,
            0,
        ]
    } else if n < -9 {
        [
            SpecialCharBits::Minus as u8,
            DigitBits::from_digit((-n / 10) as u8) as u8,
            DigitBits::from_digit((-n % 10) as u8) as u8,
            0,
            0,
        ]
    } else {
        [
            SpecialCharBits::Minus as u8,
            DigitBits::from_digit((-n % 10) as u8) as u8,
            0,
            0,
            0,
        ]
    }
}

/// Format a [`i32`] into its 7-segment byte representation as a `[u8; 5]` aligning the bytes to the right.
///
/// # Example
///
/// ```
/// # use tm1637_embedded_hal::{numbers::r_i32_5, mappings::{DigitBits, SpecialCharBits}};
/// let formatted = r_i32_5(1);
/// assert_eq!(formatted, [0, 0, 0, 0, DigitBits::One as u8]);
///
/// let formatted = r_i32_5(-1);
/// assert_eq!(formatted, [0, 0, 0, SpecialCharBits::Minus as u8, DigitBits::One as u8]);
///
/// let formatted = r_i32_5(-1234);
/// assert_eq!(formatted, [SpecialCharBits::Minus as u8, DigitBits::from_digit(1) as u8, DigitBits::from_digit(2) as u8, DigitBits::from_digit(3) as u8, DigitBits::from_digit(4) as u8]);
/// ```
pub fn r_i32_5(n: i32) -> [u8; 5] {
    let n = n.clamp(-9999, 99999);

    if n > 0 {
        r_u32_5(n.unsigned_abs())
    } else if n < -999 {
        [
            SpecialCharBits::Minus as u8,
            DigitBits::from_digit((-n / 1000) as u8) as u8,
            DigitBits::from_digit(((-n / 100) % 10) as u8) as u8,
            DigitBits::from_digit(((-n / 10) % 10) as u8) as u8,
            DigitBits::from_digit((-n % 10) as u8) as u8,
        ]
    } else if n < -99 {
        [
            0,
            SpecialCharBits::Minus as u8,
            DigitBits::from_digit((-n / 100) as u8) as u8,
            DigitBits::from_digit(((-n / 10) % 10) as u8) as u8,
            DigitBits::from_digit((-n % 10) as u8) as u8,
        ]
    } else if n < -9 {
        [
            0,
            0,
            SpecialCharBits::Minus as u8,
            DigitBits::from_digit((-n / 10) as u8) as u8,
            DigitBits::from_digit((-n % 10) as u8) as u8,
        ]
    } else {
        [
            0,
            0,
            0,
            SpecialCharBits::Minus as u8,
            DigitBits::from_digit((-n % 10) as u8) as u8,
        ]
    }
}

/// Format a [`i32`] into its 7-segment byte representation as a `[u8; 6]` aligning the bytes to the left.
///
/// # Example
///
/// ```
/// # use tm1637_embedded_hal::{numbers::i32_6, mappings::{DigitBits, SpecialCharBits}};
/// let formatted = i32_6(1);
/// assert_eq!(formatted, [DigitBits::One as u8, 0, 0, 0, 0, 0]);
///
/// let formatted = i32_6(-1);
/// assert_eq!(formatted, [SpecialCharBits::Minus as u8, DigitBits::One as u8, 0, 0, 0, 0]);
///
/// let formatted = i32_6(-12345);
/// assert_eq!(formatted, [SpecialCharBits::Minus as u8, DigitBits::from_digit(1) as u8, DigitBits::from_digit(2) as u8, DigitBits::from_digit(3) as u8, DigitBits::from_digit(4) as u8, DigitBits::from_digit(5) as u8]);
/// ```
pub fn i32_6(n: i32) -> [u8; 6] {
    let n = n.clamp(-99999, 999999);

    if n > 0 {
        u32_6(n.unsigned_abs())
    } else if n < -9999 {
        [
            SpecialCharBits::Minus as u8,
            DigitBits::from_digit((-n / 10000) as u8) as u8,
            DigitBits::from_digit(((-n / 1000) % 10) as u8) as u8,
            DigitBits::from_digit(((-n / 100) % 10) as u8) as u8,
            DigitBits::from_digit(((-n / 10) % 10) as u8) as u8,
            DigitBits::from_digit((-n % 10) as u8) as u8,
        ]
    } else if n < -999 {
        [
            SpecialCharBits::Minus as u8,
            DigitBits::from_digit((-n / 1000) as u8) as u8,
            DigitBits::from_digit(((-n / 100) % 10) as u8) as u8,
            DigitBits::from_digit(((-n / 10) % 10) as u8) as u8,
            DigitBits::from_digit((-n % 10) as u8) as u8,
            0,
        ]
    } else if n < -99 {
        [
            SpecialCharBits::Minus as u8,
            DigitBits::from_digit((-n / 100) as u8) as u8,
            DigitBits::from_digit(((-n / 10) % 10) as u8) as u8,
            DigitBits::from_digit((-n % 10) as u8) as u8,
            0,
            0,
        ]
    } else if n < -9 {
        [
            SpecialCharBits::Minus as u8,
            DigitBits::from_digit((-n / 10) as u8) as u8,
            DigitBits::from_digit((-n % 10) as u8) as u8,
            0,
            0,
            0,
        ]
    } else {
        [
            SpecialCharBits::Minus as u8,
            DigitBits::from_digit((-n % 10) as u8) as u8,
            0,
            0,
            0,
            0,
        ]
    }
}

/// Format a [`i32`] into its 7-segment byte representation as a `[u8; 6]` aligning the bytes to the right.
///
/// # Example
///
/// ```
/// # use tm1637_embedded_hal::{numbers::r_i32_6, mappings::{DigitBits, SpecialCharBits}};
/// let formatted = r_i32_6(1);
/// assert_eq!(formatted, [0, 0, 0, 0, 0, DigitBits::One as u8]);
///
/// let formatted = r_i32_6(-1);
/// assert_eq!(formatted, [0, 0, 0, 0, SpecialCharBits::Minus as u8, DigitBits::One as u8]);
///
/// let formatted = r_i32_6(-12345);
/// assert_eq!(formatted, [SpecialCharBits::Minus as u8, DigitBits::from_digit(1) as u8, DigitBits::from_digit(2) as u8, DigitBits::from_digit(3) as u8, DigitBits::from_digit(4) as u8, DigitBits::from_digit(5) as u8]);
/// ```
pub fn r_i32_6(n: i32) -> [u8; 6] {
    let n = n.clamp(-99999, 999999);

    if n > 0 {
        r_u32_6(n.unsigned_abs())
    } else if n < -9999 {
        [
            SpecialCharBits::Minus as u8,
            DigitBits::from_digit((-n / 10000) as u8) as u8,
            DigitBits::from_digit(((-n / 1000) % 10) as u8) as u8,
            DigitBits::from_digit(((-n / 100) % 10) as u8) as u8,
            DigitBits::from_digit(((-n / 10) % 10) as u8) as u8,
            DigitBits::from_digit((-n % 10) as u8) as u8,
        ]
    } else if n < -999 {
        [
            0,
            SpecialCharBits::Minus as u8,
            DigitBits::from_digit((-n / 1000) as u8) as u8,
            DigitBits::from_digit(((-n / 100) % 10) as u8) as u8,
            DigitBits::from_digit(((-n / 10) % 10) as u8) as u8,
            DigitBits::from_digit((-n % 10) as u8) as u8,
        ]
    } else if n < -99 {
        [
            0,
            0,
            SpecialCharBits::Minus as u8,
            DigitBits::from_digit((-n / 100) as u8) as u8,
            DigitBits::from_digit(((-n / 10) % 10) as u8) as u8,
            DigitBits::from_digit((-n % 10) as u8) as u8,
        ]
    } else if n < -9 {
        [
            0,
            0,
            0,
            SpecialCharBits::Minus as u8,
            DigitBits::from_digit((-n / 10) as u8) as u8,
            DigitBits::from_digit((-n % 10) as u8) as u8,
        ]
    } else {
        [
            0,
            0,
            0,
            0,
            SpecialCharBits::Minus as u8,
            DigitBits::from_digit((-n % 10) as u8) as u8,
        ]
    }
}
