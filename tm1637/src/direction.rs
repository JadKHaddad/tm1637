/// Direction for moving bytes.
#[derive(Debug, Default, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Direction {
    /// Move bytes from left to right.
    #[default]
    LeftToRight,
    /// Move bytes from right to left.
    RightToLeft,
}
