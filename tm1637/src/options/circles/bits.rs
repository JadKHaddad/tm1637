use crate::mappings::SegmentBits;

/// Rotating circle bits.
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RotatingCircleBits {
    /// ```text
    ///   ---
    ///  |   |
    ///  |
    ///   ---
    ///```
    A = SegmentBits::SegB as u8
        | SegmentBits::SegA as u8
        | SegmentBits::SegF as u8
        | SegmentBits::SegE as u8
        | SegmentBits::SegD as u8,
    /// ```text
    ///   ---
    ///  |
    ///  |   |
    ///   ---
    ///```
    B = SegmentBits::SegA as u8
        | SegmentBits::SegF as u8
        | SegmentBits::SegE as u8
        | SegmentBits::SegD as u8
        | SegmentBits::SegC as u8,
    /// ```text
    ///   
    ///  |   |
    ///  |   |
    ///   ---
    ///```
    C = SegmentBits::SegF as u8
        | SegmentBits::SegE as u8
        | SegmentBits::SegD as u8
        | SegmentBits::SegC as u8
        | SegmentBits::SegB as u8,
    /// ```text
    ///   ---
    ///      |
    ///  |   |
    ///   ---
    ///```
    D = SegmentBits::SegE as u8
        | SegmentBits::SegD as u8
        | SegmentBits::SegC as u8
        | SegmentBits::SegB as u8
        | SegmentBits::SegA as u8,
    /// ```text
    ///   ---
    ///  |   |
    ///      |
    ///   ---
    ///```
    E = SegmentBits::SegD as u8
        | SegmentBits::SegC as u8
        | SegmentBits::SegB as u8
        | SegmentBits::SegA as u8
        | SegmentBits::SegF as u8,
    /// ```text
    ///   ---
    ///  |   |
    ///  |   |
    ///   
    ///```
    F = SegmentBits::SegC as u8
        | SegmentBits::SegB as u8
        | SegmentBits::SegA as u8
        | SegmentBits::SegF as u8
        | SegmentBits::SegE as u8,
}

impl RotatingCircleBits {
    /// Returns all rotating circle bits.
    pub const fn all() -> [RotatingCircleBits; 6] {
        [
            RotatingCircleBits::A,
            RotatingCircleBits::B,
            RotatingCircleBits::C,
            RotatingCircleBits::D,
            RotatingCircleBits::E,
            RotatingCircleBits::F,
        ]
    }

    /// Resturns all rotating circle bits reversed.
    pub const fn all_reversed() -> [RotatingCircleBits; 6] {
        [
            RotatingCircleBits::F,
            RotatingCircleBits::E,
            RotatingCircleBits::D,
            RotatingCircleBits::C,
            RotatingCircleBits::B,
            RotatingCircleBits::A,
        ]
    }

    /// Returns all rotating circle bits as [`u8`].
    pub const fn all_u8() -> [u8; 6] {
        [
            RotatingCircleBits::A as u8,
            RotatingCircleBits::B as u8,
            RotatingCircleBits::C as u8,
            RotatingCircleBits::D as u8,
            RotatingCircleBits::E as u8,
            RotatingCircleBits::F as u8,
        ]
    }

    /// Resturns all rotating circle bits reversed as [`u8`].
    pub const fn all_u8_reversed() -> [u8; 6] {
        [
            RotatingCircleBits::F as u8,
            RotatingCircleBits::E as u8,
            RotatingCircleBits::D as u8,
            RotatingCircleBits::C as u8,
            RotatingCircleBits::B as u8,
            RotatingCircleBits::A as u8,
        ]
    }
}
