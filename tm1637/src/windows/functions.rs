use crate::scroll::{ScrollDirection, ScrollStyle};

use super::{CircularWindows, CircularWindowsReversed, LinearWindows};

#[auto_enums::auto_enum(Iterator)]
pub fn windows_iter<const N: usize>(
    iter: impl DoubleEndedIterator<Item = u8>,
    direction: ScrollDirection,
    style: ScrollStyle,
) -> impl Iterator<Item = [u8; N]> {
    match style {
        ScrollStyle::Circular => windows_circular_iter::<N>(iter, direction),
        ScrollStyle::Linear => windows_linear_iter::<N>(iter, direction),
    }
}

#[auto_enums::auto_enum(Iterator)]
pub fn windows_circular_iter<const N: usize>(
    iter: impl DoubleEndedIterator<Item = u8>,
    direction: ScrollDirection,
) -> impl Iterator<Item = [u8; N]> {
    match direction {
        ScrollDirection::LeftToRight => CircularWindows::<N, _>::new(iter),
        ScrollDirection::RightToLeft => CircularWindowsReversed::<N, _>::new(iter),
    }
}

#[auto_enums::auto_enum(Iterator)]
pub fn windows_linear_iter<const N: usize>(
    iter: impl DoubleEndedIterator<Item = u8>,
    direction: ScrollDirection,
) -> impl Iterator<Item = [u8; N]> {
    match direction {
        ScrollDirection::LeftToRight => LinearWindows::<N, _>::new(iter),
        ScrollDirection::RightToLeft => LinearWindows::<N, _>::new(iter).rev(),
    }
}

pub fn windows<const N: usize>(
    bytes: &[u8],
    direction: ScrollDirection,
    style: ScrollStyle,
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
        ScrollStyle::Circular => Outer::A(
            windows_circular::<N>(bytes, direction)
                .map(|w| w.into_iter())
                .map(Inner::A),
        ),
        ScrollStyle::Linear => Outer::B(
            windows_linear::<N>(bytes, direction)
                .map(|w| w.iter().copied())
                .map(Inner::B),
        ),
    }
}

pub fn windows_circular<const N: usize>(
    bytes: &[u8],
    direction: ScrollDirection,
) -> impl Iterator<Item = [u8; N]> + '_ {
    (0..=bytes.len()).map(move |i| {
        let mut window = [0u8; N];

        for j in 0..N {
            window[j] = match direction {
                ScrollDirection::LeftToRight => bytes[(i + j) % bytes.len()],
                ScrollDirection::RightToLeft => bytes[(bytes.len() - i + j) % bytes.len()],
            };
        }

        window
    })
}

#[auto_enums::auto_enum(Iterator)]
pub fn windows_linear<const N: usize>(
    bytes: &[u8],
    direction: ScrollDirection,
) -> impl Iterator<Item = &[u8]> + '_ {
    match direction {
        ScrollDirection::LeftToRight => bytes.windows(N),
        ScrollDirection::RightToLeft => bytes.windows(N).rev(),
    }
}

#[cfg(test)]
mod tests {
    extern crate std;
    use std::vec;
    use std::vec::Vec;

    use super::*;

    #[test]
    fn windows_circular_left_to_right() {
        let slice = b"lorem";
        let iter = windows_circular::<3>(slice, ScrollDirection::LeftToRight);
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
    fn windows_circular_right_to_left() {
        let slice = b"lorem";
        let iter = windows_circular::<3>(slice, ScrollDirection::RightToLeft);
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
    fn windows_linear_left_to_right() {
        let slice = b"lorem";
        let iter = windows_linear::<3>(slice, ScrollDirection::LeftToRight);
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
    fn windows_linear_right_to_left() {
        let slice = b"lorem";
        let iter = windows_linear::<3>(slice, ScrollDirection::RightToLeft);
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
