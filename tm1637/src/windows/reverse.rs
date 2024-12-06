/// Internal state for [`ReverseWindows`].
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
enum BufferState {
    Filling(usize),
    Interrupted,
    Full,
}

/// Core for [`CircularWindowsReversed`](super::CircularWindowsReversed).
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ReverseWindows<const N: usize, I> {
    buffer: [u8; N],
    state: BufferState,
    iter: I,
}

impl<const N: usize, I> ReverseWindows<N, I> {
    pub const fn new(iter: I) -> Self {
        Self {
            buffer: [0; N],
            state: BufferState::Filling(0),
            iter,
        }
    }
}

impl<const N: usize, I> Iterator for ReverseWindows<N, I>
where
    I: DoubleEndedIterator<Item = u8>,
{
    type Item = [u8; N];

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.state {
                BufferState::Filling(counter) => match self.iter.next() {
                    Some(byte) => {
                        self.buffer[counter] = byte;

                        let counter = counter + 1;

                        if counter == N {
                            self.state = BufferState::Full;

                            return Some(self.buffer);
                        }

                        self.state = BufferState::Filling(counter);
                    }
                    None => {
                        self.state = BufferState::Interrupted;

                        return Some(self.buffer);
                    }
                },
                BufferState::Full => match self.iter.next_back() {
                    Some(byte) => {
                        for i in (1..N).rev() {
                            self.buffer[i] = self.buffer[i - 1];
                        }

                        self.buffer[0] = byte;

                        return Some(self.buffer);
                    }
                    None => return None,
                },
                BufferState::Interrupted => return None,
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
        let windows = ReverseWindows::<4, _>::new(iter);
        let collected = windows.collect::<Vec<_>>();

        assert_eq!(vec![[0, 0, 0, 0],], collected);

        let iter = b"1".iter().copied();
        let windows = ReverseWindows::<4, _>::new(iter);
        let collected = windows.collect::<Vec<_>>();

        assert_eq!(vec![[b'1', 0, 0, 0],], collected);

        let iter = b"12".iter().copied();
        let windows = ReverseWindows::<4, _>::new(iter);
        let collected = windows.collect::<Vec<_>>();

        assert_eq!(vec![[b'1', b'2', 0, 0],], collected);

        let iter = b"123".iter().copied();
        let windows = ReverseWindows::<4, _>::new(iter);
        let collected = windows.collect::<Vec<_>>();

        assert_eq!(vec![[b'1', b'2', b'3', 0],], collected);
    }

    #[test]
    fn equals_n() {
        let iter = b"1234".iter().copied();
        let windows = ReverseWindows::<4, _>::new(iter);
        let collected = windows.collect::<Vec<_>>();

        assert_eq!(vec![[b'1', b'2', b'3', b'4']], collected);
    }

    #[test]
    fn greater_than_n() {
        let iter = b"lorem".iter().copied();
        let windows = ReverseWindows::<3, _>::new(iter);
        let collected = windows.collect::<Vec<_>>();

        assert_eq!(
            vec![[b'l', b'o', b'r'], [b'm', b'l', b'o'], [b'e', b'm', b'l'],],
            collected
        );
    }
}
