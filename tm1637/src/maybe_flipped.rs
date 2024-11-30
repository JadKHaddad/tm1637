use crate::{Flipped, NotFlipped};

pub trait MaybeFlipped<const N: usize> {
    fn calculate(
        position: usize,
        bytes: impl DoubleEndedIterator<Item = u8> + ExactSizeIterator<Item = u8>,
    ) -> (usize, impl Iterator<Item = u8>);
}

impl<const N: usize> MaybeFlipped<N> for NotFlipped {
    fn calculate(
        position: usize,
        bytes: impl DoubleEndedIterator<Item = u8> + ExactSizeIterator<Item = u8>,
    ) -> (usize, impl Iterator<Item = u8>) {
        (position, bytes)
    }
}

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

        let bytes = calculate_bytes::<N>(bytes, position);

        (
            N - position - bytes.len(),
            bytes.map(crate::mappings::flip_mirror),
        )
    }
}
