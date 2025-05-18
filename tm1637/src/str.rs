//! [`str`] parsing utilities.

use ::core::str::Bytes;

use crate::mappings::{from_ascii_byte, SegmentBits};

/// Parse a string to it's corresponding 7-segment display bits.
///
/// Dots are ignored unless they appear after a character. The dot is then or'd with the character.
///
/// # Example
///
/// ```rust
/// use tm1637_embedded_hal::{str::StrParser, mappings::{DigitBits, LoCharBits, SegmentBits, UpCharBits}};
///
/// let input = "..oH.3..45..............6.7";
/// let mut parser = StrParser::new(input);
///
/// assert_eq!(7, parser.len());
/// assert_eq!(LoCharBits::LoO as u8, parser.next().unwrap());
/// assert_eq!(UpCharBits::UpH as u8 | SegmentBits::Dot as u8, parser.next().unwrap());
/// assert_eq!(DigitBits::Seven as u8, parser.next_back().unwrap());
/// assert_eq!(4, parser.len());
/// ```
///
#[derive(Debug, Clone)]
pub struct StrParser<'a> {
    bytes: Bytes<'a>,
    current: Option<u8>,
    /// While reading backwards, we need to know if we have read a dot.
    ///
    /// - `0`: No dot has been read.
    /// - `SegmentBits::Dot as u8`: A dot has been read.
    ///
    /// We or this value with the found digit on the next call to `next_back`.
    or: u8,
    size: usize,
}

#[cfg(feature = "defmt")]
impl defmt::Format for StrParser<'_> {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "StrParser {{ .. }}")
    }
}

impl<'a> StrParser<'a> {
    /// Create a new [`StrParser`] from a &[`str`].
    pub fn new(str: &'a str) -> Self {
        Self {
            bytes: str.bytes(),
            current: None,
            or: 0,
            size: str.bytes().filter(|byte| *byte != b'.').count(),
        }
    }
}

impl<'a> From<&'a str> for StrParser<'a> {
    fn from(value: &'a str) -> Self {
        Self::new(value)
    }
}

impl Iterator for StrParser<'_> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.size == 0 {
            return None;
        }

        loop {
            match self.bytes.next() {
                Some(byte) => match byte {
                    b'.' => match self.current.take() {
                        Some(current) => {
                            self.size -= 1;

                            return Some(from_ascii_byte(current) | SegmentBits::Dot as u8);
                        }
                        None => continue,
                    },
                    byte => match self.current.replace(byte) {
                        Some(current) => {
                            self.size -= 1;

                            return Some(from_ascii_byte(current));
                        }
                        None => self.current = Some(byte),
                    },
                },
                None => match self.current.take().map(from_ascii_byte) {
                    Some(current) => {
                        self.size -= 1;

                        return Some(current);
                    }
                    None => return None,
                },
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.size, Some(self.size))
    }
}

impl ExactSizeIterator for StrParser<'_> {}

impl DoubleEndedIterator for StrParser<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.size == 0 {
            return None;
        }

        loop {
            match self.bytes.next_back() {
                Some(byte) => match byte {
                    b'.' => {
                        self.or = SegmentBits::Dot as u8;

                        continue;
                    }
                    byte => {
                        let byte = from_ascii_byte(byte) | self.or;

                        self.or = 0;
                        self.size -= 1;

                        return Some(byte);
                    }
                },
                None => {
                    return match self.current.take() {
                        Some(current) => {
                            self.size -= 1;

                            Some(from_ascii_byte(current) | self.or)
                        }
                        None => {
                            self.or = 0;

                            None
                        }
                    };
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate std;
    use std::vec;
    use std::vec::Vec;

    use crate::mappings::{DigitBits, UpCharBits};

    use super::*;

    #[test]
    fn no_dots() {
        let parser = StrParser::new("1234");
        let result: Vec<u8> = parser.collect();

        assert_eq!(
            vec![
                DigitBits::One as u8,
                DigitBits::Two as u8,
                DigitBits::Three as u8,
                DigitBits::Four as u8
            ],
            result
        );

        let parser = StrParser::new("HE.LLO");
        let result: Vec<u8> = parser.take(3).rev().collect();

        assert_eq!(
            vec![
                UpCharBits::UpH as u8,
                UpCharBits::UpE as u8 | SegmentBits::Dot as u8,
                UpCharBits::UpL as u8,
            ]
            .into_iter()
            .rev()
            .collect::<Vec<_>>(),
            result
        );
    }

    #[test]
    fn len() {
        let mut parser = StrParser::new("..12.3..45..............6.7");

        assert_eq!(7, parser.len());
        parser.next();
        assert_eq!(6, parser.len());
        parser.next();
        assert_eq!(5, parser.len());
        parser.next();
        assert_eq!(4, parser.len());
        parser.next();
        assert_eq!(3, parser.len());
        parser.next();
        assert_eq!(2, parser.len());
        parser.next();
        assert_eq!(1, parser.len());
        parser.next();
        assert_eq!(0, parser.len());
        assert_eq!(None, parser.next());
        assert_eq!(0, parser.len());
    }

    #[test]
    fn dots() {
        let parser = StrParser::new("..12.3..45..............6.7");
        let result: Vec<u8> = parser.collect();
        assert_eq!(
            vec![
                DigitBits::One as u8,
                DigitBits::Two as u8 | SegmentBits::Dot as u8,
                DigitBits::Three as u8 | SegmentBits::Dot as u8,
                DigitBits::Four as u8,
                DigitBits::Five as u8 | SegmentBits::Dot as u8,
                DigitBits::Six as u8 | SegmentBits::Dot as u8,
                DigitBits::Seven as u8
            ],
            result
        );
    }

    #[test]
    fn no_dots_rev() {
        let parser = StrParser::new("1234");
        let result: Vec<u8> = parser.rev().collect();
        let mut expected = vec![
            DigitBits::One as u8,
            DigitBits::Two as u8,
            DigitBits::Three as u8,
            DigitBits::Four as u8,
        ];
        expected.reverse();
        assert_eq!(expected, result);
    }

    #[test]
    fn dots_rev() {
        let parser = StrParser::new("..12.3..45..............6.7");
        let result: Vec<u8> = parser.rev().collect();
        let mut expected = vec![
            DigitBits::One as u8,
            DigitBits::Two as u8 | SegmentBits::Dot as u8,
            DigitBits::Three as u8 | SegmentBits::Dot as u8,
            DigitBits::Four as u8,
            DigitBits::Five as u8 | SegmentBits::Dot as u8,
            DigitBits::Six as u8 | SegmentBits::Dot as u8,
            DigitBits::Seven as u8,
        ];
        expected.reverse();
        assert_eq!(expected, result);
    }

    #[test]
    fn back_and_forth() {
        let mut parser = StrParser::new("..12.3..45..............6.7");

        assert_eq!(7, parser.len());

        assert_eq!(Some(DigitBits::One as u8), parser.next());
        assert_eq!(6, parser.len());

        assert_eq!(
            Some(DigitBits::Two as u8 | SegmentBits::Dot as u8),
            parser.next()
        );
        assert_eq!(5, parser.len());

        assert_eq!(Some(DigitBits::Seven as u8), parser.next_back());
        assert_eq!(4, parser.len());

        assert_eq!(
            Some(DigitBits::Three as u8 | SegmentBits::Dot as u8),
            parser.next()
        );
        assert_eq!(3, parser.len());

        assert_eq!(
            Some(DigitBits::Six as u8 | SegmentBits::Dot as u8),
            parser.next_back()
        );
        assert_eq!(2, parser.len());

        assert_eq!(Some(DigitBits::Four as u8), parser.next());
        assert_eq!(1, parser.len());

        assert_eq!(
            Some(DigitBits::Five as u8 | SegmentBits::Dot as u8),
            parser.next()
        );
        assert_eq!(0, parser.len());

        assert_eq!(None, parser.next_back());
        assert_eq!(None, parser.next());
        assert_eq!(0, parser.len());
    }

    // TODO: remove this test and add a test for this case where next and next back overlap with dots and everything else
    // After fix remove the test in circular.rs (fn see())
    #[test]
    #[ignore = "debug"]
    fn see() {
        let mut parser = StrParser::new("012345678");

        let elem = parser.next().unwrap();
        std::println!("elem: {:?}", crate::mappings::str_from_byte(elem));

        let elem = parser.next().unwrap();
        std::println!("elem: {:?}", crate::mappings::str_from_byte(elem));

        let elem = parser.next().unwrap();
        std::println!("elem: {:?}", crate::mappings::str_from_byte(elem));

        let elem = parser.next().unwrap();
        std::println!("elem: {:?}", crate::mappings::str_from_byte(elem));

        std::println!();

        let elem = parser.next_back().unwrap();
        std::println!("elem: {:?}", crate::mappings::str_from_byte(elem));

        let elem = parser.next_back().unwrap();
        std::println!("elem: {:?}", crate::mappings::str_from_byte(elem));

        let elem = parser.next_back().unwrap();
        std::println!("elem: {:?}", crate::mappings::str_from_byte(elem));

        let elem = parser.next_back().unwrap();
        std::println!("elem: {:?}", crate::mappings::str_from_byte(elem));
        let elem = parser.next_back().unwrap();
        std::println!("elem: {:?}", crate::mappings::str_from_byte(elem));

        std::println!("------------------------->");

        let mut parser = "012345678".bytes();

        let elem = parser.next().unwrap();
        std::println!("elem: {:?}", elem as char);

        let elem = parser.next().unwrap();
        std::println!("elem: {:?}", elem as char);

        let elem = parser.next().unwrap();
        std::println!("elem: {:?}", elem as char);

        let elem = parser.next().unwrap();
        std::println!("elem: {:?}", elem as char);

        std::println!();

        let elem = parser.next_back().unwrap();
        std::println!("elem: {:?}", elem as char);

        let elem = parser.next_back().unwrap();
        std::println!("elem: {:?}", elem as char);

        let elem = parser.next_back().unwrap();
        std::println!("elem: {:?}", elem as char);

        let elem = parser.next_back().unwrap();
        std::println!("elem: {:?}", elem as char);

        let elem = parser.next_back().unwrap();
        std::println!("elem: {:?}", elem as char);
    }
}
