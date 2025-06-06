//! Brightness level for the `TM1637` device.

/// The brightness level.
///
/// Represents a byte that can be sent directly (as a cmd) to the `TM1637` to set the brightness level.
///
/// # Bits
///
/// - 1-3: Brightness level (0-7)
/// - 4: Display state (0 - off, 1 - on)
/// - 5-7: Base address
#[repr(u8)]
#[derive(Debug, Default, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Brightness {
    /// Display off.
    Off = 0b10000000,
    /// Brightness level 0. Lowest brightness.
    #[default]
    L0 = 0b10001000,
    /// Brightness level 1.
    L1 = 0b10001001,
    /// Brightness level 2.
    L2 = 0b10001010,
    /// Brightness level 3.
    L3 = 0b10001011,
    /// Brightness level 4.
    L4 = 0b10001100,
    /// Brightness level 5.
    L5 = 0b10001101,
    /// Brightness level 6.
    L6 = 0b10001110,
    /// Brightness level 7. Highest brightness.
    L7 = 0b10001111,
}
