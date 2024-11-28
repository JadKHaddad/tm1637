//! Device definition and implementation.

use duplicate::duplicate_item;

use crate::Brightness;

/// Identity trait.
///
/// Used to trick the compiler while using [`duplicate_item`] to implement `async` and `blocking` versions of the same module.
/// Using this trait, we can write normal rust code that can also be formatted by `rustfmt`.
trait Identity: Sized {
    fn identity(self) -> Self {
        self
    }
}

impl<T: Sized> Identity for T {}

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

    /// Build an [`AsyncTM1637`](crate::AsyncTM1637) instance.
    pub fn build_async<const N: usize>(self) -> asynch::TM1637<N, CLK, DIO, DELAY> {
        asynch::TM1637::new(
            self.clk,
            self.dio,
            self.delay,
            self.brightness,
            self.delay_us,
        )
    }

    /// Build a [`BlockingTM1637`](crate::BlockingTM1637) instance.
    pub fn build_blocking<const N: usize>(self) -> blocking::TM1637<N, CLK, DIO, DELAY> {
        blocking::TM1637::new(
            self.clk,
            self.dio,
            self.delay,
            self.brightness,
            self.delay_us,
        )
    }
}

#[duplicate_item(
    name          module        async     await               delay_trait;
    ["Async"]     [asynch]      [async]   [await.identity()]  [embedded_hal_async::delay::DelayNs];
    ["Blocking"]  [blocking]    []        [identity()]        [embedded_hal::delay::DelayNs];
)]
pub mod module {
    //! Device definition and implementation.

    mod inner {
        use super::super::Identity;
        use crate::{Brightness, ConditionalInputPin, Error, TM1637Builder};
        use embedded_hal::digital::OutputPin;

        #[doc = name]
        /// `TM1637` 7-segment display driver.
        ///
        /// # Type parameters
        ///
        /// - `N`: Number of positions on the display.
        /// - `CLK`: Clock.
        /// - `DIO`: Data input/output.
        /// - `DELAY`: Delay provider.
        #[derive(Debug, Clone)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub struct TM1637<const N: usize, CLK, DIO, DELAY> {
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
        }

        impl<const N: usize, CLK, DIO, DELAY> TM1637<N, CLK, DIO, DELAY> {
            /// Create a new [`TM1637`] instance.
            pub const fn new(
                clk: CLK,
                dio: DIO,
                delay: DELAY,
                brightness: Brightness,
                delay_us: u32,
            ) -> Self {
                Self {
                    clk,
                    dio,
                    delay,
                    brightness,
                    delay_us,
                }
            }

            /// Create a new [`TM1637Builder`] instance.
            ///
            /// See [`TM1637Builder::new`] for default values.
            pub const fn builder(
                clk: CLK,
                dio: DIO,
                delay: DELAY,
            ) -> TM1637Builder<CLK, DIO, DELAY> {
                TM1637Builder::new(clk, dio, delay)
            }

            /// Get the number of positions on the display.
            pub const fn num_positions(&self) -> usize {
                N
            }

            /// Get the brightness level.
            pub const fn brightness(&self) -> Brightness {
                self.brightness
            }

            /// Get the delay in microseconds.
            pub const fn delay_us(&self) -> u32 {
                self.delay_us
            }

            /// Get a reference to the clock pin.
            pub const fn clk(&self) -> &CLK {
                &self.clk
            }

            /// Get a mutable reference to the clock pin.
            pub fn clk_mut(&mut self) -> &mut CLK {
                &mut self.clk
            }

            /// Get a reference to the data input/output pin.
            pub const fn dio(&self) -> &DIO {
                &self.dio
            }

            /// Get a mutable reference to the data input/output pin.
            pub fn dio_mut(&mut self) -> &mut DIO {
                &mut self.dio
            }

            /// Get a reference to the delay provider.
            pub const fn delay(&self) -> &DELAY {
                &self.delay
            }

            /// Get a mutable reference to the delay provider.
            pub fn delay_mut(&mut self) -> &mut DELAY {
                &mut self.delay
            }

            /// Split the [`TM1637`] instance into its parts.
            pub fn into_parts(self) -> (CLK, DIO, DELAY) {
                (self.clk, self.dio, self.delay)
            }
        }

        impl<const N: usize, CLK, DIO, DELAY, ERR> TM1637<N, CLK, DIO, DELAY>
        where
            CLK: OutputPin<Error = ERR>,
            DIO: OutputPin<Error = ERR> + ConditionalInputPin<ERR>,
            DELAY: delay_trait,
        {
            /// Send a byte to the display and wait for the ACK.
            async fn write_byte(&mut self, byte: u8) -> Result<(), Error<ERR>> {
                let mut rest = byte;

                for _ in 0..8 {
                    self.clk.set_low()?;

                    match rest & 0x01 {
                        1 => self.dio.set_high()?,
                        _ => self.dio.set_low()?,
                    }

                    self.bit_delay().await;

                    self.clk.set_high()?;
                    self.bit_delay().await;

                    rest >>= 1;
                }

                self.clk.set_low()?;
                self.dio.set_low()?;
                self.bit_delay().await;

                self.clk.set_high()?;
                self.bit_delay().await;

                // Ack
                #[cfg(feature = "ack")]
                let ack = self.wait_for_ack().await?;

                self.clk.set_low()?;
                self.dio.set_low()?;
                self.bit_delay().await;

                #[cfg(feature = "ack")]
                ack.then_some(()).ok_or(Error::Ack)?;

                Ok(())
            }

            /// Wait for 255 cycles for the acknowledgment signal from the display.
            #[cfg(feature = "ack")]
            async fn wait_for_ack(&mut self) -> Result<bool, Error<ERR>> {
                for _ in 0..255 {
                    if self.dio.is_low()? {
                        return Ok(true);
                    }

                    self.bit_delay().await;
                }

                Ok(false)
            }

            /// Start the communication with the display.
            async fn start(&mut self) -> Result<(), Error<ERR>> {
                self.dio.set_high()?;
                self.clk.set_high()?;
                self.bit_delay().await;
                self.dio.set_low()?;
                self.bit_delay().await;

                Ok(())
            }

            /// Stop the communication with the display.
            async fn stop(&mut self) -> Result<(), Error<ERR>> {
                self.dio.set_low()?;
                self.clk.set_high()?;
                self.bit_delay().await;
                self.dio.set_high()?;
                self.bit_delay().await;

                Ok(())
            }

            /// Write the `cmd` to the display.
            async fn write_cmd_raw(&mut self, cmd: u8) -> Result<(), Error<ERR>> {
                self.start().await?;
                self.write_byte(cmd).await?;
                self.stop().await?;

                Ok(())
            }

            // Perform command 1.
            async fn write_start_segments_cmd(&mut self) -> Result<(), Error<ERR>> {
                self.write_cmd_raw(0x40).await?;

                Ok(())
            }

            // Perform command 2.
            async fn write_set_segments_cmd<ITER: Iterator<Item = u8>>(
                &mut self,
                position: u8,
                bytes: ITER,
            ) -> Result<(), Error<ERR>> {
                self.start().await?;

                self.write_byte(0xc0 | (position & 0x03)).await?;

                for byte in bytes {
                    self.write_byte(byte).await?;
                }

                self.stop().await?;

                Ok(())
            }

            /// Perform command 3.
            async fn write_brightness_cmd(
                &mut self,
                brightness: Brightness,
            ) -> Result<(), Error<ERR>> {
                self.write_cmd_raw(brightness as u8).await
            }

            /// Delay for [`TM1637::delay_us`] microseconds using [`TM1637::delay`] provider.
            async fn bit_delay(&mut self) {
                self.delay.delay_us(self.delay_us).await;
            }

            /// Initialize the display.
            ///
            /// Clear the display and set the brightness level.
            pub async fn init(&mut self) -> Result<(), Error<ERR>> {
                self.clear().await?;
                self.write_cmd_raw(self.brightness as u8).await
            }

            /// Turn the display on.
            pub async fn on(&mut self) -> Result<(), Error<ERR>> {
                self.write_cmd_raw(self.brightness as u8).await
            }

            /// Turn the display off.
            pub async fn off(&mut self) -> Result<(), Error<ERR>> {
                self.write_cmd_raw(Brightness::Off as u8).await
            }

            /// Clear the display.
            pub async fn clear(&mut self) -> Result<(), Error<ERR>> {
                self.write_segments_raw_iter(0, core::iter::repeat(0).take(self.num_positions()))
                    .await
            }

            /// Write the given `bytes` to the display starting from the given `position`.
            ///
            /// See [`TM1637::write_segments_raw_mapped`].
            pub async fn write_segments_raw(
                &mut self,
                position: u8,
                bytes: &[u8],
            ) -> Result<(), Error<ERR>> {
                self.write_segments_raw_iter(position, bytes.iter().copied())
                    .await
            }

            /// Write the given `bytes` to the display starting from the given `position` mapping each byte using the provided `map` function.
            ///
            /// See [`TM1637::write_segments_raw_iter`].
            pub async fn write_segments_raw_mapped(
                &mut self,
                position: u8,
                bytes: &[u8],
                map: impl FnMut(u8) -> u8,
            ) -> Result<(), Error<ERR>> {
                self.write_segments_raw_iter(position, bytes.iter().copied().map(map))
                    .await
            }

            /// TODO: the position is not correct
            pub async fn write_segments_raw_flipped(
                &mut self,
                position: u8,
                bytes: &[u8],
            ) -> Result<(), Error<ERR>> {
                self.write_segments_raw_iter(
                    self.num_positions() as u8 - position - bytes.len() as u8,
                    bytes
                        .iter()
                        .copied()
                        .rev()
                        .map(crate::mappings::flip_mirror),
                )
                .await
            }

            /// Write the given `bytes` to the display starting from the given `position`.
            ///
            /// # Notes
            /// - Positions greater than [`TM1637::num_positions`] will be ignored.
            /// - Bytes with index greater than [`TM1637::num_positions`] will be ignored.
            ///
            /// Brightness level will not be written to the device on each call. Make sure to call [`TM1637::write_brightness`] or [`TM1637::init`] to set the brightness level.
            pub async fn write_segments_raw_iter(
                &mut self,
                position: u8,
                bytes: impl Iterator<Item = u8>,
            ) -> Result<(), Error<ERR>> {
                #[cfg(not(feature = "disable-checks"))]
                if position as usize >= self.num_positions() {
                    return Ok(());
                }

                #[cfg(not(feature = "disable-checks"))]
                let bytes = bytes.take(self.num_positions() - position as usize);

                // Comm 1
                self.write_start_segments_cmd().await?;

                // Comm 2
                self.write_set_segments_cmd(position, bytes).await?;

                Ok(())
            }

            /// Set [`TM1637::brightness`] and write the brightness level to the display.
            pub async fn write_brightness(
                &mut self,
                brightness: Brightness,
            ) -> Result<(), Error<ERR>> {
                self.brightness = brightness;

                self.write_brightness_cmd(brightness).await
            }

            /// Move the given `bytes` across the display starting and ending at `position`.
            ///
            /// If the length of the bytes is less than or equal to [`TM1637::num_positions`] - `position`, the bytes will only be written to the display.
            ///
            /// See [`TM1637::move_segments_raw_mapped`].
            pub async fn move_segments_raw(
                &mut self,
                position: u8,
                bytes: &[u8],
                delay_ms: u32,
            ) -> Result<(), Error<ERR>> {
                self.move_segments_raw_mapped(position, bytes, delay_ms, |byte| byte)
                    .await
            }

            /// Move the given `bytes` across the display starting and ending at `position` mapping each byte using the provided `map` function.
            pub async fn move_segments_raw_mapped(
                &mut self,
                position: u8,
                bytes: &[u8],
                delay_ms: u32,
                map: impl FnMut(u8) -> u8 + Clone,
            ) -> Result<(), Error<ERR>> {
                if bytes.len() <= self.num_positions() - position as usize {
                    return self.write_segments_raw_mapped(position, bytes, map).await;
                }

                for i in 0..=bytes.len() {
                    let mut window = [0u8; N];

                    for j in 0..self.num_positions() {
                        window[j] = bytes[(i + j) % bytes.len()];
                    }

                    self.write_segments_raw_mapped(position, &window, map.clone())
                        .await?;

                    self.delay.delay_ms(delay_ms).await;
                }

                Ok(())
            }

            /// Write the given `ascii_str` to the display starting from the given `position` mapping each byte using [`from_ascii_byte`](crate::mappings::from_ascii_byte).
            ///
            /// See [`TM1637::write_segments_raw_mapped`].
            ///
            /// # Example
            ///
            /// Write the string `"Err"` to the display:
            ///
            /// ```rust, ignore
            /// let mut tm = TM1637Builder::new(clk_pin, dio_pin, delay)
            ///    .brightness(Brightness::L3)
            ///    .build::<4>();
            ///
            /// tm.init().ok();
            ///
            /// tm.write_ascii_str(0, "Err").ok();
            /// ```
            ///
            /// On a `4-digit display`, this will look like this:
            ///
            /// ```text
            /// +---+ +---+ +---+ +---+
            /// | E | | r | | r | |   |
            /// +---+ +---+ +---+ +---+
            /// ```
            pub async fn write_ascii_str(
                &mut self,
                position: u8,
                ascii_str: &str,
            ) -> Result<(), Error<ERR>> {
                self.write_segments_raw_iter(
                    position,
                    ascii_str
                        .as_bytes()
                        .iter()
                        .copied()
                        .map(crate::mappings::from_ascii_byte),
                )
                .await
            }

            /// TODO: the position is not correct
            pub async fn write_ascii_str_flipped(
                &mut self,
                position: u8,
                ascii_str: &str,
            ) -> Result<(), Error<ERR>> {
                self.write_segments_raw_iter(
                    self.num_positions() as u8 - position - ascii_str.len() as u8,
                    ascii_str
                        .as_bytes()
                        .iter()
                        .copied()
                        .rev()
                        .map(crate::mappings::from_ascii_byte)
                        .map(crate::mappings::flip_mirror),
                )
                .await
            }

            /// Move the given `ascii_str` across the display starting and ending at `position` mapping each byte using [`from_ascii_byte`](crate::mappings::from_ascii_byte).
            ///
            /// See [`TM1637::move_segments_raw_mapped`].
            ///
            /// # Example
            ///
            /// Move the string `"HELLO "` across a `4-digit display`:
            ///
            /// ```rust, ignore
            /// let mut tm = TM1637Builder::new(clk_pin, dio_pin, delay)
            ///    .brightness(Brightness::L3)
            ///    .build::<4>();
            ///
            /// tm.init().ok();
            ///
            /// tm.move_ascii_str(0, "HELLO ", 500).ok();
            /// ```
            ///
            /// On a `4-digit display`, this will look like this:
            ///
            /// ```text
            /// +---+ +---+ +---+ +---+
            /// | H | | E | | L | | L |
            /// +---+ +---+ +---+ +---+
            ///
            /// +---+ +---+ +---+ +---+
            /// | E | | L | | L | | O |
            /// +---+ +---+ +---+ +---+
            ///
            /// +---+ +---+ +---+ +---+
            /// | L | | L | | O | |   |
            /// +---+ +---+ +---+ +---+
            ///
            /// +---+ +---+ +---+ +---+
            /// | L | | O | |   | | H |
            /// +---+ +---+ +---+ +---+
            ///
            /// +---+ +---+ +---+ +---+
            /// | O | |   | | H | | E |
            /// +---+ +---+ +---+ +---+
            ///
            /// +---+ +---+ +---+ +---+
            /// |   | | H | | E | | L |
            /// +---+ +---+ +---+ +---+
            ///
            /// +---+ +---+ +---+ +---+
            /// | H | | E | | L | | L |
            /// +---+ +---+ +---+ +---+
            /// ```
            pub async fn move_ascii_str(
                &mut self,
                position: u8,
                ascii_str: &str,
                delay_ms: u32,
            ) -> Result<(), Error<ERR>> {
                self.move_segments_raw_mapped(
                    position,
                    ascii_str.as_bytes(),
                    delay_ms,
                    crate::mappings::from_ascii_byte,
                )
                .await
            }
        }
    }

    pub use inner::*;
}
