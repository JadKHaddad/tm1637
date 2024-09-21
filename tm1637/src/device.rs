//! Device definition and implementation.

use duplicate::duplicate_item;

/// Identity trait.
///
/// Used to trick the compiler while using [`duplicate_item`] to implement `async` and `blocking` versions of the same module.
/// Using this trait, we can write normal rust code that can also be formatted by `rustfmt`.
#[cfg(any(feature = "async", feature = "blocking"))]
trait Identity: Sized {
    fn identity(self) -> Self {
        self
    }
}

#[cfg(any(feature = "async", feature = "blocking"))]
impl<T: Sized> Identity for T {}

#[duplicate_item(
    feature_        module        async     await               delay_trait;
    ["async"]       [asynch]      [async]   [await.identity()]  [embedded_hal_async::delay::DelayNs];
    ["blocking"]    [blocking]    []        [identity()]        [embedded_hal::delay::DelayNs];
)]
pub mod module {
    //! Device definition and implementation.

    #[cfg(feature=feature_)]
    mod inner {
        use super::super::Identity;
        use crate::brightness::{Brightness, DisplayState};
        use embedded_hal::digital::OutputPin;

        /// `TM1637` 7-segment display builder.
        #[derive(Clone)]
        #[cfg_attr(feature = "impl-defmt-format", derive(defmt::Format))]
        #[cfg_attr(feature = "impl-debug", derive(core::fmt::Debug))]
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
        #[derive(Clone)]
        #[cfg_attr(feature = "impl-defmt-format", derive(defmt::Format))]
        #[cfg_attr(feature = "impl-debug", derive(core::fmt::Debug))]
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

        impl<CLK, DIO, DELAY, ERR> TM1637<CLK, DIO, DELAY>
        where
            CLK: OutputPin<Error = ERR>,
            DIO: OutputPin<Error = ERR>,
            DELAY: delay_trait,
        {
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

            /// Send a byte to the display and wait for the ACK.
            async fn write_byte(&mut self, byte: u8) -> Result<(), ERR> {
                let mut rest = byte;

                for _ in 0..8 {
                    self.bit_delay().await;
                    tri!(self.clk.set_low());
                    self.bit_delay().await;

                    match rest & 0x01 {
                        1 => tri!(self.dio.set_high()),
                        _ => tri!(self.dio.set_low()),
                    }

                    self.bit_delay().await;
                    tri!(self.clk.set_high());
                    self.bit_delay().await;

                    rest >>= 1;
                }

                tri!(self.clk.set_low());
                tri!(self.dio.set_high());
                self.bit_delay().await;

                tri!(self.clk.set_high());
                self.bit_delay().await;

                tri!(self.clk.set_low());
                self.bit_delay().await;

                Ok(())
            }

            /// Write the `cmd` to the display.
            async fn write_cmd_raw(&mut self, cmd: u8) -> Result<(), ERR> {
                tri!(self.start().await);
                tri!(self.write_byte(cmd).await);
                tri!(self.stop().await);

                Ok(())
            }

            /// Start the communication with the display.
            async fn start(&mut self) -> Result<(), ERR> {
                tri!(self.dio.set_low());
                self.bit_delay().await;
                tri!(self.clk.set_low());
                self.bit_delay().await;

                Ok(())
            }

            /// Stop the communication with the display.
            async fn stop(&mut self) -> Result<(), ERR> {
                tri!(self.dio.set_low());
                self.bit_delay().await;
                tri!(self.clk.set_high());
                self.bit_delay().await;
                tri!(self.dio.set_high());
                self.bit_delay().await;

                Ok(())
            }

            /// Delay for [`TM1637::delay_us`] microseconds using [`TM1637::delay`] provider.
            async fn bit_delay(&mut self) {
                self.delay.delay_us(self.delay_us).await;
            }

            /// Initialize the display.
            ///
            /// Clear the display and set the brightness level.
            pub async fn init(&mut self) -> Result<(), ERR> {
                tri!(self.clear().await);
                self.write_cmd_raw(self.brightness as u8).await
            }

            /// Turn the display on.
            pub async fn on(&mut self) -> Result<(), ERR> {
                self.write_cmd_raw(self.brightness as u8).await
            }

            /// Turn the display off.
            pub async fn off(&mut self) -> Result<(), ERR> {
                self.write_cmd_raw(DisplayState::Off as u8).await
            }

            /// Clear the display.
            pub async fn clear(&mut self) -> Result<(), ERR> {
                self.write_segments_raw_iter(
                    0,
                    core::iter::repeat(0).take(self.num_positions as usize),
                )
                .await
            }

            /// Write the given bytes to the display starting from the given position.
            ///
            /// See [`TM1637::write_segments_raw_iter`].
            pub async fn write_segments_raw(
                &mut self,
                position: u8,
                bytes: &[u8],
            ) -> Result<(), ERR> {
                self.write_segments_raw_iter(position, bytes.iter().copied())
                    .await
            }

            /// Write the given bytes to the display starting from the given position.
            ///
            /// ## Notes:
            /// - Positions greater than [`TM1637::num_positions`] will be ignored.
            /// - Bytes with index greater than [`TM1637::num_positions`] will be ignored.
            ///
            /// Brightness level will not be written to the device on each call. Make sure to call [`TM1637::write_brightness`] or [`TM1637::init`] to set the brightness level.
            pub async fn write_segments_raw_iter<ITER: Iterator<Item = u8>>(
                &mut self,
                position: u8,
                bytes: ITER,
            ) -> Result<(), ERR> {
                #[cfg(not(feature = "disable-checks"))]
                if position >= self.num_positions {
                    return Ok(());
                }

                // COMM1
                tri!(self.write_cmd_raw(0x40).await);

                // COMM2
                tri!(self.start().await);
                tri!(self.write_byte(0xc0 | (position & 0x03)).await);

                #[cfg(not(feature = "disable-checks"))]
                let bytes = bytes.take(self.num_positions as usize - position as usize);

                for byte in bytes {
                    tri!(self.write_byte(byte).await);
                }

                tri!(self.stop().await);

                Ok(())
            }

            /// Set [`TM1637::brightness`] and write the brightness level to the display.
            pub async fn write_brightness(&mut self, brightness: Brightness) -> Result<(), ERR> {
                self.brightness = brightness;
                self.write_cmd_raw(brightness as u8).await
            }

            /// Move all segments across the display starting and ending at `position`.
            ///
            /// If the length of the bytes is less than or equal to [`TM1637::num_positions`] - `position`, the bytes will be written to the display.
            pub async fn move_segments<const N: usize>(
                &mut self,
                position: u8,
                bytes: &[u8],
                delay_ms: u32,
            ) -> Result<(), ERR> {
                let num_positions = self.num_positions as usize;

                if bytes.len() <= num_positions - position as usize {
                    return self.write_segments_raw(position, bytes).await;
                }

                for i in 0..=bytes.len() {
                    let mut window = [0u8; N];
                    for j in 0..num_positions {
                        window[j] = bytes[(i + j) % bytes.len()];
                    }

                    tri!(self.write_segments_raw(position, &window).await);

                    self.delay.delay_ms(delay_ms).await;
                }

                Ok(())
            }
        }
    }

    #[cfg(feature=feature_)]
    pub use inner::*;
}
