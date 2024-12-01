use crate::mappings::{from_ascii_byte, SegmentBits};

#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct StrParser<I> {
    iter: I,
    current: Option<u8>,
}

impl<I> StrParser<I> {
    pub const fn new(iter: I) -> Self {
        Self {
            iter,
            current: None,
        }
    }
}

impl<I> Iterator for StrParser<I>
where
    I: Iterator<Item = u8>,
{
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.iter.next() {
                Some(byte) => match byte {
                    b'.' => match self.current.take() {
                        Some(current) => {
                            return Some(from_ascii_byte(current) | SegmentBits::SegPoint as u8);
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

#[cfg(test)]
mod tests {
    extern crate std;
    use std::vec;
    use std::vec::Vec;

    use crate::mappings::DigitBits;

    use super::*;

    #[test]
    fn str_parser() {
        let input = b"1234";
        let parser = StrParser::new(input.iter().copied());
        let result: Vec<u8> = parser.collect();
        assert_eq!(
            result,
            vec![
                DigitBits::One as u8,
                DigitBits::Two as u8,
                DigitBits::Three as u8,
                DigitBits::Four as u8
            ]
        );
    }

    #[test]
    fn str_parser_with_dot() {
        let input = b"..12.3..45..............6.7";
        let parser = StrParser::new(input.iter().copied());
        let result: Vec<u8> = parser.collect();
        assert_eq!(
            result,
            vec![
                DigitBits::One as u8,
                DigitBits::Two as u8 | SegmentBits::SegPoint as u8,
                DigitBits::Three as u8 | SegmentBits::SegPoint as u8,
                DigitBits::Four as u8,
                DigitBits::Five as u8 | SegmentBits::SegPoint as u8,
                DigitBits::Six as u8 | SegmentBits::SegPoint as u8,
                DigitBits::Seven as u8
            ]
        );
    }
}
