//! Device functionality in async and blocking modes.

use crate::device::brightness::Brightness;

#[cfg(feature = "async")]
pub mod asynchronous;
#[cfg(feature = "blocking")]
pub mod blocking;

/// Sealed trait for `TM1637` devices.
pub(crate) trait Sealed {}

/// Private main functionality for `TM1637` devices.
#[allow(private_bounds)]
pub trait BaseTM1637<CLK, DIO, DELAY>: Sealed {
    /// Get the clock pin.
    fn clk(&self) -> &CLK;

    /// Get the mutable clock pin.
    fn clk_mut(&mut self) -> &mut CLK;

    /// Get the data pin.
    fn dio(&self) -> &DIO;

    /// Get the mutable data pin.
    fn dio_mut(&mut self) -> &mut DIO;

    /// Get the delay provider.
    fn delay(&self) -> &DELAY;

    /// Get the mutable delay provider.
    fn delay_mut(&mut self) -> &mut DELAY;

    /// Get the brightness.
    fn brightness(&self) -> Brightness;

    /// Get the mutable brightness.
    fn brightness_mut(&mut self) -> &mut Brightness;

    /// Get the delay in microseconds.
    fn delay_us(&self) -> u32;

    /// Get the number of display positions.
    fn num_positions(&self) -> u8;
}
