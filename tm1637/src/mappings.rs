//! Mappings for 7-segment display characters.

//       A
//      ---
//  F  |   |  B
//      -G-
//  E  |   |  C
//      ---
//       D

/// Maps the segment from the device to its bit.
#[repr(u8)]
#[derive(Debug)]
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
    /// Double point
    ///
    /// ## Usage
    /// `Or` this bit with the bit responsible for displaying the double point. Often second position.
    SegPoint = 0b10000000,
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
            SegmentBits::SegPoint,
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
            SegmentBits::SegPoint as u8,
        ]
    }
}

/// Maps a digit to its closest possible representation on a 7-segment display.
#[repr(u8)]
#[derive(Debug)]
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

/// Maps a character to its closest possible representation on a 7-segment display.
#[repr(u8)]
#[derive(Debug)]
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
#[derive(Debug)]
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
#[derive(Debug)]
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
