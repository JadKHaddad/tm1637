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
    /// `Or` this bit with the bit responsible for displaying the double point. Often second address.
    SegPoint = 0b10000000,
}

impl SegmentBits {
    /// Returns all segments.
    pub fn all() -> [SegmentBits; 8] {
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
    pub fn all_u8() -> [u8; 8] {
        Self::all().map(|bit| bit as u8)
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
    pub fn all() -> [DigitBits; 10] {
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
    pub fn all_u8() -> [u8; 10] {
        Self::all().map(|bit| bit as u8)
    }

    /// Create a new [`DigitBits`] from a [`u8`] digit.
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
}

impl UpCharBits {
    /// Returns all uppercase characters.
    pub fn all() -> [UpCharBits; 13] {
        [
            UpCharBits::UpA,
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
        ]
    }

    /// Returns all uppercase characters as [`u8`].
    pub fn all_u8() -> [u8; 13] {
        Self::all().map(|bit| bit as u8)
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
    /// Lowercase H
    LoH = 0x74,
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
    pub fn all() -> [LoCharBits; 12] {
        [
            LoCharBits::LoA,
            LoCharBits::LoB,
            LoCharBits::LoC,
            LoCharBits::LoD,
            LoCharBits::LoH,
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
    pub fn all_u8() -> [u8; 12] {
        Self::all().map(|bit| bit as u8)
    }
}

/// Maps a character to its closest possible representation on a 7-segment display.
/// The 8th segment is the dot.
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
    /// Dot (.)
    Dot = SegmentBits::SegPoint as u8,
}

impl SpecialCharBits {
    /// Returns all special characters.
    pub fn all() -> [SpecialCharBits; 6] {
        [
            SpecialCharBits::Space,
            SpecialCharBits::Minus,
            SpecialCharBits::Underscore,
            SpecialCharBits::Equals,
            SpecialCharBits::QuestionMark,
            SpecialCharBits::Dot,
        ]
    }

    /// Returns all special characters as [`u8`].
    pub fn all_u8() -> [u8; 6] {
        Self::all().map(|bit| bit as u8)
    }
}
