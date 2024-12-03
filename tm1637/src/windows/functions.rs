use crate::{Direction, WindowsStyle};

use super::{CircularWindows, CircularWindowsReversed, LinearWindows};

#[auto_enums::auto_enum(Iterator)]
pub fn windows_new_api<const N: usize>(
    iter: impl DoubleEndedIterator<Item = u8>,
    direction: Direction,
    style: WindowsStyle,
) -> impl Iterator<Item = [u8; N]> {
    match style {
        WindowsStyle::Circular => windows_circular::<N>(iter, direction),
        WindowsStyle::Linear => windows_linear::<N>(iter, direction),
    }
}

#[auto_enums::auto_enum(Iterator)]
pub fn windows_circular<const N: usize>(
    iter: impl DoubleEndedIterator<Item = u8>,
    direction: Direction,
) -> impl Iterator<Item = [u8; N]> {
    match direction {
        Direction::LeftToRight => CircularWindows::<N, _>::new(iter),
        Direction::RightToLeft => CircularWindowsReversed::<N, _>::new(iter),
    }
}

#[auto_enums::auto_enum(Iterator)]
pub fn windows_linear<const N: usize>(
    iter: impl DoubleEndedIterator<Item = u8>,
    direction: Direction,
) -> impl Iterator<Item = [u8; N]> {
    match direction {
        Direction::LeftToRight => LinearWindows::<N, _>::new(iter),
        Direction::RightToLeft => LinearWindows::<N, _>::new(iter).rev(),
    }
}

// Old api keep, it's more performant
pub fn windows<const N: usize>(
    bytes: &[u8],
    direction: Direction,
    style: WindowsStyle,
) -> impl Iterator<Item = impl Iterator<Item = u8> + '_> + '_ {
    #[auto_enums::enum_derive(Iterator)]
    enum Outer<A, B> {
        A(A),
        B(B),
    }

    #[auto_enums::enum_derive(Iterator)]
    enum Inner<A, B> {
        A(A),
        B(B),
    }

    match style {
        WindowsStyle::Circular => Outer::A(
            windows_overlapping::<N>(bytes, direction)
                .map(|w| w.into_iter())
                .map(Inner::A),
        ),
        WindowsStyle::Linear => Outer::B(
            windows_non_overlapping::<N>(bytes, direction)
                .map(|w| w.iter().copied())
                .map(Inner::B),
        ),
    }
}

// Old api keep, it's more performant
pub fn windows_overlapping<const N: usize>(
    bytes: &[u8],
    direction: Direction,
) -> impl Iterator<Item = [u8; N]> + '_ {
    (0..=bytes.len()).map(move |i| {
        let mut window = [0u8; N];

        for j in 0..N {
            window[j] = match direction {
                Direction::LeftToRight => bytes[(i + j) % bytes.len()],
                Direction::RightToLeft => bytes[(bytes.len() - i + j) % bytes.len()],
            };
        }

        window
    })
}

// Old api keep, it's more performant
#[auto_enums::auto_enum(Iterator)]
pub fn windows_non_overlapping<const N: usize>(
    bytes: &[u8],
    direction: Direction,
) -> impl Iterator<Item = &[u8]> + '_ {
    match direction {
        Direction::LeftToRight => bytes.windows(N),
        Direction::RightToLeft => bytes.windows(N).rev(),
    }
}

#[cfg(test)]
mod tests {
    extern crate std;
    use std::vec;
    use std::vec::Vec;

    use super::*;

    #[test]
    fn windows_overlapping_left_to_right() {
        let slice = b"lorem";
        let iter = windows_overlapping::<3>(slice, Direction::LeftToRight);
        let collected: Vec<Vec<u8>> = iter.map(|i| i.into_iter().collect()).collect();

        assert_eq!(
            collected,
            vec![
                vec![b'l', b'o', b'r'],
                vec![b'o', b'r', b'e'],
                vec![b'r', b'e', b'm'],
                vec![b'e', b'm', b'l'],
                vec![b'm', b'l', b'o'],
                vec![b'l', b'o', b'r'],
            ]
        );
    }

    #[test]
    fn windows_overlapping_right_to_left() {
        let slice = b"lorem";
        let iter = windows_overlapping::<3>(slice, Direction::RightToLeft);
        let collected: Vec<Vec<u8>> = iter.map(|i| i.into_iter().collect()).collect();

        assert_eq!(
            collected,
            vec![
                vec![b'l', b'o', b'r'],
                vec![b'm', b'l', b'o'],
                vec![b'e', b'm', b'l'],
                vec![b'r', b'e', b'm'],
                vec![b'o', b'r', b'e'],
                vec![b'l', b'o', b'r'],
            ]
        );
    }

    #[test]
    fn windows_non_overlapping_left_to_right() {
        let slice = b"lorem";
        let iter = windows_non_overlapping::<3>(slice, Direction::LeftToRight);
        let collected: Vec<Vec<u8>> = iter.map(|i| i.to_vec()).collect();

        assert_eq!(
            collected,
            vec![
                vec![b'l', b'o', b'r'],
                vec![b'o', b'r', b'e'],
                vec![b'r', b'e', b'm'],
            ]
        );
    }

    #[test]
    fn windows_non_overlapping_right_to_left() {
        let slice = b"lorem";
        let iter = windows_non_overlapping::<3>(slice, Direction::RightToLeft);
        let collected: Vec<Vec<u8>> = iter.map(|i| i.to_vec()).collect();

        assert_eq!(
            collected,
            vec![
                vec![b'r', b'e', b'm'],
                vec![b'o', b'r', b'e'],
                vec![b'l', b'o', b'r'],
            ]
        );
    }
}
