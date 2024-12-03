/// Style for scrolling bytes.
#[derive(Debug, Default, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ScrollStyle {
    #[default]
    /// Bytes are moved in circular overlapping windows.
    ///
    /// # Example
    ///
    /// The display has 4 digits and the bytes are `HELLO `, the display will show:
    ///
    /// ```text
    /// +---+ +---+ +---+ +---+
    /// | H | | E | | L | | L |
    /// +---+ +---+ +---+ +---+
    ///
    /// +---+ +---+ +---+ +---+
    /// | E | | L | | L | | O |
    /// +---+ +---+ +---+ +---+
    ///
    /// +---+ +---+ +---+ +---+
    /// | L | | L | | O | |   |
    /// +---+ +---+ +---+ +---+
    ///
    /// +---+ +---+ +---+ +---+
    /// | L | | O | |   | | H |
    /// +---+ +---+ +---+ +---+
    ///
    /// +---+ +---+ +---+ +---+
    /// | O | |   | | H | | E |
    /// +---+ +---+ +---+ +---+
    ///
    /// +---+ +---+ +---+ +---+
    /// |   | | H | | E | | L |
    /// +---+ +---+ +---+ +---+
    ///
    /// +---+ +---+ +---+ +---+
    /// | H | | E | | L | | L |
    /// +---+ +---+ +---+ +---+
    /// ```
    Circular,
    /// Bytes are moved in windows.
    ///
    /// # Example
    ///
    /// The display has 4 digits and the bytes are `HELLO `, the display will show:
    ///
    /// ```text
    /// +---+ +---+ +---+ +---+
    /// | H | | E | | L | | L |
    /// +---+ +---+ +---+ +---+
    ///
    /// +---+ +---+ +---+ +---+
    /// | E | | L | | L | | O |
    /// +---+ +---+ +---+ +---+
    ///
    /// +---+ +---+ +---+ +---+
    /// | L | | L | | O | |   |
    /// +---+ +---+ +---+ +---+
    /// ```
    Linear,
}
