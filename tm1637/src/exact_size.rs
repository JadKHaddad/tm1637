use core::iter::Chain;

#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ExactSize<I> {
    inner: I,
    size: usize,
}

fn exact_size_chain<A, B, I>(first: A, second: B) -> ExactSize<Chain<A, B>>
where
    A: DoubleEndedIterator<Item = I> + ExactSizeIterator,
    B: DoubleEndedIterator<Item = I> + ExactSizeIterator,
{
    let size = first.len() + second.len();
    let iter = first.chain(second);

    ExactSize { inner: iter, size }
}

pub trait ExactSizeChainExt<B>: Sized {
    fn exact_size_chain(self, second: B) -> ExactSize<Chain<Self, B>>;
}

impl<A, B, I> ExactSizeChainExt<B> for A
where
    A: DoubleEndedIterator<Item = I> + ExactSizeIterator,
    B: DoubleEndedIterator<Item = I> + ExactSizeIterator,
{
    fn exact_size_chain(self, second: B) -> ExactSize<Chain<Self, B>> {
        exact_size_chain(self, second)
    }
}

impl<I: Iterator> Iterator for ExactSize<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.next() {
            None => None,
            Some(x) => {
                self.size -= 1;
                Some(x)
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.size, Some(self.size))
    }
}

impl<I: Iterator> ExactSizeIterator for ExactSize<I> {}

impl<I: DoubleEndedIterator> DoubleEndedIterator for ExactSize<I> {
    fn next_back(&mut self) -> Option<Self::Item> {
        match self.inner.next_back() {
            None => None,
            Some(x) => {
                self.size -= 1;
                Some(x)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exact_size() {
        let xs = [0, 1, 2];
        let ys = [30, 40, 50, 60];

        // First iterator is exhausted first
        let mut iter = xs.iter().exact_size_chain(ys.iter());

        assert_eq!(iter.len(), 7);
        assert_eq!(iter.next(), Some(&0));
        assert_eq!(iter.len(), 6);
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.len(), 5);
        assert_eq!(iter.next_back(), Some(&60));
        assert_eq!(iter.len(), 4);
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.len(), 3);
        assert_eq!(iter.next(), Some(&30));
        assert_eq!(iter.len(), 2);
        assert_eq!(iter.next(), Some(&40));
        assert_eq!(iter.len(), 1);
        assert_eq!(iter.next(), Some(&50));
        assert_eq!(iter.len(), 0);
        assert_eq!(iter.next(), None);

        // Second iterator is exhausted first
        let mut iter = xs.iter().exact_size_chain(ys.iter());

        assert_eq!(iter.len(), 7);
        assert_eq!(iter.next_back(), Some(&60));
        assert_eq!(iter.len(), 6);
        assert_eq!(iter.next(), Some(&0));
        assert_eq!(iter.len(), 5);
        assert_eq!(iter.next_back(), Some(&50));
        assert_eq!(iter.len(), 4);
        assert_eq!(iter.next_back(), Some(&40));
        assert_eq!(iter.len(), 3);
        assert_eq!(iter.next_back(), Some(&30));
        assert_eq!(iter.len(), 2);
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.len(), 1);
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.len(), 0);
        assert_eq!(iter.next(), None);
    }
}
