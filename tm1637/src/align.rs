use crate::exact_size::ExactSizeChainExt;

pub trait Aligned {
    /// Aligns the bytes from a human readable sequence to a sequence that can be written to the display.
    fn align(
        position: usize,
        iter: impl DoubleEndedIterator<Item = u8> + ExactSizeIterator,
    ) -> (usize, impl Iterator<Item = u8>) {
        (position, iter)
    }

    // Aligns the position
    fn position(position: usize) -> usize {
        position
    }
}

pub struct Align<const N: usize>;

impl Aligned for Align<4> {
    fn align(
        position: usize,
        iter: impl DoubleEndedIterator<Item = u8> + ExactSizeIterator,
    ) -> (usize, impl Iterator<Item = u8>) {
        // Don't write more bytes than needed
        (position, iter.take(4 - position))
    }
}

impl Aligned for Align<6> {
    fn align(
        position: usize,
        iter: impl DoubleEndedIterator<Item = u8> + ExactSizeIterator,
    ) -> (usize, impl Iterator<Item = u8>) {
        #[auto_enums::enum_derive(Iterator)]
        enum I<A, B> {
            A(A),
            B(B),
        }

        if position > 6 {
            return (3, I::A(core::iter::empty()));
        }

        let iter = padding_6(iter).take(6 - position).rev();

        // 3 is a magic number to make the alignment work
        // Reversed iterators with <= 6 elements will align perfectly using position 3
        (3, I::B(iter))
    }

    fn position(_: usize) -> usize {
        3
    }
}

/// Iterators with less than 6 elements will never align whithout leaving empty spaces between digits, so we pad with zeros
///
/// The padding will be written to the display, overwriting the digits that are already there.
fn padding_6(
    iter: impl DoubleEndedIterator<Item = u8> + ExactSizeIterator,
) -> impl DoubleEndedIterator<Item = u8> + ExactSizeIterator {
    #[auto_enums::enum_derive(DoubleEndedIterator)]
    enum I<A, B> {
        A(A),
        B(B),
    }

    impl<A: ExactSizeIterator<Item = u8>, B: ExactSizeIterator<Item = u8>> ExactSizeIterator
        for I<A, B>
    {
        fn len(&self) -> usize {
            match self {
                I::A(a) => a.len(),
                I::B(b) => b.len(),
            }
        }
    }

    let len = iter.len();

    if len < 6 {
        return I::A(iter.exact_size_chain(core::iter::repeat(0).take(6 - len)));
    };

    I::B(iter)
}
