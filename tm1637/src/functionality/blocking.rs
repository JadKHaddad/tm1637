//! Blocking functionality.

use crate::device::brightness::{Brightness, DisplayState};

use super::{BaseTM1637, TM1637Error};

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
    fn write_byte(&mut self, byte: u8) -> Result<(), TM1637Error<ERR>> {
        let mut rest = byte;

        for _ in 0..8 {
            self.bit_delay();

            tri_digital!(self.clk_mut().set_low());

            self.bit_delay();

            match rest & 0x01 {
                1 => tri_digital!(self.dio_mut().set_high()),
                _ => tri_digital!(self.dio_mut().set_low()),
            }
            self.bit_delay();

            tri_digital!(self.clk_mut().set_high());

            self.bit_delay();

            rest >>= 1;
        }

        tri_digital!(self.clk_mut().set_low());
        tri_digital!(self.dio_mut().set_high());

        self.bit_delay();

        tri_digital!(self.clk_mut().set_high());

        self.bit_delay();

        tri_digital!(self.clk_mut().set_low());

        self.bit_delay();

        Ok(())
    }

    /// Set the brightness level.
    fn write_brightness_raw(&mut self, brightness: u8) -> Result<(), TM1637Error<ERR>> {
        tri_digital!(self.start());
        tri!(self.write_byte(brightness));
        tri_digital!(self.stop());

        Ok(())
    }

    /// Start the communication with the display.
    fn start(&mut self) -> Result<(), ERR> {
        tri!(self.dio_mut().set_low());
        self.bit_delay();
        tri!(self.clk_mut().set_low());
        self.bit_delay();

        Ok(())
    }

    /// Stop the communication with the display.
    fn stop(&mut self) -> Result<(), ERR> {
        tri!(self.dio_mut().set_low());
        self.bit_delay();
        tri!(self.clk_mut().set_high());
        self.bit_delay();
        tri!(self.dio_mut().set_high());
        self.bit_delay();

        Ok(())
    }

    /// Delay for the given amount of microseconds with the delay provider.
    fn bit_delay(&mut self) {
        let delay_us = self.delay_us();
        self.delay_mut().delay_us(delay_us);
    }
}

/// Blocking functionality.
///
/// Bring this trait into scope to enable blocking functionality for TM1637 devices.
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
    fn init(&mut self) -> Result<(), TM1637Error<ERR>> {
        self.clear()?;
        self.write_brightness_raw(self.brightness() as u8)
    }

    /// Turn the display on.
    fn on(&mut self) -> Result<(), TM1637Error<ERR>> {
        self.write_brightness_raw(self.brightness() as u8)
    }

    /// Turn the display off.
    fn off(&mut self) -> Result<(), TM1637Error<ERR>> {
        self.write_brightness_raw(DisplayState::OFF as u8)
    }

    /// Clear the display.
    fn clear(&mut self) -> Result<(), TM1637Error<ERR>> {
        self.write_segments_raw_iter(0, core::iter::repeat(0).take(self.num_positions() as usize))
    }

    /// Write the given bytes to the display starting from the given position.
    ///
    /// See [`BlockingTM1637::write_segments_raw_iter`].
    fn write_segments_raw(&mut self, address: u8, bytes: &[u8]) -> Result<(), TM1637Error<ERR>> {
        self.write_segments_raw_iter(address, bytes.iter().map(|b| *b))
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
    ) -> Result<(), TM1637Error<ERR>> {
        #[cfg(not(feature = "disable-checks"))]
        if position >= self.num_positions() {
            return Ok(());
        }

        // COMM1
        tri_digital!(self.start());
        tri!(self.write_byte(0x40));
        tri_digital!(self.stop());

        // COMM2
        tri_digital!(self.start());
        tri!(self.write_byte(0xc0 | (position & 0x03)));

        #[cfg(not(feature = "disable-checks"))]
        let bytes = bytes.take(self.num_positions() as usize - position as usize);

        for byte in bytes {
            tri!(self.write_byte(byte));
        }

        tri_digital!(self.stop());

        Ok(())
    }

    /// Set `brightness` in `Self` and write the brightness level.
    fn write_brightness(&mut self, brightness: Brightness) -> Result<(), TM1637Error<ERR>> {
        *self.brightness_mut() = brightness;
        self.write_brightness_raw(brightness as u8)
    }
}
