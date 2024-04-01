//! Device definition and implementation.

use crate::functionality::BaseTM1637;

pub mod brightness;

/// `TM1637` 7-segment display driver.
#[derive(Debug, Clone)]
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

impl<CLK, DIO, DELAY> TM1637<CLK, DIO, DELAY> {
    /// Create a new `TM1637` instance.
    pub fn new(clk: CLK, dio: DIO, delay: DELAY, delay_us: u32, address_count: u8) -> Self {
        Self {
            clk,
            dio,
            delay,
            delay_us,
            address_count,
        }
    }
}

impl<CLK, DIO, DELAY> BaseTM1637<CLK, DIO, DELAY> for TM1637<CLK, DIO, DELAY> {
    fn clk(&mut self) -> &mut CLK {
        &mut self.clk
    }

    fn dio(&mut self) -> &mut DIO {
        &mut self.dio
    }

    fn delay(&mut self) -> &mut DELAY {
        &mut self.delay
    }

    fn delay_us(&self) -> u32 {
        self.delay_us
    }

    fn address_count(&self) -> u8 {
        self.address_count
    }
}

#[cfg(feature = "blocking")]
mod impl_blocking {
    use embedded_hal::{
        delay::DelayNs,
        digital::{InputPin, OutputPin},
    };

    use crate::functionality::blocking::{BlockingTM1637, PrivateBlockingTM1637};

    use super::TM1637;

    impl<CLK, DIO, DELAY, ERR> PrivateBlockingTM1637<CLK, DIO, DELAY, ERR> for TM1637<CLK, DIO, DELAY>
    where
        CLK: OutputPin<Error = ERR>,
        DIO: InputPin<Error = ERR> + OutputPin<Error = ERR>,
        DELAY: DelayNs,
    {
    }

    impl<CLK, DIO, DELAY, ERR> BlockingTM1637<CLK, DIO, DELAY, ERR> for TM1637<CLK, DIO, DELAY>
    where
        CLK: OutputPin<Error = ERR>,
        DIO: InputPin<Error = ERR> + OutputPin<Error = ERR>,
        DELAY: DelayNs,
    {
    }
}

#[cfg(feature = "async")]
mod impl_async {
    use embedded_hal::digital::{InputPin, OutputPin};
    use embedded_hal_async::delay::DelayNs;

    use crate::functionality::asynchronous::{AsyncTM1637, PrivateAsyncTM1637};

    use super::TM1637;

    impl<CLK, DIO, DELAY, ERR> PrivateAsyncTM1637<CLK, DIO, DELAY, ERR> for TM1637<CLK, DIO, DELAY>
    where
        CLK: OutputPin<Error = ERR>,
        DIO: InputPin<Error = ERR> + OutputPin<Error = ERR>,
        DELAY: DelayNs,
    {
    }

    impl<CLK, DIO, DELAY, ERR> AsyncTM1637<CLK, DIO, DELAY, ERR> for TM1637<CLK, DIO, DELAY>
    where
        CLK: OutputPin<Error = ERR>,
        DIO: InputPin<Error = ERR> + OutputPin<Error = ERR>,
        DELAY: DelayNs,
    {
    }
}
