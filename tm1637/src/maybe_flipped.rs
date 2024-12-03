use crate::tokens::{Flipped, NotFlipped};

pub trait MaybeFlipped<const N: usize> {
    fn calculate(
        position: usize,
        bytes: impl DoubleEndedIterator<Item = u8> + ExactSizeIterator<Item = u8>,
    ) -> (usize, impl Iterator<Item = u8>);

    fn position(position: usize, len: usize) -> usize;
}

impl<const N: usize> MaybeFlipped<N> for NotFlipped {
    fn calculate(
        position: usize,
        bytes: impl DoubleEndedIterator<Item = u8> + ExactSizeIterator<Item = u8>,
    ) -> (usize, impl Iterator<Item = u8>) {
        (position, bytes)
    }

    fn position(position: usize, _: usize) -> usize {
        position
    }
}

// TODO: fix the flip
impl<const N: usize> MaybeFlipped<N> for Flipped {
    fn calculate(
        position: usize,
        bytes: impl DoubleEndedIterator<Item = u8> + ExactSizeIterator<Item = u8>,
    ) -> (usize, impl Iterator<Item = u8>) {
        #[auto_enums::auto_enum(ExactSizeIterator)]
        fn calculate_bytes<const N: usize>(
            bytes: impl DoubleEndedIterator<Item = u8> + ExactSizeIterator<Item = u8>,
            position: usize,
        ) -> impl ExactSizeIterator<Item = u8> {
            match bytes.len() + position > N {
                true => bytes.take(N - position).rev(),
                false => bytes.rev(),
            }
        }

        let position_ = match bytes.len() + position > N {
            true => 0,
            false => N - bytes.len() - position,
        };

        let bytes = calculate_bytes::<N>(bytes, position);

        (
            // Assertion is failing on esp!
            position_,
            bytes.map(crate::mappings::flip_mirror),
        )
    }

    fn position(_: usize, _: usize) -> usize {
        0
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
    fn flipped() {
        // less 2
        let iter = [DigitBits::Zero as u8].iter().copied();

        let (position, bytes) = <Flipped as MaybeFlipped<3>>::calculate(0, iter.clone());
        let bytes = bytes.collect::<Vec<u8>>();

        assert_eq!(position, 2);
        assert_eq!(bytes, vec![DigitBits::Zero as u8]);

        let (position, bytes) = <Flipped as MaybeFlipped<3>>::calculate(1, iter.clone());
        let bytes = bytes.collect::<Vec<_>>();

        assert_eq!(position, 1);
        assert_eq!(bytes, vec![DigitBits::Zero as u8]);

        let (position, bytes) = <Flipped as MaybeFlipped<3>>::calculate(2, iter.clone());
        let bytes = bytes.collect::<Vec<_>>();

        assert_eq!(position, 0);
        assert_eq!(bytes, vec![DigitBits::Zero as u8]);

        let (position, bytes) = <Flipped as MaybeFlipped<3>>::calculate(3, iter.clone());
        let bytes = bytes.collect::<Vec<_>>();

        assert_eq!(position, 0);
        assert_eq!(bytes, vec![]);

        // less 1
        let iter = [DigitBits::Zero as u8, DigitBits::Eight as u8]
            .iter()
            .copied();

        let (position, bytes) = <Flipped as MaybeFlipped<3>>::calculate(0, iter.clone());
        let bytes = bytes.collect::<Vec<u8>>();

        assert_eq!(position, 1);
        assert_eq!(bytes, vec![DigitBits::Eight as u8, DigitBits::Zero as u8]);

        let (position, bytes) = <Flipped as MaybeFlipped<3>>::calculate(1, iter.clone());
        let bytes = bytes.collect::<Vec<_>>();

        assert_eq!(position, 0);
        assert_eq!(bytes, vec![DigitBits::Eight as u8, DigitBits::Zero as u8]);

        let (position, bytes) = <Flipped as MaybeFlipped<3>>::calculate(2, iter.clone());
        let bytes = bytes.collect::<Vec<_>>();

        assert_eq!(position, 0);
        assert_eq!(bytes, vec![DigitBits::Zero as u8]);

        let (position, bytes) = <Flipped as MaybeFlipped<3>>::calculate(3, iter.clone());
        let bytes = bytes.collect::<Vec<_>>();

        assert_eq!(position, 0);
        assert_eq!(bytes, vec![]);

        // exact
        let iter = [
            DigitBits::Zero as u8,
            DigitBits::Eight as u8,
            DigitBits::Three as u8,
        ]
        .iter()
        .copied();

        let (position, bytes) = <Flipped as MaybeFlipped<3>>::calculate(0, iter.clone());
        let bytes = bytes.collect::<Vec<u8>>();

        assert_eq!(position, 0);
        assert_eq!(
            bytes,
            vec![
                UpCharBits::UpE as u8,
                DigitBits::Eight as u8,
                DigitBits::Zero as u8
            ]
        );

        let (position, bytes) = <Flipped as MaybeFlipped<3>>::calculate(1, iter.clone());
        let bytes = bytes.collect::<Vec<_>>();

        assert_eq!(position, 0);
        assert_eq!(bytes, vec![DigitBits::Eight as u8, DigitBits::Zero as u8]);

        let (position, bytes) = <Flipped as MaybeFlipped<3>>::calculate(2, iter.clone());
        let bytes = bytes.collect::<Vec<_>>();

        assert_eq!(position, 0);
        assert_eq!(bytes, vec![DigitBits::Zero as u8]);

        let (position, bytes) = <Flipped as MaybeFlipped<3>>::calculate(3, iter.clone());
        let bytes = bytes.collect::<Vec<_>>();

        assert_eq!(position, 0);
        assert_eq!(bytes, vec![]);

        // greater
        let iter = [
            DigitBits::Zero as u8,
            DigitBits::Eight as u8,
            DigitBits::Three as u8,
            DigitBits::Eight as u8,
        ]
        .iter()
        .copied();

        let (position, bytes) = <Flipped as MaybeFlipped<3>>::calculate(0, iter.clone());
        let bytes = bytes.collect::<Vec<u8>>();

        assert_eq!(position, 0);
        assert_eq!(
            bytes,
            vec![
                UpCharBits::UpE as u8,
                DigitBits::Eight as u8,
                DigitBits::Zero as u8,
            ]
        );

        let (position, bytes) = <Flipped as MaybeFlipped<3>>::calculate(1, iter.clone());
        let bytes = bytes.collect::<Vec<_>>();

        assert_eq!(position, 0);
        assert_eq!(bytes, vec![DigitBits::Eight as u8, DigitBits::Zero as u8,]);

        let (position, bytes) = <Flipped as MaybeFlipped<3>>::calculate(2, iter.clone());
        let bytes = bytes.collect::<Vec<_>>();

        assert_eq!(position, 0);
        assert_eq!(bytes, vec![DigitBits::Zero as u8,]);

        let (position, bytes) = <Flipped as MaybeFlipped<3>>::calculate(3, iter.clone());
        let bytes = bytes.collect::<Vec<_>>();

        assert_eq!(position, 0);
        assert_eq!(bytes, vec![]);
    }
}
