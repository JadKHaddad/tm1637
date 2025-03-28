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
        #[auto_enums::enum_derive(Iterator)]
        enum I<A, B> {
            A(A),
            B(B),
        }

        if position > 3 {
            return (position, I::A(core::iter::empty()));
        }

        // Don't write more bytes than needed
        (position, I::B(iter.take(4 - position)))
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

        if position > 5 {
            return (3, I::A(core::iter::empty()));
        }

        // 3 is a magic number to make the alignment work
        // Reversed iterators with <= 6 elements will align perfectly using position 3
        (3, I::B(padding_6(iter).take(6 - position).rev()))
    }

    fn position(_: usize) -> usize {
        3
    }
}

/// Iterators with less than 6 elements will never align without leaving empty spaces between digits, so we pad with zeros
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

#[cfg(test)]
mod tests {
    extern crate std;
    use std::vec;
    use std::vec::Vec;

    use super::*;

    #[test]
    fn align_4() {
        let iter = [1, 2].iter().copied();
        let (position, iter) = Align::<4>::align(0, iter);

        assert_eq!(position, 0);
        assert_eq!(iter.collect::<Vec<_>>(), vec![1, 2,]);

        let iter = [1, 2, 3, 4].iter().copied();
        let (position, iter) = Align::<4>::align(0, iter);

        assert_eq!(position, 0);
        assert_eq!(iter.collect::<Vec<_>>(), vec![1, 2, 3, 4]);

        let iter = [1, 2, 3, 4].iter().copied();
        let (position, iter) = Align::<4>::align(1, iter);

        assert_eq!(position, 1);
        assert_eq!(iter.collect::<Vec<_>>(), vec![1, 2, 3]);

        let iter = [1, 2, 3, 4, 5, 6].iter().copied();
        let (position, iter) = Align::<4>::align(2, iter);

        assert_eq!(position, 2);
        assert_eq!(iter.collect::<Vec<_>>(), vec![1, 2]);

        let iter = [1, 2, 3, 4, 5, 6].iter().copied();
        let (position, iter) = Align::<4>::align(4, iter);

        assert_eq!(position, 4);
        assert_eq!(iter.collect::<Vec<_>>(), vec![]);

        let iter = [1, 2, 3, 4, 5, 6].iter().copied();
        let (position, iter) = Align::<4>::align(5, iter);

        assert_eq!(position, 5);
        assert_eq!(iter.collect::<Vec<_>>(), vec![]);
    }

    #[test]
    fn align_6() {
        // less is padded with zeros
        let iter = [1, 2, 3].iter().copied();
        let (position, iter) = Align::<6>::align(0, iter);

        assert_eq!(position, 3);
        assert_eq!(iter.collect::<Vec<_>>(), vec![0, 0, 0, 3, 2, 1]);

        let iter = [1, 2, 3, 4, 5, 6].iter().copied();
        let (position, iter) = Align::<6>::align(0, iter);

        assert_eq!(position, 3);
        assert_eq!(iter.collect::<Vec<_>>(), vec![6, 5, 4, 3, 2, 1]);

        let iter = [1, 2, 3, 4, 5, 6].iter().copied();
        let (position, iter) = Align::<6>::align(1, iter);

        assert_eq!(position, 3);
        assert_eq!(iter.collect::<Vec<_>>(), vec![5, 4, 3, 2, 1]);

        let iter = [1, 2, 3, 4, 5, 6].iter().copied();
        let (position, iter) = Align::<6>::align(2, iter);

        assert_eq!(position, 3);
        assert_eq!(iter.collect::<Vec<_>>(), vec![4, 3, 2, 1]);

        let iter = [1, 2, 3, 4, 5, 6].iter().copied();
        let (position, iter) = Align::<6>::align(3, iter);

        assert_eq!(position, 3);
        assert_eq!(iter.collect::<Vec<_>>(), vec![3, 2, 1]);

        let iter = [1, 2, 3, 4, 5, 6].iter().copied();
        let (position, iter) = Align::<6>::align(4, iter);

        assert_eq!(position, 3);
        assert_eq!(iter.collect::<Vec<_>>(), vec![2, 1]);

        let iter = [1, 2, 3, 4, 5, 6].iter().copied();
        let (position, iter) = Align::<6>::align(5, iter);

        assert_eq!(position, 3);
        assert_eq!(iter.collect::<Vec<_>>(), vec![1]);

        let iter = [1, 2, 3, 4, 5, 6].iter().copied();
        let (position, iter) = Align::<6>::align(6, iter);

        assert_eq!(position, 3);
        assert_eq!(iter.collect::<Vec<_>>(), vec![]);

        let iter = [1, 2, 3, 4, 5, 6].iter().copied();
        let (position, iter) = Align::<6>::align(7, iter);

        assert_eq!(position, 3);
        assert_eq!(iter.collect::<Vec<_>>(), vec![]);
    }
}
