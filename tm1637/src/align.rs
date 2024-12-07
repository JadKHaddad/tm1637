use crate::exact_size::ExactSizeChainExt;

pub trait Aligned {
    fn align(
        position: usize,
        iter: impl DoubleEndedIterator<Item = u8> + ExactSizeIterator,
    ) -> (usize, impl Iterator<Item = u8>) {
        (position, iter)
    }

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

        let iter = padding(iter).take(6 - position).rev();

        // 3 is a magic number to make the alignment work
        (3, I::B(iter))
    }

    fn position(_: usize) -> usize {
        3
    }
}

fn padding(
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

    // less than 6 digits will never align whithout overwriting the last digits, so we pad with zeros
    if len < 6 {
        return I::A(iter.exact_size_chain(core::iter::repeat(0).take(6 - len)));
    };

    I::B(iter)
}
