use super::{reverse::ReverseWindows, LinearWindows};

/// Internal state for [`CircularWindows`].
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
enum WindowsState<const N: usize> {
    Init,
    First([u8; N]),
    FirstAndLast([u8; N], [u8; N]),
}

/// Circular windows iterator.
///
/// Represents [`ScrollStyle::Circular`](crate::options::ScrollStyle::Circular) and [`ScrollDirection::LeftToRight`](crate::options::ScrollDirection::LeftToRight).
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct CircularWindows<const N: usize, I> {
    state: WindowsState<N>,
    iter: LinearWindows<N, I>,
    counter: usize,
}

impl<const N: usize, I> CircularWindows<N, I> {
    /// Create a new [`CircularWindows`] iterator.
    pub const fn new(iter: I) -> Self {
        Self {
            state: WindowsState::<N>::Init,
            iter: LinearWindows::<N, I>::new(iter),
            counter: 0,
        }
    }
}

/// Shifts the last item to the left and inserts firt[index] at the end.
fn shift_left<const N: usize>(first: &[u8; N], last: &[u8; N], index: usize) -> [u8; N] {
    let mut buffer = [0; N];

    buffer[..(N - 1)].copy_from_slice(&last[1..((N - 1) + 1)]);

    buffer[N - 1] = first[index];

    buffer
}

impl<const N: usize, I> Iterator for CircularWindows<N, I>
where
    I: Iterator<Item = u8>,
{
    type Item = [u8; N];

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(item) => match self.state {
                WindowsState::Init => {
                    self.state = WindowsState::First(item);

                    Some(item)
                }
                WindowsState::First(first_item) => {
                    self.state = WindowsState::FirstAndLast(first_item, item);

                    Some(item)
                }
                WindowsState::FirstAndLast(first_item, _) => {
                    self.state = WindowsState::FirstAndLast(first_item, item);

                    Some(item)
                }
            },
            None => match self.state {
                WindowsState::Init => None,
                WindowsState::First(_) => None,
                WindowsState::FirstAndLast(first_item, last_item) => {
                    if self.counter < N {
                        let item = shift_left(&first_item, &last_item, self.counter);

                        self.counter += 1;

                        self.state = WindowsState::FirstAndLast(first_item, item);

                        return Some(item);
                    }

                    None
                }
            },
        }
    }
}

/// Reversed circular windows iterator.
///
/// Represents [`ScrollStyle::Circular`](crate::options::ScrollStyle::Circular) and [`ScrollDirection::RightToLeft`](crate::options::ScrollDirection::RightToLeft).
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct CircularWindowsReversed<const N: usize, I> {
    state: WindowsState<N>,
    iter: ReverseWindows<N, I>,
    counter: usize,
}

impl<const N: usize, I> CircularWindowsReversed<N, I> {
    /// Create a new [`CircularWindowsReversed`] iterator.
    pub const fn new(iter: I) -> Self {
        Self {
            state: WindowsState::<N>::Init,
            iter: ReverseWindows::<N, I>::new(iter),
            counter: 0,
        }
    }
}

/// Shifts the last item to the right and inserts firt[N - 1 - index] at the front.
fn shift_right<const N: usize>(first: &[u8; N], last: &[u8; N], index: usize) -> [u8; N] {
    let mut buffer = [0; N];

    buffer[1..].copy_from_slice(&last[..(N - 1)]);

    buffer[0] = first[N - 1 - index];

    buffer
}

impl<const N: usize, I> Iterator for CircularWindowsReversed<N, I>
where
    I: DoubleEndedIterator<Item = u8>,
{
    type Item = [u8; N];

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(item) => match self.state {
                WindowsState::Init => {
                    self.state = WindowsState::First(item);

                    Some(item)
                }
                WindowsState::First(first_item) => {
                    self.state = WindowsState::FirstAndLast(first_item, item);

                    Some(item)
                }
                WindowsState::FirstAndLast(first_item, _) => {
                    self.state = WindowsState::FirstAndLast(first_item, item);

                    Some(item)
                }
            },
            None => match self.state {
                WindowsState::Init => None,
                WindowsState::First(_) => None,
                WindowsState::FirstAndLast(first_item, last_item) => {
                    if self.counter < N {
                        let item = shift_right(&first_item, &last_item, self.counter);

                        self.counter += 1;

                        self.state = WindowsState::FirstAndLast(first_item, item);

                        return Some(item);
                    }

                    None
                }
            },
        }
    }
}

#[cfg(test)]
mod test {
    extern crate std;
    use std::vec;
    use std::vec::Vec;

    use super::*;

    #[test]
    fn shift_left_test() {
        let first = [b'l', b'o', b'r'];
        let last = [b'r', b'e', b'm'];

        assert_eq!([b'e', b'm', b'l'], shift_left(&first, &last, 0));
    }

    #[test]
    fn less_than_n() {
        let iter = b"".iter().copied();
        let windows = CircularWindows::<4, _>::new(iter);
        let collected = windows.collect::<Vec<_>>();

        assert_eq!(vec![[0, 0, 0, 0],], collected);

        let iter = b"1".iter().copied();
        let windows = CircularWindows::<4, _>::new(iter);
        let collected = windows.collect::<Vec<_>>();

        assert_eq!(vec![[b'1', 0, 0, 0],], collected);

        let iter = b"12".iter().copied();
        let windows = CircularWindows::<4, _>::new(iter);
        let collected = windows.collect::<Vec<_>>();

        assert_eq!(vec![[b'1', b'2', 0, 0],], collected);

        let iter = b"123".iter().copied();
        let windows = CircularWindows::<4, _>::new(iter);
        let collected = windows.collect::<Vec<_>>();

        assert_eq!(vec![[b'1', b'2', b'3', 0],], collected);
    }

    #[test]
    fn equals_n() {
        let iter = b"1234".iter().copied();
        let windows = CircularWindows::<4, _>::new(iter);
        let collected = windows.collect::<Vec<_>>();

        assert_eq!(vec![[b'1', b'2', b'3', b'4']], collected);
    }

    #[test]
    fn greater_than_n() {
        let iter = b"12345678".iter().copied();
        let windows = CircularWindows::<4, _>::new(iter);
        let collected = windows.collect::<Vec<_>>();

        assert_eq!(
            vec![
                [b'1', b'2', b'3', b'4'],
                [b'2', b'3', b'4', b'5'],
                [b'3', b'4', b'5', b'6'],
                [b'4', b'5', b'6', b'7'],
                [b'5', b'6', b'7', b'8'],
                // N times shifted
                [b'6', b'7', b'8', b'1'],
                [b'7', b'8', b'1', b'2'],
                [b'8', b'1', b'2', b'3'],
                [b'1', b'2', b'3', b'4'],
            ],
            collected
        );
    }

    #[test]
    fn less_than_n_rev() {
        let iter = b"".iter().copied();
        let windows = CircularWindowsReversed::<4, _>::new(iter);
        let collected = windows.collect::<Vec<_>>();

        assert_eq!(vec![[0, 0, 0, 0],], collected);

        let iter = b"1".iter().copied();
        let windows = CircularWindowsReversed::<4, _>::new(iter);
        let collected = windows.collect::<Vec<_>>();

        assert_eq!(vec![[b'1', 0, 0, 0],], collected);

        let iter = b"12".iter().copied();
        let windows = CircularWindowsReversed::<4, _>::new(iter);
        let collected = windows.collect::<Vec<_>>();

        assert_eq!(vec![[b'1', b'2', 0, 0],], collected);

        let iter = b"123".iter().copied();
        let windows = CircularWindowsReversed::<4, _>::new(iter);
        let collected = windows.collect::<Vec<_>>();

        assert_eq!(vec![[b'1', b'2', b'3', 0],], collected);
    }

    #[test]
    fn equals_n_rev() {
        let iter = b"1234".iter().copied();
        let windows = CircularWindowsReversed::<4, _>::new(iter);
        let collected = windows.collect::<Vec<_>>();

        assert_eq!(vec![[b'1', b'2', b'3', b'4']], collected);
    }

    #[test]
    fn greater_than_n_rev() {
        let iter = b"12345678".iter().copied();
        let windows = CircularWindowsReversed::<4, _>::new(iter);
        let collected = windows.collect::<Vec<_>>();

        assert_eq!(
            vec![
                [b'1', b'2', b'3', b'4'],
                [b'8', b'1', b'2', b'3'],
                [b'7', b'8', b'1', b'2'],
                [b'6', b'7', b'8', b'1'],
                [b'5', b'6', b'7', b'8'],
                // N times shifted
                [b'4', b'5', b'6', b'7'],
                [b'3', b'4', b'5', b'6'],
                [b'2', b'3', b'4', b'5'],
                [b'1', b'2', b'3', b'4'],
            ],
            collected
        );
    }
}
