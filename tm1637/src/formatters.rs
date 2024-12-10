//! Format numbers into byte arrays.
//!
//! Functions for converting numbers ([`i8`]s, [`i16`]s, [`i32`]s or [`f32`]s) into arrays of bytes
//! that can be sent to a TM1637 display.
//!
//! There are versions of these functions that are meant for 4-digit displays
//! and for 6-digit displays. The 6-digit versions take into account that the
//! order of the bytes does not directly correlate with the order of the physical
//! digits.
//!
//! All numbers are aligned to the right.
//!
//! # Example
//!
//! ```rust
//! use tm1637_embedded_hal::{formatters::i16_to_4digits, mock::Noop, TM1637Builder};
//!
//! let mut tm = TM1637Builder::new(Noop, Noop, Noop).build_blocking::<4>();
//!
//! tm.init().ok();
//!
//! tm.display_slice(0, &i16_to_4digits(1234));
//! ```

use crate::mappings::{DigitBits, UpsideDownDigitBits};

/// Formats a [`i16`] clamped between `-999` and `9999`, for a `4-digit display`.
///
/// # Example
///
/// A counter that goes from `-100` to `100`:
///
/// ```rust
/// use tm1637_embedded_hal::{formatters::i16_to_4digits, mock::Noop, TM1637Builder};
/// use embedded_hal::delay::DelayNs;
///
/// let mut delay = Noop;
/// let mut tm = TM1637Builder::new(Noop, Noop, Noop).build_blocking::<4>();
///
/// tm.init().ok();
///
/// for i in -100..100 {
///     let segs = i16_to_4digits(i);
///     tm.display_slice(0, &segs).ok();
///
///     delay.delay_ms(100);
/// }
/// ```
pub fn i16_to_4digits(n: i16) -> [u8; 4] {
    let mut bytes: [u8; 4] = [0; 4];
    let mut m: i16 = n.clamp(-999, 9999).abs();

    for position in (0..4).rev() {
        bytes[position as usize] = DigitBits::from_digit((m % 10) as u8) as u8;

        m /= 10;

        if m == 0 {
            if n < 0 {
                bytes[position as usize - 1] = 0b01000000; // minus sign
            }
            break;
        };
    }

    bytes
}

/// Formats a [`i32`] clamped between `-99999` and `999999`, for a `6-digit display`.
pub fn i32_to_6digits(n: i32) -> [u8; 6] {
    let mut b: [u8; 6] = [0; 6];
    let mut m: i32 = n.clamp(-99999, 999999).abs();

    for position in (0..6).rev() {
        b[position as usize] = DigitBits::from_digit((m % 10) as u8) as u8;

        m /= 10;

        if m == 0 {
            if !n.is_positive() {
                b[position as usize - 1] = 0b01000000; // minus sign
            }
            break;
        };
    }

    // Swizzle bytes around to fit the order of 6-digit displays
    [b[2], b[1], b[0], b[5], b[4], b[3]]
}

/// Formats a [`i8`] clamped between `-9` and `99`, appending the degrees symbol `(°)`
/// and an `uppercase C`, for a `4-digit display`.
pub fn celsius_to_4digits(n: i8) -> [u8; 4] {
    let mut m: i8 = n.clamp(-9, 99);

    // 3rd and 4th bytes are the degrees symbol (°) and uppercase C
    let mut b: [u8; 4] = [0, 0, 0x63, 0x39];

    for position in (0..2).rev() {
        b[position as usize] = DigitBits::from_digit((m.abs() % 10) as u8) as u8;

        m /= 10;

        if m == 0 {
            if !n.is_positive() {
                b[position as usize - 1] = 0b01000000; // minus sign
            }
            break;
        };
    }
    b
}

/// Formats a [`i16`] clamped between `-99` and `999`, appending the degrees symbol `(°)`,
/// for a `4-digit display`.
pub fn degrees_to_4digits(n: i16) -> [u8; 4] {
    let mut m: i16 = n.clamp(-99, 999);

    // 4th byte is the degrees symbol (°)
    let mut b: [u8; 4] = [0, 0, 0, 0x63];

    for position in (0..3).rev() {
        b[position as usize] = DigitBits::from_digit((m.abs() % 10) as u8) as u8;

        m /= 10;

        if m == 0 {
            if !n.is_positive() {
                b[position as usize - 1] = 0b01000000; // minus sign
            }
            break;
        };
    }
    b
}

/// Formats two [`u8`]s between `0` and `99`, with an optional colon between them.
///
/// This will only work for `4-digit displays` where there's a physical colon,
/// and that colon acts as the decimal dot between the 2nd and 3rd digit.
///
/// # Example
///
/// Let's create a clock displaying `12:34` with a blinking colon:
///
/// ```rust
/// use tm1637_embedded_hal::{formatters::clock_to_4digits, mock::Noop, TM1637Builder};
/// use embedded_hal::delay::DelayNs;
///
/// let mut delay = Noop;
///
/// let mut tm = TM1637Builder::new(Noop, Noop, Noop).build_blocking::<4>();
///
/// tm.init().ok();
///
/// for hour in 12..24 {
///     for minute in 34..60 {
///         for second in 0..120 {
///             let blink = second % 2 == 0;
///             let segs = clock_to_4digits(hour, minute, blink);
///
///             tm.display_slice(0, &segs).ok();
///
///             delay.delay_ms(500);
///         }
///     }
/// }
/// ```
pub fn clock_to_4digits(hour: u8, minute: u8, colon: bool) -> [u8; 4] {
    let mut b: [u8; 4] = [0, 0, 0, 0];

    if hour >= 10 {
        b[0] = DigitBits::from_digit(hour / 10) as u8;
    }
    b[1] = DigitBits::from_digit(hour % 10) as u8;

    if colon {
        b[1] |= 0b1000_0000
    }
    b[2] = DigitBits::from_digit(minute / 10) as u8;
    b[3] = DigitBits::from_digit(minute % 10) as u8;

    b
}

/// Formats a [`i16`] clamped between `-999` and `9999`, for an `upside-down 4-digit display`.
pub fn i16_to_upsidedown_4digits(n: i16) -> [u8; 4] {
    let mut bytes: [u8; 4] = [0; 4];
    let mut m: i16 = n.clamp(-999, 9999).abs();

    for position in 0..4 {
        bytes[position as usize] = UpsideDownDigitBits::from_digit((m % 10) as u8) as u8;

        m /= 10;

        if m == 0 {
            if !n.is_positive() {
                bytes[position as usize + 1] = 0b01000000; // minus sign
            }
            break;
        };
    }

    bytes
}

/// Formats a [`f32`] with the given amount of decimal digits, for a `6-digit display`.
pub fn f32_to_6digits(n: f32, decimals: u8) -> [u8; 6] {
    use core::ops::Mul;

    let mut b: [u8; 6] = [0; 6];
    let decimal_position = 5 - decimals;

    let mut m: i32 = ((n.mul(10i32.pow(decimals as u32) as f32)
        + if n.is_sign_positive() {
            0.5_f32
        } else {
            -0.5_f32
        }) as i32)
        .clamp(-99999, 999999)
        .abs();

    for position in (0..6).rev() {
        b[position as usize] = DigitBits::from_digit((m % 10) as u8) as u8;

        m /= 10;

        if position == decimal_position {
            // Add a dot here
            b[position as usize] |= 0b1000_0000;
        }
        if m == 0 && position <= decimal_position {
            // Add the minus sign only when the digit with the decimal point
            // has been done; do not break earlier.
            if !n.is_sign_positive() {
                b[position as usize - 1] = 0b01000000; // minus sign
            }
            break;
        };
    }

    // Swizzle bytes around to fit the order of 6-digit displays
    [b[2], b[1], b[0], b[5], b[4], b[3]]
}
