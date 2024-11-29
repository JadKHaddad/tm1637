use crate::{AsyncTM1637, BlockingTM1637, Brightness};

/// `TM1637` 7-segment display builder.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct TM1637Builder<CLK, DIO, DELAY> {
    clk: CLK,
    dio: DIO,
    delay: DELAY,
    brightness: Brightness,
    delay_us: u32,
}

impl<CLK, DIO, DELAY> TM1637Builder<CLK, DIO, DELAY> {
    /// Create a new [`TM1637Builder`] instance with default values.
    ///
    /// - `brightness`: [`Brightness::L0`]
    /// - `delay_us`: 10
    pub const fn new(clk: CLK, dio: DIO, delay: DELAY) -> Self {
        Self {
            clk,
            dio,
            delay,
            brightness: Brightness::L0,
            delay_us: 10,
        }
    }

    /// Set the brightness level.
    pub fn brightness(mut self, brightness: Brightness) -> Self {
        self.brightness = brightness;
        self
    }

    /// Set the delay in microseconds.
    pub fn delay_us(mut self, delay_us: u32) -> Self {
        self.delay_us = delay_us;
        self
    }

    /// Build an [`AsyncTM1637`] instance.
    pub fn build_async<const N: usize>(self) -> AsyncTM1637<N, CLK, DIO, DELAY> {
        AsyncTM1637::new(
            self.clk,
            self.dio,
            self.delay,
            self.brightness,
            self.delay_us,
        )
    }

    /// Build a [`BlockingTM1637`] instance.
    pub fn build_blocking<const N: usize>(self) -> BlockingTM1637<N, CLK, DIO, DELAY> {
        BlockingTM1637::new(
            self.clk,
            self.dio,
            self.delay,
            self.brightness,
            self.delay_us,
        )
    }
}
