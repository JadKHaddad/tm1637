//! Device functionality in async and blocking modes.

use crate::device::brightness::Brightness;

#[cfg(feature = "async")]
pub mod asynchronous;
#[cfg(feature = "blocking")]
pub mod blocking;

/// Private main functionality for `TM1637` devices.
pub(crate) trait BaseTM1637<CLK, DIO, DELAY> {
    fn clk(&self) -> &CLK;

    fn clk_mut(&mut self) -> &mut CLK;

    fn dio(&self) -> &DIO;

    fn dio_mut(&mut self) -> &mut DIO;

    fn delay(&self) -> &DELAY;

    fn delay_mut(&mut self) -> &mut DELAY;

    fn brightness(&self) -> Brightness;

    fn brightness_mut(&mut self) -> &mut Brightness;

    fn delay_us(&self) -> u32;

    fn num_positions(&self) -> u8;
}
