use crate::{
    mode::Mode,
    tokens::{Async, Blocking},
    Brightness, TM1637,
};

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
    /// - `delay_us`: 100
    pub const fn new(clk: CLK, dio: DIO, delay: DELAY) -> Self {
        Self {
            clk,
            dio,
            delay,
            brightness: Brightness::L0,
            delay_us: 100,
        }
    }

    /// Set the brightness level.
    pub const fn brightness(mut self, brightness: Brightness) -> Self {
        self.brightness = brightness;
        self
    }

    /// Set the delay in microseconds.
    pub const fn delay_us(mut self, delay_us: u32) -> Self {
        self.delay_us = delay_us;
        self
    }

    /// Build a [`TM1637`] instance with the specified mode.
    ///
    /// ## Async
    ///
    /// ```rust
    /// use tm1637_embedded_hal::{mock::Noop, tokens::Async, TM1637Builder};
    ///
    /// let clk = Noop;
    /// let dio = Noop;
    /// let delay = Noop;
    ///
    /// let tm = TM1637Builder::new(clk, dio, delay).build::<4, Async>();
    /// ```
    ///
    /// ## Blocking
    ///
    /// ```rust
    /// use tm1637_embedded_hal::{mock::Noop, tokens::Blocking, TM1637Builder};
    ///
    /// let clk = Noop;
    /// let dio = Noop;
    /// let delay = Noop;
    ///
    /// let tm = TM1637Builder::new(clk, dio, delay).build::<4, Blocking>();
    /// ```
    pub fn build<const N: usize, T: Mode>(self) -> TM1637<N, T, CLK, DIO, DELAY> {
        TM1637::new(
            self.clk,
            self.dio,
            self.delay,
            self.brightness,
            self.delay_us,
        )
    }

    /// Build an async [`TM1637`] instance.
    pub fn build_async<const N: usize>(self) -> TM1637<N, Async, CLK, DIO, DELAY> {
        self.build()
    }

    /// Build a blocking [`TM1637`] instance.
    pub fn build_blocking<const N: usize>(self) -> TM1637<N, Blocking, CLK, DIO, DELAY> {
        self.build()
    }
}
