use crate::Windows;

#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
enum WindowsState<const N: usize> {
    Init,
    First([u8; N]),
    FirstAndLast([u8; N], [u8; N]),
}

#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct CircularWindows<const N: usize, I> {
    state: WindowsState<N>,
    iter: Windows<N, I>,
    counter: usize,
}

impl<const N: usize, I> CircularWindows<N, I> {
    pub const fn new(iter: I) -> Self {
        Self {
            state: WindowsState::<N>::Init,
            iter: Windows::<N, I>::new(iter),
            counter: 0,
        }
    }
}

fn combine<const N: usize>(first: [u8; N], last: [u8; N], index: usize) -> [u8; N] {
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
                WindowsState::First(first_item) => Some(first_item),
                WindowsState::FirstAndLast(first_item, last_item) => {
                    if self.counter < N {
                        let item = combine(first_item, last_item, self.counter);

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
    fn test() {
        let iter = b"lorem".iter().copied();
        let windows = CircularWindows::<3, _>::new(iter);

        for item in windows {
            std::println!(
                "{:?}",
                item.into_iter().map(|b| b as char).collect::<Vec<_>>()
            );
        }
    }

    #[test]
    fn combine_test() {
        let first = [b'l', b'o', b'r'];
        let last = [b'r', b'e', b'm'];

        assert_eq!([b'e', b'm', b'l'], combine(first, last, 0));
    }
}
