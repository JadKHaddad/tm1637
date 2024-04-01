//! Asynchronous functionality.

use crate::device::brightness::Brightness;

use super::{BaseTM1637, Bit, TM1637Error};

use embedded_hal::digital::{InputPin, OutputPin};
use embedded_hal_async::delay::DelayNs;

/// Private asynchronous functionality.
pub(crate) trait PrivateAsyncTM1637<CLK, DIO, DELAY, ERR>:
    BaseTM1637<CLK, DIO, DELAY>
where
    CLK: OutputPin<Error = ERR>,
    DIO: InputPin<Error = ERR> + OutputPin<Error = ERR>,
    DELAY: DelayNs,
{
    /// Send a byte to the display and wait for the ACK.
    async fn send_byte(&mut self, byte: u8) -> Result<(), TM1637Error<ERR>> {
        let mut rest = byte;
        for _ in 0..8 {
            let bit = if rest & 1 != 0 { Bit::ONE } else { Bit::ZERO };
            tri_digital!(self.send_bit_and_delay(bit).await);
            rest = rest >> 1;
        }

        // Wait for the ACK
        tri_digital!(self.send_bit_and_delay(Bit::ONE).await);
        for _ in 0..255 {
            if tri_digital!(self.dio().is_low()) {
                return Ok(());
            }

            self.bit_delay().await;
        }

        Err(TM1637Error::Ack)
    }

    /// Start the communication with the display.
    async fn start(&mut self) -> Result<(), ERR> {
        tri!(self.send_bit_and_delay(Bit::ONE).await);
        tri!(self.dio().set_low());

        Ok(())
    }

    /// Stop the communication with the display.
    async fn stop(&mut self) -> Result<(), ERR> {
        tri!(self.send_bit_and_delay(Bit::ZERO).await);
        tri!(self.dio().set_high());
        self.bit_delay().await;

        Ok(())
    }

    /// Send a bit to the display and delay.
    async fn send_bit_and_delay(&mut self, value: Bit) -> Result<(), ERR> {
        tri!(self.clk().set_low());
        match value {
            Bit::ONE => tri!(self.dio().set_high()),
            Bit::ZERO => tri!(self.dio().set_low()),
        }
        tri!(self.clk().set_high());
        self.bit_delay().await;

        Ok(())
    }

    /// Delay for the given amount of microseconds with the delay provider.
    async fn bit_delay(&mut self) {
        let delay_us = self.delay_us();
        self.delay().delay_us(delay_us).await;
    }
}

/// Asynchronous functionality.
///
/// Bring this trait into scope to enable asynchronous functionality for TM1637 devices.
#[allow(private_bounds)]
#[allow(async_fn_in_trait)]
pub trait AsyncTM1637<CLK, DIO, DELAY, ERR>: PrivateAsyncTM1637<CLK, DIO, DELAY, ERR>
where
    CLK: OutputPin<Error = ERR>,
    DIO: InputPin<Error = ERR> + OutputPin<Error = ERR>,
    DELAY: DelayNs,
{
    /// Clear the display.
    async fn clear(&mut self) -> Result<(), TM1637Error<ERR>> {
        self.write_raw_iter(0, core::iter::repeat(0).take(self.address_count() as usize))
            .await
    }

    /// Write the given bytes to the display starting from the given address. See [`BlockingTM1637::write_raw_iter`].
    ///
    /// ## Notes:
    /// - Addresses greater than [`BaseTM1637::address_count`] will be ignored.
    /// - Bytes with index greater than [`BaseTM1637::address_count`] will be ignored.
    async fn write_raw(&mut self, address: u8, bytes: &[u8]) -> Result<(), TM1637Error<ERR>> {
        self.write_raw_iter(address, bytes.iter().map(|b| *b)).await
    }

    /// Write the given bytes to the display starting from the given address. See [`BlockingTM1637::write_raw`].
    ///
    /// ## Notes:
    /// - Addresses greater than [`BaseTM1637::address_count`] will be ignored.
    /// - Bytes with index greater than [`BaseTM1637::address_count`] will be ignored.
    async fn write_raw_iter<ITER: Iterator<Item = u8>>(
        &mut self,
        address: u8,
        bytes: ITER,
    ) -> Result<(), TM1637Error<ERR>> {
        if address >= self.address_count() {
            return Ok(());
        }

        tri_digital!(self.start().await);
        tri!(self.send_byte(0xc0 | (address & 0x0f)).await);

        let bytes_to_send = bytes.take(self.address_count() as usize - address as usize);
        for byte in bytes_to_send {
            tri!(self.send_byte(byte).await);
        }

        tri_digital!(self.stop().await);

        Ok(())
    }

    /// Set the brightness level.
    async fn set_brightness(&mut self, brightness: Brightness) -> Result<(), TM1637Error<ERR>> {
        tri_digital!(self.start().await);
        tri!(self.send_byte(brightness as u8).await);
        tri_digital!(self.stop().await);

        Ok(())
    }
}
