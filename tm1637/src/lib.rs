#![no_std]

/// Our custom `try!` macro aka `?`, to get rid of [`core::convert::From`]/[`core::convert::Into`] used by the `?` operator.
macro_rules! tri {
    ($e:expr $(,)?) => {
        match $e {
            core::result::Result::Ok(value) => value,
            core::result::Result::Err(err) => {
                return core::result::Result::Err(err);
            }
        }
    };
}

use tri;

pub mod mappings;

/// The level of brightness.
///
/// Represents a byte that can be sent to the `TM1637` to set the brightness level.
///
/// ## Bits:
/// - 1-3: Brightness level (0-7)
/// - 4: Display on/off
/// - 5-7: Base address
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Brightness {
    /// Display off.
    OFF = 0b10000000,
    /// Brightness level 0. Lowest brightness.
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

/// A bit. For private use.
enum Bit {
    /// Zero.
    ZERO,
    /// One.
    ONE,
}

#[derive(Debug)]
pub enum TM1637Error<ERR> {
    /// Acknowledge error. The display did not acknowledge the sent byte.
    Ack,
    /// Digital error.
    Digital(ERR),
}

impl<ERR> From<ERR> for TM1637Error<ERR> {
    fn from(err: ERR) -> Self {
        TM1637Error::Digital(err)
    }
}

/// `TM1637` 7-segment display driver.
#[derive(Debug)]
pub struct TM1637<CLK, DIO, DELAY> {
    /// Clock.
    clk: CLK,
    /// Data input/output.
    dio: DIO,
    /// Delay provider.
    delay: DELAY,
    /// The delay in microseconds.
    ///
    /// Experiment with this value to find the best value for your display.
    delay_us: u32,
    /// The number of addresses on the display.
    address_count: u8,
}

use embedded_hal::{
    delay::DelayNs,
    digital::{InputPin, OutputPin},
};

impl<CLK, DIO, DELAY, ERR> TM1637<CLK, DIO, DELAY>
where
    CLK: OutputPin<Error = ERR>,
    DIO: InputPin<Error = ERR> + OutputPin<Error = ERR>,
    DELAY: DelayNs,
{
    pub fn new(clk: CLK, dio: DIO, delay: DELAY, delay_us: u32, address_count: u8) -> Self {
        Self {
            clk,
            dio,
            delay,
            delay_us,
            address_count,
        }
    }

    /// Clear the display.
    pub fn clear(&mut self) -> Result<(), TM1637Error<ERR>> {
        self.write_raw_iter(0, core::iter::repeat(0).take(self.address_count as usize))
    }

    /// Write the given bytes to the display starting from the given address. See [`TM1637::write_raw_iter`].
    ///
    /// ## Notes:
    /// - Addresses greater than [`TM1637::address_count`] will be ignored.
    /// - Bytes with index greater than [`TM1637::address_count`] will be ignored.
    pub fn write_raw(&mut self, address: u8, bytes: &[u8]) -> Result<(), TM1637Error<ERR>> {
        self.write_raw_iter(address, bytes.iter().map(|b| *b))
    }

    /// Write the given bytes to the display starting from the given address. See [`TM1637::write_raw`].
    ///
    /// ## Notes:
    /// - Addresses greater than [`Self::address_count`] will be ignored.
    /// - Bytes with index greater than [`Self::address_count`] will be ignored.
    pub fn write_raw_iter<ITER: Iterator<Item = u8>>(
        &mut self,
        address: u8,
        bytes: ITER,
    ) -> Result<(), TM1637Error<ERR>> {
        if address >= self.address_count {
            return Ok(());
        }

        self.start()?;
        self.send_byte(0xc0 | (address & 0x0f))?;

        let bytes_to_send = bytes.take(self.address_count as usize - address as usize);
        for byte in bytes_to_send {
            self.send_byte(byte)?;
        }

        self.stop()?;

        Ok(())
    }

    /// Set the brightness level.
    pub fn set_brightness(&mut self, brightness: Brightness) -> Result<(), TM1637Error<ERR>> {
        self.start()?;
        self.send_byte(brightness as u8)?;
        self.stop()?;

        Ok(())
    }

    /// Send a byte to the display and wait for the ACK.
    fn send_byte(&mut self, byte: u8) -> Result<(), TM1637Error<ERR>> {
        let mut rest = byte;
        for _ in 0..8 {
            let bit = if rest & 1 != 0 { Bit::ONE } else { Bit::ZERO };
            self.send_bit_and_delay(bit)?;
            rest = rest >> 1;
        }

        // Wait for the ACK
        self.send_bit_and_delay(Bit::ONE)?;
        for _ in 0..255 {
            if self.dio.is_low()? {
                return Ok(());
            }

            self.delay();
        }

        Err(TM1637Error::Ack)
    }

    /// Start the communication with the display.
    fn start(&mut self) -> Result<(), ERR> {
        self.send_bit_and_delay(Bit::ONE)?;
        self.dio.set_low()?;

        Ok(())
    }

    /// Stop the communication with the display.
    fn stop(&mut self) -> Result<(), ERR> {
        self.send_bit_and_delay(Bit::ZERO)?;
        self.dio.set_high()?;
        self.delay();

        Ok(())
    }

    /// Send a bit to the display and delay.
    fn send_bit_and_delay(&mut self, value: Bit) -> Result<(), ERR> {
        self.clk.set_low()?;
        match value {
            Bit::ONE => self.dio.set_high()?,
            Bit::ZERO => self.dio.set_low()?,
        }
        self.clk.set_high()?;
        self.delay();

        Ok(())
    }

    /// Delay for the given amount of microseconds with the delay provider.
    fn delay(&mut self) {
        self.delay.delay_us(self.delay_us);
    }
}
