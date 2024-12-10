/// Internal state for [`LinearWindows`].
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
enum BufferState {
    Filling(usize),
    Full,
}

/// Linear windows iterator.
///
/// - Represents [`ScrollStyle::Linear`](crate::options::ScrollStyle::Linear) and [`ScrollDirection::LeftToRight`](crate::options::ScrollDirection::LeftToRight).
/// - Reversed, represents [`ScrollStyle::Linear`](crate::options::ScrollStyle::Linear) and [`ScrollDirection::RightToLeft`](crate::options::ScrollDirection::RightToLeft).
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct LinearWindows<const N: usize, I> {
    buffer: [u8; N],
    state: BufferState,
    iter: I,
}

impl<const N: usize, I> LinearWindows<N, I> {
    pub const fn new(iter: I) -> Self {
        Self {
            buffer: [0; N],
            state: BufferState::Filling(0),
            iter,
        }
    }
}

// Linear left to right.
impl<const N: usize, I> Iterator for LinearWindows<N, I>
where
    I: Iterator<Item = u8>,
{
    type Item = [u8; N];

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.iter.next() {
                Some(byte) => {
                    match self.state {
                        BufferState::Filling(counter) => {
                            self.buffer[counter] = byte;

                            let counter = counter + 1;

                            if counter == N {
                                self.state = BufferState::Full;

                                return Some(self.buffer);
                            }

                            self.state = BufferState::Filling(counter);
                        }
                        BufferState::Full => {
                            // At this point, we shift everything to the left making space for the new byte an N - 1
                            // and then we add the new byte at the end.

                            // TODO: Optimize this.
                            for i in 0..N - 1 {
                                self.buffer[i] = self.buffer[i + 1];
                            }

                            self.buffer[N - 1] = byte;

                            return Some(self.buffer);
                        }
                    }
                }
                None => {
                    if matches!(self.state, BufferState::Filling(_)) {
                        self.state = BufferState::Full;

                        return Some(self.buffer);
                    }

                    return None;
                }
            }
        }
    }
}

// Linear right to left.
impl<const N: usize, I> DoubleEndedIterator for LinearWindows<N, I>
where
    I: DoubleEndedIterator<Item = u8>,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        loop {
            match self.iter.next_back() {
                Some(byte) => match self.state {
                    BufferState::Filling(counter) => {
                        for i in (1..=counter).rev() {
                            self.buffer[i] = self.buffer[i - 1];
                        }

                        self.buffer[0] = byte;

                        let counter = counter + 1;

                        if counter == N {
                            self.state = BufferState::Full;

                            return Some(self.buffer);
                        }

                        self.state = BufferState::Filling(counter);
                    }
                    BufferState::Full => {
                        for i in (1..N).rev() {
                            self.buffer[i] = self.buffer[i - 1];
                        }

                        self.buffer[0] = byte;

                        return Some(self.buffer);
                    }
                },
                None => {
                    if matches!(self.state, BufferState::Filling(_)) {
                        self.state = BufferState::Full;

                        return Some(self.buffer);
                    }

                    return None;
                }
            }
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
    fn less_than_n() {
        let iter = b"".iter().copied();
        let windows = LinearWindows::<4, _>::new(iter);
        let collected = windows.collect::<Vec<_>>();

        assert_eq!(vec![[0, 0, 0, 0],], collected);

        let iter = b"1".iter().copied();
        let windows = LinearWindows::<4, _>::new(iter);
        let collected = windows.collect::<Vec<_>>();

        assert_eq!(vec![[b'1', 0, 0, 0],], collected);

        let iter = b"12".iter().copied();
        let windows = LinearWindows::<4, _>::new(iter);
        let collected = windows.collect::<Vec<_>>();

        assert_eq!(vec![[b'1', b'2', 0, 0],], collected);

        let iter = b"123".iter().copied();
        let windows = LinearWindows::<4, _>::new(iter);
        let collected = windows.collect::<Vec<_>>();

        assert_eq!(vec![[b'1', b'2', b'3', 0],], collected);
    }

    #[test]
    fn equals_n() {
        let iter = b"1234".iter().copied();
        let windows = LinearWindows::<4, _>::new(iter);
        let collected = windows.collect::<Vec<_>>();

        assert_eq!(vec![[b'1', b'2', b'3', b'4']], collected);
    }

    #[test]
    fn greater_than_n() {
        let iter = b"123456".iter().copied();
        let windows = LinearWindows::<4, _>::new(iter);
        let collected = windows.collect::<Vec<_>>();

        assert_eq!(
            vec![
                [b'1', b'2', b'3', b'4'],
                [b'2', b'3', b'4', b'5'],
                [b'3', b'4', b'5', b'6']
            ],
            collected
        );
    }

    #[test]
    fn less_than_n_rev() {
        let iter = b"".iter().copied();
        let windows = LinearWindows::<4, _>::new(iter);
        let collected = windows.rev().collect::<Vec<_>>();

        assert_eq!(vec![[0, 0, 0, 0],], collected);

        let iter = b"1".iter().copied();
        let windows = LinearWindows::<4, _>::new(iter);
        let collected = windows.rev().collect::<Vec<_>>();

        assert_eq!(vec![[b'1', 0, 0, 0],], collected);

        let iter = b"12".iter().copied();
        let windows = LinearWindows::<4, _>::new(iter);
        let collected = windows.rev().collect::<Vec<_>>();

        assert_eq!(vec![[b'1', b'2', 0, 0],], collected);

        let iter = b"123".iter().copied();
        let windows = LinearWindows::<4, _>::new(iter);
        let collected = windows.rev().collect::<Vec<_>>();

        assert_eq!(vec![[b'1', b'2', b'3', 0],], collected);
    }

    #[test]
    fn equals_n_rev() {
        let iter = b"1234".iter().copied();
        let windows = LinearWindows::<4, _>::new(iter);
        let collected = windows.rev().collect::<Vec<_>>();

        assert_eq!(vec![[b'1', b'2', b'3', b'4']], collected);
    }

    #[test]
    fn greater_than_n_rev() {
        let iter = b"123456".iter().copied();
        let windows = LinearWindows::<4, _>::new(iter);
        let collected = windows.rev().collect::<Vec<_>>();

        let mut expected = vec![
            [b'1', b'2', b'3', b'4'],
            [b'2', b'3', b'4', b'5'],
            [b'3', b'4', b'5', b'6'],
        ];

        expected.reverse();

        assert_eq!(expected, collected);
    }
}
