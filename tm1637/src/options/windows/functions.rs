use crate::options::scroll::{ScrollDirection, ScrollStyle};

use super::{CircularWindows, CircularWindowsReversed, LinearWindows};

/// Returns an iterator over windows of size `N` from the input bytes.
#[auto_enums::auto_enum(Iterator)]
pub fn windows<const N: usize>(
    bytes: impl DoubleEndedIterator<Item = u8>,
    direction: ScrollDirection,
    style: ScrollStyle,
) -> impl Iterator<Item = [u8; N]> {
    match style {
        ScrollStyle::Circular => windows_circular::<N>(bytes, direction),
        ScrollStyle::Linear => windows_linear::<N>(bytes, direction),
    }
}

/// Returns an iterator over circular windows of size `N` from the input bytes.
///
/// See [`windows`].
#[auto_enums::auto_enum(Iterator)]
pub fn windows_circular<const N: usize>(
    bytes: impl DoubleEndedIterator<Item = u8>,
    direction: ScrollDirection,
) -> impl Iterator<Item = [u8; N]> {
    match direction {
        ScrollDirection::LeftToRight => CircularWindows::<N, _>::new(bytes),
        ScrollDirection::RightToLeft => CircularWindowsReversed::<N, _>::new(bytes),
    }
}

/// Returns an iterator over linear windows of size `N` from the input bytes.
///
/// See [`windows`].
#[auto_enums::auto_enum(Iterator)]
pub fn windows_linear<const N: usize>(
    bytes: impl DoubleEndedIterator<Item = u8>,
    direction: ScrollDirection,
) -> impl Iterator<Item = [u8; N]> {
    match direction {
        ScrollDirection::LeftToRight => LinearWindows::<N, _>::new(bytes),
        ScrollDirection::RightToLeft => LinearWindows::<N, _>::new(bytes).rev(),
    }
}
