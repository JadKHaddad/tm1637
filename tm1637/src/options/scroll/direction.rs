/// Direction for scrolling bytes.
#[derive(Debug, Default, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ScrollDirection {
    /// Move bytes from left to right.
    #[default]
    LeftToRight,
    /// Move bytes from right to left.
    RightToLeft,
}
