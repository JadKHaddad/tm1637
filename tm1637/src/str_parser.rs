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
    size: usize,
}

impl<'a> StrParser<'a> {
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
}
