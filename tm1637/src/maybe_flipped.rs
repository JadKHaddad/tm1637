use crate::tokens::{Flipped, NotFlipped};

/// A trait for recalculating the `position` and `bytes` on a `maybe flipped` display.
pub trait MaybeFlipped<const N: usize> {
    /// Calculate the new `position` and `bytes` for the display.
    fn calculate(
        position: usize,
        bytes: impl DoubleEndedIterator<Item = u8> + ExactSizeIterator<Item = u8>,
    ) -> (
        usize,
        impl DoubleEndedIterator<Item = u8> + ExactSizeIterator<Item = u8>,
    );

    /// Calculate the new `position` for the display.
    fn position(position: usize, len: usize) -> usize;

    /// Flip the display.
    fn flip() -> impl MaybeFlipped<N>;
}

impl<const N: usize> MaybeFlipped<N> for NotFlipped {
    fn calculate(
        position: usize,
        bytes: impl DoubleEndedIterator<Item = u8> + ExactSizeIterator<Item = u8>,
    ) -> (
        usize,
        impl DoubleEndedIterator<Item = u8> + ExactSizeIterator<Item = u8>,
    ) {
        (position, bytes)
    }

    fn position(position: usize, _: usize) -> usize {
        position
    }

    fn flip() -> impl MaybeFlipped<N> {
        Flipped
    }
}

impl<const N: usize> MaybeFlipped<N> for Flipped {
    fn calculate(
        position: usize,
        bytes: impl DoubleEndedIterator<Item = u8> + ExactSizeIterator<Item = u8>,
    ) -> (
        usize,
        impl DoubleEndedIterator<Item = u8> + ExactSizeIterator<Item = u8>,
    ) {
        fn calculate_bytes<const N: usize>(
            bytes: impl DoubleEndedIterator<Item = u8> + ExactSizeIterator<Item = u8>,
            position: usize,
        ) -> impl DoubleEndedIterator<Item = u8> + ExactSizeIterator<Item = u8> {
            #[auto_enums::enum_derive(DoubleEndedIterator)]
            enum I<A, B, C> {
                A(A),
                B(B),
                C(C),
            }

            impl<
                    A: ExactSizeIterator<Item = u8>,
                    B: ExactSizeIterator<Item = u8>,
                    C: ExactSizeIterator<Item = u8>,
                > ExactSizeIterator for I<A, B, C>
            {
                fn len(&self) -> usize {
                    match self {
                        I::A(a) => a.len(),
                        I::B(b) => b.len(),
                        I::C(c) => c.len(),
                    }
                }
            }

            if position > N {
                return I::C(core::iter::empty());
            }

            match bytes.len() + position > N {
                true => I::A(bytes.take(N - position).rev()),
                false => I::B(bytes.rev()),
            }
        }

        let new_position = match bytes.len() + position > N {
            true => 0,
            false => N - bytes.len() - position,
        };

        let bytes = calculate_bytes::<N>(bytes, position);

        (new_position, bytes.map(crate::mappings::flip_mirror))
    }

    fn position(position: usize, len: usize) -> usize {
        match len + position > N {
            true => 0,
            false => N - len - position,
        }
    }

    fn flip() -> impl MaybeFlipped<N> {
        NotFlipped
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
