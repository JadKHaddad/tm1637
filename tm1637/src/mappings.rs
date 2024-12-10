//! Mappings for 7-segment display characters.
//!
//! ```text
//!      A
//!     ---
//! F  |   |  B
//!     -G-
//! E  |   |  C
//!     ---
//!      D
//! ```

/// Maps the segment from the device to its bit.
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SegmentBits {
    /// A segment
    SegA = 0b00000001,
    /// B segment
    SegB = 0b00000010,
    /// C segment
    SegC = 0b00000100,
    /// D segment
    SegD = 0b00001000,
    /// E segment
    SegE = 0b00010000,
    /// F segment
    SegF = 0b00100000,
    /// G segment
    SegG = 0b01000000,
    /// Double point or dot
    ///
    /// # Usage
    ///
    /// - `Or` this bit with the other bits to display the dot.
    /// - `Or` this bit with the bit responsible for displaying the double point. Often second position on a 4-digit display.
    Dot = 0b10000000,
    // TODO: maybe rename this to Colon
    // We still have to figure out how to handle dps
}

impl SegmentBits {
    /// Returns all segments.
    pub const fn all() -> [SegmentBits; 8] {
        [
            SegmentBits::SegA,
            SegmentBits::SegB,
            SegmentBits::SegC,
            SegmentBits::SegD,
            SegmentBits::SegE,
            SegmentBits::SegF,
            SegmentBits::SegG,
            SegmentBits::Dot,
        ]
    }

    /// Returns all segments as u8.
    pub const fn all_u8() -> [u8; 8] {
        [
            SegmentBits::SegA as u8,
            SegmentBits::SegB as u8,
            SegmentBits::SegC as u8,
            SegmentBits::SegD as u8,
            SegmentBits::SegE as u8,
            SegmentBits::SegF as u8,
            SegmentBits::SegG as u8,
            SegmentBits::Dot as u8,
        ]
    }
}

/// Maps a digit to its closest possible representation on a 7-segment display.
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DigitBits {
    /// 0
    Zero = 0b00111111,
    /// 1
    One = 0b00000110,
    /// 2
    Two = 0b01011011,
    /// 3
    Three = 0b01001111,
    /// 4
    Four = 0b01100110,
    /// 5
    Five = 0b01101101,
    /// 6
    Six = 0b01111101,
    /// 7
    Seven = 0b00000111,
    /// 8
    Eight = 0b01111111,
    /// 9
    Nine = 0b01101111,
}

impl DigitBits {
    /// Returns all digits.
    pub const fn all() -> [DigitBits; 10] {
        [
            DigitBits::Zero,
            DigitBits::One,
            DigitBits::Two,
            DigitBits::Three,
            DigitBits::Four,
            DigitBits::Five,
            DigitBits::Six,
            DigitBits::Seven,
            DigitBits::Eight,
            DigitBits::Nine,
        ]
    }

    /// Returns all digits as [`u8`].
    pub const fn all_u8() -> [u8; 10] {
        [
            DigitBits::Zero as u8,
            DigitBits::One as u8,
            DigitBits::Two as u8,
            DigitBits::Three as u8,
            DigitBits::Four as u8,
            DigitBits::Five as u8,
            DigitBits::Six as u8,
            DigitBits::Seven as u8,
            DigitBits::Eight as u8,
            DigitBits::Nine as u8,
        ]
    }

    /// Creates a new [`DigitBits`] from a [`u8`] digit.
    pub fn from_digit(digit: u8) -> Self {
        match digit {
            0 => DigitBits::Zero,
            1 => DigitBits::One,
            2 => DigitBits::Two,
            3 => DigitBits::Three,
            4 => DigitBits::Four,
            5 => DigitBits::Five,
            6 => DigitBits::Six,
            7 => DigitBits::Seven,
            8 => DigitBits::Eight,
            9 => DigitBits::Nine,
            _ => DigitBits::Zero,
        }
    }
}

/// Maps a hex digit to its closest possible representation on a 7-segment display.
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HexDigitBits {
    /// 0
    Zero = DigitBits::Zero as u8,
    /// 1
    One = DigitBits::One as u8,
    /// 2
    Two = DigitBits::Two as u8,
    /// 3
    Three = DigitBits::Three as u8,
    /// 4
    Four = DigitBits::Four as u8,
    /// 5
    Five = DigitBits::Five as u8,
    /// 6
    Six = DigitBits::Six as u8,
    /// 7
    Seven = DigitBits::Seven as u8,
    /// 8
    Eight = DigitBits::Eight as u8,
    /// 9
    Nine = DigitBits::Nine as u8,
    /// A
    A = UpCharBits::UpA as u8,
    /// b
    B = LoCharBits::LoB as u8,
    /// C
    C = UpCharBits::UpC as u8,
    /// d
    D = LoCharBits::LoD as u8,
    /// E
    E = UpCharBits::UpE as u8,
    /// F
    F = UpCharBits::UpF as u8,
}

impl HexDigitBits {
    /// Returns all digits.
    pub const fn all() -> [HexDigitBits; 16] {
        [
            HexDigitBits::Zero,
            HexDigitBits::One,
            HexDigitBits::Two,
            HexDigitBits::Three,
            HexDigitBits::Four,
            HexDigitBits::Five,
            HexDigitBits::Six,
            HexDigitBits::Seven,
            HexDigitBits::Eight,
            HexDigitBits::Nine,
            HexDigitBits::A,
            HexDigitBits::B,
            HexDigitBits::C,
            HexDigitBits::D,
            HexDigitBits::E,
            HexDigitBits::F,
        ]
    }

    /// Returns all digits as [`u8`].
    pub const fn all_u8() -> [u8; 16] {
        [
            HexDigitBits::Zero as u8,
            HexDigitBits::One as u8,
            HexDigitBits::Two as u8,
            HexDigitBits::Three as u8,
            HexDigitBits::Four as u8,
            HexDigitBits::Five as u8,
            HexDigitBits::Six as u8,
            HexDigitBits::Seven as u8,
            HexDigitBits::Eight as u8,
            HexDigitBits::Nine as u8,
            HexDigitBits::A as u8,
            HexDigitBits::B as u8,
            HexDigitBits::C as u8,
            HexDigitBits::D as u8,
            HexDigitBits::E as u8,
            HexDigitBits::F as u8,
        ]
    }

    /// Creates a new [`HexDigitBits`] from a [`u8`] digit.
    pub fn from_digit(digit: u8) -> Self {
        match digit {
            0 => HexDigitBits::Zero,
            1 => HexDigitBits::One,
            2 => HexDigitBits::Two,
            3 => HexDigitBits::Three,
            4 => HexDigitBits::Four,
            5 => HexDigitBits::Five,
            6 => HexDigitBits::Six,
            7 => HexDigitBits::Seven,
            8 => HexDigitBits::Eight,
            9 => HexDigitBits::Nine,
            10 => HexDigitBits::A,
            11 => HexDigitBits::B,
            12 => HexDigitBits::C,
            13 => HexDigitBits::D,
            14 => HexDigitBits::E,
            15 => HexDigitBits::F,
            _ => HexDigitBits::Zero,
        }
    }
}

/// Maps a upside-down digit to its closest possible representation on a 7-segment display.
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UpsideDownDigitBits {
    /// Upside-down 0
    Zero = 0b00111111,
    /// Upside-down 1
    One = 0b00110000,
    /// Upside-down 2
    Two = 0b01011011,
    /// Upside-down 3
    Three = 0b01111001,
    /// Upside-down 4
    Four = 0b01110100,
    /// Upside-down 5
    Five = 0b01101101,
    /// Upside-down 6
    Six = 0b01101111,
    /// Upside-down 7
    Seven = 0b00111000,
    /// Upside-down 8
    Eight = 0b01111111,
    /// Upside-down 9
    Nine = 0b01111101,
}
impl UpsideDownDigitBits {
    /// Returns all digits.
    pub const fn all() -> [UpsideDownDigitBits; 10] {
        [
            UpsideDownDigitBits::Zero,
            UpsideDownDigitBits::One,
            UpsideDownDigitBits::Two,
            UpsideDownDigitBits::Three,
            UpsideDownDigitBits::Four,
            UpsideDownDigitBits::Five,
            UpsideDownDigitBits::Six,
            UpsideDownDigitBits::Seven,
            UpsideDownDigitBits::Eight,
            UpsideDownDigitBits::Nine,
        ]
    }

    /// Returns all digits as [`u8`].
    pub const fn all_u8() -> [u8; 10] {
        [
            UpsideDownDigitBits::Zero as u8,
            UpsideDownDigitBits::One as u8,
            UpsideDownDigitBits::Two as u8,
            UpsideDownDigitBits::Three as u8,
            UpsideDownDigitBits::Four as u8,
            UpsideDownDigitBits::Five as u8,
            UpsideDownDigitBits::Six as u8,
            UpsideDownDigitBits::Seven as u8,
            UpsideDownDigitBits::Eight as u8,
            UpsideDownDigitBits::Nine as u8,
        ]
    }

    /// Creates a new [`DigitBits`] from a [`u8`] digit.
    pub fn from_digit(digit: u8) -> Self {
        match digit {
            0 => UpsideDownDigitBits::Zero,
            1 => UpsideDownDigitBits::One,
            2 => UpsideDownDigitBits::Two,
            3 => UpsideDownDigitBits::Three,
            4 => UpsideDownDigitBits::Four,
            5 => UpsideDownDigitBits::Five,
            6 => UpsideDownDigitBits::Six,
            7 => UpsideDownDigitBits::Seven,
            8 => UpsideDownDigitBits::Eight,
            9 => UpsideDownDigitBits::Nine,
            _ => UpsideDownDigitBits::Zero,
        }
    }
}

/// Maps a character to its closest possible representation on a 7-segment display.
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UpCharBits {
    /// Uppercase A
    UpA = 0x77,
    /// Uppercase B
    UpB = 0x7F,
    /// Uppercase C
    UpC = 0x39,
    /// Uppercase E
    UpE = 0x79,
    // can be also done like this (OR'ing segment bits) :
    /// Uppercase F
    UpF = SegmentBits::SegA as u8
        | SegmentBits::SegF as u8
        | SegmentBits::SegE as u8
        | SegmentBits::SegG as u8,
    /// Uppercase G
    UpG = 0x3D,
    /// Uppercase H
    UpH = 0x76,
    /// Uppercase I
    UpI = 0x30,
    /// Uppercase J
    UpJ = 0x1E,
    /// Uppercase L
    UpL = 0x38,
    /// Uppercase O
    UpO = 0x3F,
    /// Uppercase P
    UpP = 0x73,
    /// Uppercase S
    UpS = 0x6D,
    /// Uppercase U
    UpU = 0x3E,
    /// Uppercase Z
    UpZ = 0x5B,
}

impl UpCharBits {
    /// Returns all uppercase characters.
    pub const fn all() -> [UpCharBits; 15] {
        [
            UpCharBits::UpA,
            UpCharBits::UpB,
            UpCharBits::UpC,
            UpCharBits::UpE,
            UpCharBits::UpF,
            UpCharBits::UpG,
            UpCharBits::UpH,
            UpCharBits::UpI,
            UpCharBits::UpJ,
            UpCharBits::UpL,
            UpCharBits::UpO,
            UpCharBits::UpP,
            UpCharBits::UpS,
            UpCharBits::UpU,
            UpCharBits::UpZ,
        ]
    }

    /// Returns all uppercase characters as [`u8`].
    pub const fn all_u8() -> [u8; 15] {
        [
            UpCharBits::UpA as u8,
            UpCharBits::UpB as u8,
            UpCharBits::UpC as u8,
            UpCharBits::UpE as u8,
            UpCharBits::UpF as u8,
            UpCharBits::UpG as u8,
            UpCharBits::UpH as u8,
            UpCharBits::UpI as u8,
            UpCharBits::UpJ as u8,
            UpCharBits::UpL as u8,
            UpCharBits::UpO as u8,
            UpCharBits::UpP as u8,
            UpCharBits::UpS as u8,
            UpCharBits::UpU as u8,
            UpCharBits::UpZ as u8,
        ]
    }
}

/// Maps a character to its closest possible representation on a 7-segment display.
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LoCharBits {
    /// Lowercase A
    LoA = 0x5F,
    /// Lowercase B
    LoB = 0x7C,
    /// Lowercase C
    LoC = 0x58,
    /// Lowercase D
    LoD = 0x5E,
    /// Lowercase e
    LoE = 0x7B,
    /// Lowercase G
    LoG = 0x6F,
    /// Lowercase H
    LoH = 0x74,
    /// Lowercase I
    LoI = 0x10,
    /// Lowercase N
    LoN = 0x54,
    /// Lowercase O
    LoO = 0x5C,
    /// Lowercase Q
    LoQ = 0x67,
    /// Lowercase R
    LoR = 0x50,
    /// Lowercase T
    LoT = 0x78,
    /// Lowercase U
    LoU = 0x1C,
    /// Lowercase Y
    LoY = 0x6E,
}

impl LoCharBits {
    /// Returns all lowercase characters.
    pub const fn all() -> [LoCharBits; 15] {
        [
            LoCharBits::LoA,
            LoCharBits::LoB,
            LoCharBits::LoC,
            LoCharBits::LoD,
            LoCharBits::LoE,
            LoCharBits::LoG,
            LoCharBits::LoH,
            LoCharBits::LoI,
            LoCharBits::LoN,
            LoCharBits::LoO,
            LoCharBits::LoQ,
            LoCharBits::LoR,
            LoCharBits::LoT,
            LoCharBits::LoU,
            LoCharBits::LoY,
        ]
    }

    /// Returns all lowercase characters as [`u8`].
    pub const fn all_u8() -> [u8; 15] {
        [
            LoCharBits::LoA as u8,
            LoCharBits::LoB as u8,
            LoCharBits::LoC as u8,
            LoCharBits::LoD as u8,
            LoCharBits::LoE as u8,
            LoCharBits::LoG as u8,
            LoCharBits::LoH as u8,
            LoCharBits::LoI as u8,
            LoCharBits::LoN as u8,
            LoCharBits::LoO as u8,
            LoCharBits::LoQ as u8,
            LoCharBits::LoR as u8,
            LoCharBits::LoT as u8,
            LoCharBits::LoU as u8,
            LoCharBits::LoY as u8,
        ]
    }
}

/// Maps a character to its closest possible representation on a 7-segment display.
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SpecialCharBits {
    /// Space symbol
    Space = 0,
    /// Minus or dash symbol
    Minus = SegmentBits::SegG as u8,
    /// Underscore (_)
    Underscore = SegmentBits::SegD as u8,
    /// Equal sign (=)
    Equals = SegmentBits::SegG as u8 | SegmentBits::SegD as u8,
    /// Question mark (?)
    QuestionMark = SegmentBits::SegA as u8
        | SegmentBits::SegB as u8
        | SegmentBits::SegG as u8
        | SegmentBits::SegE as u8,
}

impl SpecialCharBits {
    /// Returns all special characters.
    pub const fn all() -> [SpecialCharBits; 5] {
        [
            SpecialCharBits::Space,
            SpecialCharBits::Minus,
            SpecialCharBits::Underscore,
            SpecialCharBits::Equals,
            SpecialCharBits::QuestionMark,
        ]
    }

    /// Returns all special characters as [`u8`].
    pub const fn all_u8() -> [u8; 5] {
        [
            SpecialCharBits::Space as u8,
            SpecialCharBits::Minus as u8,
            SpecialCharBits::Underscore as u8,
            SpecialCharBits::Equals as u8,
            SpecialCharBits::QuestionMark as u8,
        ]
    }
}

/// Flips the segments of a byte upside down.
///
/// Swaps the segments:
/// - A and D
/// - B and C
/// - E and F
pub const fn flip(byte: u8) -> u8 {
    let a_d_swapped = ((byte & 0b00001000) >> 3) | ((byte & 0b00000001) << 3);
    let b_c_swapped = ((byte & 0b00000100) >> 1) | ((byte & 0b00000010) << 1);
    let e_f_swapped = ((byte & 0b00100000) >> 1) | ((byte & 0b00010000) << 1);

    (byte & 0b11000000) | a_d_swapped | b_c_swapped | e_f_swapped
}

/// Mirrors the segments of a byte.
///
/// Swaps the segments:
/// - B and F
/// - C and E
pub const fn mirror(byte: u8) -> u8 {
    let b_f_swapped = ((byte & 0b00100000) >> 4) | ((byte & 0b00000010) << 4);
    let c_e_swapped = ((byte & 0b00010000) >> 2) | ((byte & 0b00000100) << 2);

    (byte & 0b11001001) | b_f_swapped | c_e_swapped
}

/// Flips and mirrors the segments of a byte.
///
/// See [`flip`] and [`mirror`] for more information.
pub const fn flip_mirror(byte: u8) -> u8 {
    mirror(flip(byte))
}

/// Converts an `ASCII` byte to a 7-segment display byte.
///
/// Unknown characters are converted to `0` (all segments off).
///
/// # Note
///
/// Rust strings are `UTF-8` encoded, so what you see as a single character may be multiple bytes.
///
/// # Example
///
/// Display `Err` text on a 4-digit display:
///
/// ```rust
/// use tm1637_embedded_hal::{mappings::from_ascii_byte, mock::Noop, TM1637Builder};
///
/// let mut tm = TM1637Builder::new(Noop, Noop, Noop).build_blocking::<4>();
///
/// tm.init().ok();
///
/// let err = "Err".as_bytes().iter().copied().map(from_ascii_byte);
///
/// tm.display(0, err).ok();
/// ```
pub const fn from_ascii_byte(byte: u8) -> u8 {
    match byte {
        b'0' => DigitBits::Zero as u8,
        b'1' => DigitBits::One as u8,
        b'2' => DigitBits::Two as u8,
        b'3' => DigitBits::Three as u8,
        b'4' => DigitBits::Four as u8,
        b'5' => DigitBits::Five as u8,
        b'6' => DigitBits::Six as u8,
        b'7' => DigitBits::Seven as u8,
        b'8' => DigitBits::Eight as u8,
        b'9' => DigitBits::Nine as u8,

        b'A' => UpCharBits::UpA as u8,
        b'B' => UpCharBits::UpB as u8,
        b'C' => UpCharBits::UpC as u8,
        b'E' => UpCharBits::UpE as u8,
        b'F' => UpCharBits::UpF as u8,
        b'G' => UpCharBits::UpG as u8,
        b'H' => UpCharBits::UpH as u8,
        b'I' => UpCharBits::UpI as u8,
        b'J' => UpCharBits::UpJ as u8,
        b'L' => UpCharBits::UpL as u8,
        b'O' => UpCharBits::UpO as u8,
        b'P' => UpCharBits::UpP as u8,
        b'S' => UpCharBits::UpS as u8,
        b'U' => UpCharBits::UpU as u8,
        b'Z' => UpCharBits::UpZ as u8,

        b'a' => LoCharBits::LoA as u8,
        b'b' => LoCharBits::LoB as u8,
        b'c' => LoCharBits::LoC as u8,
        b'd' => LoCharBits::LoD as u8,
        b'e' => LoCharBits::LoE as u8,
        b'g' => LoCharBits::LoG as u8,
        b'h' => LoCharBits::LoH as u8,
        b'i' => LoCharBits::LoI as u8,
        b'n' => LoCharBits::LoN as u8,
        b'o' => LoCharBits::LoO as u8,
        b'q' => LoCharBits::LoQ as u8,
        b'r' => LoCharBits::LoR as u8,
        b't' => LoCharBits::LoT as u8,
        b'u' => LoCharBits::LoU as u8,
        b'y' => LoCharBits::LoY as u8,

        b' ' => SpecialCharBits::Space as u8,
        b'-' => SpecialCharBits::Minus as u8,
        b'_' => SpecialCharBits::Underscore as u8,
        b'=' => SpecialCharBits::Equals as u8,
        b'?' => SpecialCharBits::QuestionMark as u8,

        _ => 0,
    }
}

/// Converts a `char` to a 7-segment display byte. See [`from_ascii_byte`] for more information.
pub const fn from_char(c: char) -> u8 {
    from_ascii_byte(c as u8)
}

/// Converts a 7-segment display byte to a `str`.
pub const fn str_from_byte(byte: u8) -> &'static str {
    if byte == SegmentBits::Dot as u8 {
        "."
    } else if byte == DigitBits::Zero as u8 {
        "0"
    } else if byte == DigitBits::One as u8 {
        "1"
    } else if byte == DigitBits::Two as u8 {
        "2"
    } else if byte == DigitBits::Three as u8 {
        "3"
    } else if byte == DigitBits::Four as u8 {
        "4"
    } else if byte == DigitBits::Five as u8 {
        "5"
    } else if byte == DigitBits::Six as u8 {
        "6"
    } else if byte == DigitBits::Seven as u8 {
        "7"
    } else if byte == DigitBits::Eight as u8 {
        "8"
    } else if byte == DigitBits::Nine as u8 {
        "9"
    } else if byte == UpCharBits::UpA as u8 {
        "A"
    } else if byte == UpCharBits::UpB as u8 {
        "B"
    } else if byte == UpCharBits::UpC as u8 {
        "C"
    } else if byte == UpCharBits::UpE as u8 {
        "E"
    } else if byte == UpCharBits::UpF as u8 {
        "F"
    } else if byte == UpCharBits::UpG as u8 {
        "G"
    } else if byte == UpCharBits::UpH as u8 {
        "H"
    } else if byte == UpCharBits::UpI as u8 {
        "I"
    } else if byte == UpCharBits::UpJ as u8 {
        "J"
    } else if byte == UpCharBits::UpL as u8 {
        "L"
    } else if byte == UpCharBits::UpP as u8 {
        "P"
    } else if byte == UpCharBits::UpU as u8 {
        "U"
    } else if byte == LoCharBits::LoA as u8 {
        "a"
    } else if byte == LoCharBits::LoB as u8 {
        "b"
    } else if byte == LoCharBits::LoC as u8 {
        "c"
    } else if byte == LoCharBits::LoD as u8 {
        "d"
    } else if byte == LoCharBits::LoE as u8 {
        "e"
    } else if byte == LoCharBits::LoG as u8 {
        "g"
    } else if byte == LoCharBits::LoH as u8 {
        "h"
    } else if byte == LoCharBits::LoI as u8 {
        "i"
    } else if byte == LoCharBits::LoN as u8 {
        "n"
    } else if byte == LoCharBits::LoO as u8 {
        "o"
    } else if byte == LoCharBits::LoQ as u8 {
        "q"
    } else if byte == LoCharBits::LoR as u8 {
        "r"
    } else if byte == LoCharBits::LoT as u8 {
        "t"
    } else if byte == LoCharBits::LoU as u8 {
        "u"
    } else if byte == LoCharBits::LoY as u8 {
        "y"
    } else if byte == SpecialCharBits::Space as u8 {
        " "
    } else if byte == SpecialCharBits::Minus as u8 {
        "-"
    } else if byte == SpecialCharBits::Underscore as u8 {
        "_"
    } else if byte == SpecialCharBits::Equals as u8 {
        "="
    } else if byte == SpecialCharBits::QuestionMark as u8 {
        "?"
    } else if byte == DigitBits::Zero as u8 | SegmentBits::Dot as u8 {
        "0."
    } else if byte == DigitBits::One as u8 | SegmentBits::Dot as u8 {
        "1."
    } else if byte == DigitBits::Two as u8 | SegmentBits::Dot as u8 {
        "2."
    } else if byte == DigitBits::Three as u8 | SegmentBits::Dot as u8 {
        "3."
    } else if byte == DigitBits::Four as u8 | SegmentBits::Dot as u8 {
        "4."
    } else if byte == DigitBits::Five as u8 | SegmentBits::Dot as u8 {
        "5."
    } else if byte == DigitBits::Six as u8 | SegmentBits::Dot as u8 {
        "6."
    } else if byte == DigitBits::Seven as u8 | SegmentBits::Dot as u8 {
        "7."
    } else if byte == DigitBits::Eight as u8 | SegmentBits::Dot as u8 {
        "8."
    } else if byte == DigitBits::Nine as u8 | SegmentBits::Dot as u8 {
        "9."
    } else if byte == UpCharBits::UpA as u8 | SegmentBits::Dot as u8 {
        "A."
    } else if byte == UpCharBits::UpB as u8 | SegmentBits::Dot as u8 {
        "B."
    } else if byte == UpCharBits::UpC as u8 | SegmentBits::Dot as u8 {
        "C."
    } else if byte == UpCharBits::UpE as u8 | SegmentBits::Dot as u8 {
        "E."
    } else if byte == UpCharBits::UpF as u8 | SegmentBits::Dot as u8 {
        "F."
    } else if byte == UpCharBits::UpG as u8 | SegmentBits::Dot as u8 {
        "G."
    } else if byte == UpCharBits::UpH as u8 | SegmentBits::Dot as u8 {
        "H."
    } else if byte == UpCharBits::UpI as u8 | SegmentBits::Dot as u8 {
        "I."
    } else if byte == UpCharBits::UpJ as u8 | SegmentBits::Dot as u8 {
        "J."
    } else if byte == UpCharBits::UpL as u8 | SegmentBits::Dot as u8 {
        "L."
    } else if byte == UpCharBits::UpP as u8 | SegmentBits::Dot as u8 {
        "P."
    } else if byte == UpCharBits::UpU as u8 | SegmentBits::Dot as u8 {
        "U."
    } else if byte == LoCharBits::LoA as u8 | SegmentBits::Dot as u8 {
        "a."
    } else if byte == LoCharBits::LoB as u8 | SegmentBits::Dot as u8 {
        "b."
    } else if byte == LoCharBits::LoC as u8 | SegmentBits::Dot as u8 {
        "c."
    } else if byte == LoCharBits::LoD as u8 | SegmentBits::Dot as u8 {
        "d."
    } else if byte == LoCharBits::LoE as u8 | SegmentBits::Dot as u8 {
        "e."
    } else if byte == LoCharBits::LoG as u8 | SegmentBits::Dot as u8 {
        "g."
    } else if byte == LoCharBits::LoH as u8 | SegmentBits::Dot as u8 {
        "h."
    } else if byte == LoCharBits::LoI as u8 | SegmentBits::Dot as u8 {
        "i."
    } else if byte == LoCharBits::LoN as u8 | SegmentBits::Dot as u8 {
        "n."
    } else if byte == LoCharBits::LoO as u8 | SegmentBits::Dot as u8 {
        "o."
    } else if byte == LoCharBits::LoQ as u8 | SegmentBits::Dot as u8 {
        "q."
    } else if byte == LoCharBits::LoR as u8 | SegmentBits::Dot as u8 {
        "r."
    } else if byte == LoCharBits::LoT as u8 | SegmentBits::Dot as u8 {
        "t."
    } else if byte == LoCharBits::LoU as u8 | SegmentBits::Dot as u8 {
        "u."
    } else if byte == LoCharBits::LoY as u8 | SegmentBits::Dot as u8 {
        "y."
    } else if byte == SpecialCharBits::Minus as u8 | SegmentBits::Dot as u8 {
        "-."
    } else if byte == SpecialCharBits::Underscore as u8 | SegmentBits::Dot as u8 {
        "_."
    } else if byte == SpecialCharBits::Equals as u8 | SegmentBits::Dot as u8 {
        "=."
    } else if byte == SpecialCharBits::QuestionMark as u8 | SegmentBits::Dot as u8 {
        "?."
    } else {
        ""
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flipped_four() {
        let four = DigitBits::Four as u8;
        let flipped_four = flip(four);
        let should_flipped_four = SegmentBits::SegB as u8
            | SegmentBits::SegC as u8
            | SegmentBits::SegE as u8
            | SegmentBits::SegG as u8;

        assert_eq!(flipped_four, should_flipped_four);
    }

    #[test]
    fn flipped_e() {
        let e = UpCharBits::UpE as u8;
        let flipped_e = flip(e);
        let should_flipped_e = UpCharBits::UpE as u8;

        assert_eq!(flipped_e, should_flipped_e);
    }

    #[test]
    fn mirrored_four() {
        let four = DigitBits::Four as u8;
        let mirrored_four = mirror(four);
        let should_mirrored_four = SegmentBits::SegB as u8
            | SegmentBits::SegE as u8
            | SegmentBits::SegF as u8
            | SegmentBits::SegG as u8;

        assert_eq!(mirrored_four, should_mirrored_four);
    }

    #[test]
    fn mirrored_e() {
        let e = UpCharBits::UpE as u8;
        let mirrored_e = mirror(e);
        let should_mirrored_e = SegmentBits::SegA as u8
            | SegmentBits::SegB as u8
            | SegmentBits::SegC as u8
            | SegmentBits::SegD as u8
            | SegmentBits::SegG as u8;

        assert_eq!(mirrored_e, should_mirrored_e);
    }

    #[test]
    fn flipped_mirrored_four() {
        let four = DigitBits::Four as u8;
        let flipped_mirrored_four = flip_mirror(four);
        let should_flipped_mirrored_four = SegmentBits::SegC as u8
            | SegmentBits::SegE as u8
            | SegmentBits::SegF as u8
            | SegmentBits::SegG as u8;

        assert_eq!(flipped_mirrored_four, should_flipped_mirrored_four);
    }

    #[test]
    fn mirrored_flipped_is_flipped_mirrored() {
        let four = DigitBits::Four as u8;

        let mirrored_flipped_four = mirror(flip(four));
        let flipped_mirrored_four = flip(mirror(four));

        assert_eq!(mirrored_flipped_four, flipped_mirrored_four);
    }

    #[test]
    fn flipped_flipped_is_original() {
        let seven = DigitBits::Seven as u8;

        let flipped_flipped_seven = flip(flip(seven));

        assert_eq!(seven, flipped_flipped_seven);
    }

    #[test]
    fn mirrored_mirrored_is_original() {
        let five = DigitBits::Five as u8;

        let mirrored_mirrored_five = mirror(mirror(five));

        assert_eq!(five, mirrored_mirrored_five);
    }
}
