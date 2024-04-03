//! Blocking functionality.

use crate::device::brightness::Brightness;

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
    fn send_byte(&mut self, byte: u8) -> Result<(), TM1637Error<ERR>> {
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
    /// Clear the display.
    fn clear(&mut self) -> Result<(), TM1637Error<ERR>> {
        self.write_raw_iter(0, core::iter::repeat(0).take(self.address_count() as usize))
    }

    /// Write the given bytes to the display starting from the given address. See [`BlockingTM1637::write_raw_iter`].
    ///
    /// ## Notes:
    /// - Addresses greater than [`BaseTM1637::address_count`] will be ignored.
    /// - Bytes with index greater than [`BaseTM1637::address_count`] will be ignored.
    fn write_raw(&mut self, address: u8, bytes: &[u8]) -> Result<(), TM1637Error<ERR>> {
        self.write_raw_iter(address, bytes.iter().map(|b| *b))
    }

    /// Write the given bytes to the display starting from the given address. See [`BlockingTM1637::write_raw`].
    ///
    /// ## Notes:
    /// - Addresses greater than [`BaseTM1637::address_count`] will be ignored.
    /// - Bytes with index greater than [`BaseTM1637::address_count`] will be ignored.
    fn write_raw_iter<ITER: Iterator<Item = u8>>(
        &mut self,
        address: u8,
        bytes: ITER,
    ) -> Result<(), TM1637Error<ERR>> {
        #[cfg(not(feature = "disable-checks"))]
        if address >= self.address_count() {
            return Ok(());
        }

        // COMM1
        tri_digital!(self.start());
        tri!(self.send_byte(0x40));
        tri_digital!(self.stop());

        // COMM2
        tri_digital!(self.start());
        tri!(self.send_byte(0xc0 | (address & 0x03)));

        #[cfg(not(feature = "disable-checks"))]
        let bytes = bytes.take(self.address_count() as usize - address as usize);

        for byte in bytes {
            tri!(self.send_byte(byte));
        }

        tri_digital!(self.stop());

        // COMM3
        // tri_digital!(self.start());
        // tri!(self.send_byte(Brightness::L0 as u8));
        // tri_digital!(self.stop());

        Ok(())
    }

    /// Set the brightness level.
    fn set_brightness(&mut self, brightness: Brightness) -> Result<(), TM1637Error<ERR>> {
        tri_digital!(self.start());
        tri!(self.send_byte(brightness as u8));
        tri_digital!(self.stop());

        Ok(())
    }
}
