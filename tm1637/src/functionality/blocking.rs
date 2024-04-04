//! Blocking functionality.

use crate::device::brightness::{Brightness, DisplayState};

use super::BaseTM1637;

use embedded_hal::{
    delay::DelayNs,
    digital::{InputPin, OutputPin},
};

/// Private blocking functionality.
pub(crate) trait PrivateBlockingTM1637<CLK, DIO, DELAY, ERR>:
    BaseTM1637<CLK, DIO, DELAY>
where
    CLK: OutputPin<Error = ERR>,
    DIO: InputPin<Error = ERR> + OutputPin<Error = ERR>,
    DELAY: DelayNs,
{
    /// Send a byte to the display and wait for the ACK.
    fn write_byte(&mut self, byte: u8) -> Result<(), ERR> {
        let mut rest = byte;

        for _ in 0..8 {
            tri!(self.clk_mut().set_low());

            match rest & 0x01 {
                1 => tri!(self.dio_mut().set_high()),
                _ => tri!(self.dio_mut().set_low()),
            }

            tri!(self.clk_mut().set_high());
            self.bit_delay();

            rest >>= 1;
        }

        tri!(self.clk_mut().set_low());
        tri!(self.dio_mut().set_high());
        tri!(self.clk_mut().set_high());
        self.bit_delay();

        tri!(self.clk_mut().set_low());

        Ok(())
    }

    /// Write the `cmd` to the display.
    fn write_cmd_raw(&mut self, cmd: u8) -> Result<(), ERR> {
        tri!(self.start());
        tri!(self.write_byte(cmd));
        tri!(self.stop());

        Ok(())
    }

    /// Start the communication with the display.
    fn start(&mut self) -> Result<(), ERR> {
        tri!(self.dio_mut().set_low());
        self.bit_delay();

        Ok(())
    }

    /// Stop the communication with the display.
    fn stop(&mut self) -> Result<(), ERR> {
        tri!(self.dio_mut().set_low());
        tri!(self.clk_mut().set_high());
        self.bit_delay();
        tri!(self.dio_mut().set_high());
        self.bit_delay();

        Ok(())
    }

    /// Delay for [`BaseTM1637::delay_us()`] microseconds using [`BaseTM1637::delay()`] provider.
    fn bit_delay(&mut self) {
        let delay_us = self.delay_us();
        self.delay_mut().delay_us(delay_us);
    }
}

/// Blocking functionality.
///
/// Bring this trait into scope to enable blocking functionality for `TM1637` devices.
#[allow(private_bounds)]
#[allow(async_fn_in_trait)]
pub trait BlockingTM1637<CLK, DIO, DELAY, ERR>:
    PrivateBlockingTM1637<CLK, DIO, DELAY, ERR>
where
    CLK: OutputPin<Error = ERR>,
    DIO: InputPin<Error = ERR> + OutputPin<Error = ERR>,
    DELAY: DelayNs,
{
    /// Initialize the display.
    ///
    /// Clear the display and set the brightness level.
    fn init(&mut self) -> Result<(), ERR> {
        tri!(self.clear());
        self.write_cmd_raw(self.brightness() as u8)
    }

    /// Turn the display on.
    fn on(&mut self) -> Result<(), ERR> {
        self.write_cmd_raw(self.brightness() as u8)
    }

    /// Turn the display off.
    fn off(&mut self) -> Result<(), ERR> {
        self.write_cmd_raw(DisplayState::OFF as u8)
    }

    /// Clear the display.
    fn clear(&mut self) -> Result<(), ERR> {
        self.write_segments_raw_iter(0, core::iter::repeat(0).take(self.num_positions() as usize))
    }

    /// Write the given bytes to the display starting from the given position.
    ///
    /// See [`BlockingTM1637::write_segments_raw_iter`].
    fn write_segments_raw(&mut self, position: u8, bytes: &[u8]) -> Result<(), ERR> {
        self.write_segments_raw_iter(position, bytes.iter().map(|b| *b))
    }

    /// Write the given bytes to the display starting from the given position.
    ///
    /// ## Notes:
    /// - Positions greater than [`BaseTM1637::num_positions`] will be ignored.
    /// - Bytes with index greater than [`BaseTM1637::num_positions`] will be ignored.
    ///
    /// Brightness level will not be written to the device on each call. Make sure to call [`BlockingTM1637::write_brightness`] or [`BlockingTM1637::init`] to set the brightness level.
    fn write_segments_raw_iter<ITER: Iterator<Item = u8>>(
        &mut self,
        position: u8,
        bytes: ITER,
    ) -> Result<(), ERR> {
        #[cfg(not(feature = "disable-checks"))]
        if position >= self.num_positions() {
            return Ok(());
        }

        // COMM1
        tri!(self.write_cmd_raw(0x40));

        // COMM2
        tri!(self.start());
        tri!(self.write_byte(0xc0 | (position & 0x03)));

        #[cfg(not(feature = "disable-checks"))]
        let bytes = bytes.take(self.num_positions() as usize - position as usize);

        for byte in bytes {
            tri!(self.write_byte(byte));
        }

        tri!(self.stop());

        Ok(())
    }

    /// Set `brightness` in `Self` and write the brightness level to the display.
    fn write_brightness(&mut self, brightness: Brightness) -> Result<(), ERR> {
        *self.brightness_mut() = brightness;
        self.write_cmd_raw(brightness as u8)
    }
}
