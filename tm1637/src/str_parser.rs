use core::str::Bytes;

use crate::mappings::{from_ascii_byte, SegmentBits};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct StrParser<'a> {
    bytes: Bytes<'a>,
    current: Option<u8>,
    /// While reading backwards, we need to know if we have read a dot.
    ///
    /// - `0`: No dot has been read.
    /// - `SegmentBits::Dot as u8`: A dot has been read.
    ///
    /// We or this value with the found digit on the next call to next_back.
    or: u8,
}

impl<'a> StrParser<'a> {
    pub fn new(str: &'a str) -> Self {
        Self {
            bytes: str.bytes(),
            current: None,
            or: 0,
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
        loop {
            match self.bytes.next() {
                Some(byte) => match byte {
                    b'.' => match self.current.take() {
                        Some(current) => {
                            return Some(from_ascii_byte(current) | SegmentBits::Dot as u8);
                        }

                        None => continue,
                    },
                    byte => match self.current.replace(byte) {
                        Some(current) => {
                            return Some(from_ascii_byte(current));
                        }

                        None => self.current = Some(byte),
                    },
                },

                None => return self.current.take().map(from_ascii_byte),
            }
        }
    }
}

impl ExactSizeIterator for StrParser<'_> {
    fn len(&self) -> usize {
        self.bytes.clone().filter(|byte| *byte != b'.').count()
    }
}

impl DoubleEndedIterator for StrParser<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
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

                        return Some(byte);
                    }
                },

                None => return None,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate std;
    use std::vec;
    use std::vec::Vec;

    use crate::mappings::DigitBits;

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
    }

    #[test]
    fn len() {
        let parser = StrParser::new("..12.3..45..............6.7");

        assert_eq!(7, parser.len());
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
    fn back_no_dots() {
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
    fn back_dots() {
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
}
