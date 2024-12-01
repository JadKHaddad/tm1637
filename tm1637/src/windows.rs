#[derive(Debug)]
enum BufferState {
    Filling(usize),
    Full,
}

// None overlapping left to right windows.
#[derive(Debug)]
pub struct Windows<const N: usize, I> {
    buffer: [u8; N],
    state: BufferState,
    iter: I,
}

impl<const N: usize, I> Windows<N, I> {
    pub const fn new(iter: I) -> Self {
        Self {
            buffer: [0; N],
            state: BufferState::Filling(0),
            iter,
        }
    }
}

impl<const N: usize, I> Iterator for Windows<N, I>
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

#[cfg(test)]
mod test {
    extern crate std;
    use std::vec;
    use std::vec::Vec;

    use super::*;

    #[test]
    fn less_than_n() {
        let iter = b"123".iter().copied();
        let windows = Windows::<4, _>::new(iter);

        let collected = windows.collect::<Vec<_>>();
        assert_eq!(vec![[b'1', b'2', b'3', 0],], collected);
    }

    #[test]
    fn equals_n() {
        let iter = b"1234".iter().copied();
        let windows = Windows::<4, _>::new(iter);

        let collected = windows.collect::<Vec<_>>();
        assert_eq!(vec![[b'1', b'2', b'3', b'4']], collected);
    }

    #[test]
    fn greater_than_n() {
        let iter = b"123456".iter().copied();
        let windows = Windows::<4, _>::new(iter);

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
}
