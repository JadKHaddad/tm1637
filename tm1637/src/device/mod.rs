//! Device definition and implementation.

use crate::functionality::BaseTM1637;

use self::brightness::Brightness;

pub mod brightness;

/// `TM1637` 7-segment display builder.
#[derive(Debug, Clone)]
pub struct TM1637Builder<CLK, DIO, DELAY> {
    /// The inner [`TM1637`] instance.
    inner: TM1637<CLK, DIO, DELAY>,
}

impl<CLK, DIO, DELAY> TM1637Builder<CLK, DIO, DELAY> {
    /// Create a new [`TM1637Builder`] instance.
    pub fn new(clk: CLK, dio: DIO, delay: DELAY) -> Self {
        Self {
            inner: TM1637 {
                clk,
                dio,
                delay,
                brightness: Brightness::L0,
                delay_us: 10,
                num_positions: 4,
            },
        }
    }

    /// Set the brightness level.
    pub fn brightness(mut self, brightness: Brightness) -> Self {
        self.inner.brightness = brightness;
        self
    }

    /// Set the delay in microseconds.
    pub fn delay_us(mut self, delay_us: u32) -> Self {
        self.inner.delay_us = delay_us;
        self
    }

    /// Set the number of positions on the display.
    pub fn num_positions(mut self, num_positions: u8) -> Self {
        self.inner.num_positions = num_positions;
        self
    }

    /// Build the [`TM1637`] instance.
    pub fn build(self) -> TM1637<CLK, DIO, DELAY> {
        self.inner
    }
}

/// `TM1637` 7-segment display driver.
#[derive(Debug, Clone)]
pub struct TM1637<CLK, DIO, DELAY> {
    /// Clock.
    clk: CLK,
    /// Data input/output.
    dio: DIO,
    /// Delay provider.
    delay: DELAY,
    /// Brightness level.
    brightness: Brightness,
    /// The delay in microseconds.
    ///
    /// Experiment with this value to find the best value for your display.
    delay_us: u32,
    /// The number of positions on the display.
    num_positions: u8,
}

impl<CLK, DIO, DELAY> TM1637<CLK, DIO, DELAY> {
    /// Create a new [`TM1637`] instance.
    pub fn new(
        clk: CLK,
        dio: DIO,
        delay: DELAY,
        brightness: Brightness,
        delay_us: u32,
        num_positions: u8,
    ) -> Self {
        Self {
            clk,
            dio,
            delay,
            brightness,
            delay_us,
            num_positions,
        }
    }

    /// Create a new [`TM1637Builder`] instance.
    pub fn builder(clk: CLK, dio: DIO, delay: DELAY) -> TM1637Builder<CLK, DIO, DELAY> {
        TM1637Builder::new(clk, dio, delay)
    }
}

impl<CLK, DIO, DELAY> BaseTM1637<CLK, DIO, DELAY> for TM1637<CLK, DIO, DELAY> {
    fn clk(&self) -> &CLK {
        &self.clk
    }

    fn clk_mut(&mut self) -> &mut CLK {
        &mut self.clk
    }

    fn dio(&self) -> &DIO {
        &self.dio
    }

    fn dio_mut(&mut self) -> &mut DIO {
        &mut self.dio
    }

    fn delay(&self) -> &DELAY {
        &self.delay
    }

    fn delay_mut(&mut self) -> &mut DELAY {
        &mut self.delay
    }

    fn brightness(&self) -> Brightness {
        self.brightness
    }

    fn brightness_mut(&mut self) -> &mut Brightness {
        &mut self.brightness
    }

    fn delay_us(&self) -> u32 {
        self.delay_us
    }

    fn num_positions(&self) -> u8 {
        self.num_positions
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
