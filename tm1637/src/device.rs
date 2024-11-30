use core::marker::PhantomData;

use crate::{Brightness, TM1637Builder};

/// `TM1637` 7-segment display driver.
///
/// # Type parameters
///
/// - `N`: Number of positions on the display.
/// - `T`: Operating mode. [`Async`](crate::Async) or [`Blocking`](crate::Blocking).
/// - `CLK`: Clock.
/// - `DIO`: Data input/output.
/// - `DELAY`: Delay provider.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct TM1637<const N: usize, T, CLK, DIO, DELAY> {
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
    _token: PhantomData<T>,
}

impl<const N: usize, T, CLK, DIO, DELAY> TM1637<N, T, CLK, DIO, DELAY> {
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
            _token: PhantomData,
        }
    }

    /// Create a new [`TM1637Builder`] instance.
    ///
    /// See [`TM1637Builder::new`] for default values.
    pub const fn builder(clk: CLK, dio: DIO, delay: DELAY) -> TM1637Builder<CLK, DIO, DELAY> {
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

#[::duplicate::duplicate_item(
    module        async     await               Token                 DelayTrait;
    [asynch]      [async]   [await.identity()]  [crate::Async]        [::embedded_hal_async::delay::DelayNs];
    [blocking]    []        [identity()]        [crate::Blocking]     [::embedded_hal::delay::DelayNs];
)]
pub mod module {
    use crate::{
        Brightness, ConditionalInputPin, Direction, Error, Identity, InitDisplayOptions, TM1637,
    };
    use ::embedded_hal::digital::OutputPin;

    impl<const N: usize, CLK, DIO, DELAY, ERR> TM1637<N, Token, CLK, DIO, DELAY>
    where
        CLK: OutputPin<Error = ERR>,
        DIO: OutputPin<Error = ERR> + ConditionalInputPin<ERR>,
        DELAY: DelayTrait,
    {
        fn rev_bytes(bytes: &[u8], position: usize) -> &[u8] {
            if bytes.len() + position > N {
                &bytes[..N - position]
            } else {
                bytes
            }
        }

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
        async fn write_cmd(&mut self, cmd: u8) -> Result<(), Error<ERR>> {
            self.start().await?;
            self.write_byte(cmd).await?;
            self.stop().await?;

            Ok(())
        }

        /// Perform command 1.
        async fn write_start_display_cmd(&mut self) -> Result<(), Error<ERR>> {
            self.write_cmd(0x40).await?;

            Ok(())
        }

        /// Perform command 2.
        async fn write_display_cmd(
            &mut self,
            position: usize,
            bytes: impl Iterator<Item = u8>,
        ) -> Result<(), Error<ERR>> {
            self.start().await?;

            self.write_byte(0xc0 | (position as u8 & 0x03)).await?;

            for byte in bytes {
                self.write_byte(byte).await?;
            }

            self.stop().await?;

            Ok(())
        }

        /// Perform command 3.
        async fn write_brightness_cmd(&mut self, brightness: Brightness) -> Result<(), Error<ERR>> {
            self.write_cmd(brightness as u8).await
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
            self.write_cmd(self.brightness as u8).await
        }

        /// Turn the display on.
        pub async fn on(&mut self) -> Result<(), Error<ERR>> {
            self.write_cmd(self.brightness as u8).await
        }

        /// Turn the display off.
        pub async fn off(&mut self) -> Result<(), Error<ERR>> {
            self.write_cmd(Brightness::Off as u8).await
        }

        /// Clear the display.
        pub async fn clear(&mut self) -> Result<(), Error<ERR>> {
            self.display(0, ::core::iter::repeat(0).take(self.num_positions()))
                .await
        }

        /// Set [`TM1637::brightness`] and write the brightness level to the display.
        pub async fn write_brightness(&mut self, brightness: Brightness) -> Result<(), Error<ERR>> {
            self.brightness = brightness;

            self.write_brightness_cmd(brightness).await
        }

        /// Write the given `bytes` to the display starting from `position`.
        ///
        /// # Notes
        ///
        /// - Positions greater than [`TM1637::num_positions`] will be written to the display regardless.
        /// - Bytes with index greater than [`TM1637::num_positions`] will be written to the display regardless.
        ///
        /// Brightness level will not be written to the device on each call. Make sure to call [`TM1637::write_brightness`] or [`TM1637::init`] to set the brightness level.
        ///
        /// See [`TM1637::display`].
        pub async fn display_unchecked(
            &mut self,
            position: usize,
            bytes: impl Iterator<Item = u8>,
        ) -> Result<(), Error<ERR>> {
            // Comm 1
            self.write_start_display_cmd().await?;

            // Comm 2
            self.write_display_cmd(position, bytes).await?;

            Ok(())
        }

        /// Write the given `bytes` to the display starting from `position`.
        ///
        /// # Notes
        ///
        /// - Positions greater than [`TM1637::num_positions`] will be ignored.
        /// - Bytes with index greater than [`TM1637::num_positions`] will be ignored.
        ///
        /// Brightness level will not be written to the device on each call. Make sure to call [`TM1637::write_brightness`] or [`TM1637::init`] to set the brightness level.
        ///
        /// See [`TM1637::display_unchecked`].
        pub async fn display(
            &mut self,
            position: usize,
            bytes: impl Iterator<Item = u8>,
        ) -> Result<(), Error<ERR>> {
            if position >= self.num_positions() {
                return Ok(());
            }

            self.display_unchecked(position, bytes.take(self.num_positions() - position))
                .await
        }

        /// Write the given `bytes` to the display starting from `position`.
        ///
        /// See [`TM1637::display_unchecked`].
        pub async fn display_slice_unchecked(
            &mut self,
            position: usize,
            bytes: &[u8],
        ) -> Result<(), Error<ERR>> {
            self.display_unchecked(position, bytes.iter().copied())
                .await
        }

        /// Write the given `bytes` to the display starting from `position`.
        ///
        /// See [`TM1637::display_slice_mapped`].
        pub async fn display_slice(
            &mut self,
            position: usize,
            bytes: &[u8],
        ) -> Result<(), Error<ERR>> {
            self.display(position, bytes.iter().copied()).await
        }

        /// Write the given `bytes` to the display starting from `position` mapping each byte using the provided `map` function.
        ///
        /// See [`TM1637::display_unchecked`].
        pub async fn display_slice_mapped_unchecked(
            &mut self,
            position: usize,
            bytes: &[u8],
            map: impl FnMut(u8) -> u8,
        ) -> Result<(), Error<ERR>> {
            self.display_unchecked(position, bytes.iter().copied().map(map))
                .await
        }

        pub async fn display_slice_mapped_dotted_unchecked(
            &mut self,
            position: usize,
            bytes: &[u8],
            dots: &[bool],
            map: impl FnMut(u8) -> u8,
        ) -> Result<(), Error<ERR>> {
            self.display_unchecked(
                position,
                crate::mappings::zip_dots(bytes.iter().copied().map(map), dots.iter().copied()),
            )
            .await
        }

        /// Write the given `bytes` to the display starting from `position` mapping each byte using the provided `map` function.
        ///
        /// See [`TM1637::display`].
        pub async fn display_slice_mapped(
            &mut self,
            position: usize,
            bytes: &[u8],
            map: impl FnMut(u8) -> u8,
        ) -> Result<(), Error<ERR>> {
            self.display(position, bytes.iter().copied().map(map)).await
        }

        pub async fn display_slice_dotted_mapped(
            &mut self,
            position: usize,
            bytes: &[u8],
            dots: &[bool],
            map: impl FnMut(u8) -> u8,
        ) -> Result<(), Error<ERR>> {
            self.display(
                position,
                crate::mappings::zip_dots(bytes.iter().copied().map(map), dots.iter().copied()),
            )
            .await
        }

        /// Write the given `bytes` to the display in reversed order starting from `position` mapping each byte using the provided `map` function.
        ///
        /// # Notes
        /// - `position` is mapped to the reversed position on the display.
        /// - If you write to `position` 0, the bytes will be written to the last position on the display.
        /// - If you write to `position` 1, the bytes will be written to the second last position on the display.
        /// - And so on...
        ///
        /// Core functionality for `flipped` displays.
        pub async fn display_slice_rev_mapped(
            &mut self,
            position: usize,
            bytes: &[u8],
            map: impl FnMut(u8) -> u8,
        ) -> Result<(), Error<ERR>> {
            let bytes = Self::rev_bytes(bytes, position);

            self.display_unchecked(
                self.num_positions() - position - bytes.len(),
                bytes.iter().copied().rev().map(map),
            )
            .await
        }

        pub async fn display_slice_rev_dotted_mapped(
            &mut self,
            position: usize,
            bytes: &[u8],
            dots: &[bool],
            map: impl FnMut(u8) -> u8,
        ) -> Result<(), Error<ERR>> {
            let bytes = Self::rev_bytes(bytes, position);

            self.display_unchecked(
                self.num_positions() - position - bytes.len(),
                crate::mappings::zip_dots(
                    bytes.iter().copied().rev().map(map),
                    dots.iter().copied(),
                ),
            )
            .await
        }

        /// Write the given `bytes` to the display in reversed order starting from `position`.
        ///
        /// See [`TM1637::display_slice_rev_mapped`].
        pub async fn display_slice_rev(
            &mut self,
            position: usize,
            bytes: &[u8],
        ) -> Result<(), Error<ERR>> {
            self.display_slice_rev_mapped(position, bytes, Identity::identity)
                .await
        }

        /// Write the given `bytes` to a `flipped` display starting from `position`.
        ///
        /// See [`TM1637::display_slice_flipped_mapped`].
        pub async fn display_slice_flipped(
            &mut self,
            position: usize,
            bytes: &[u8],
        ) -> Result<(), Error<ERR>> {
            self.display_slice_flipped_mapped(position, bytes, Identity::identity)
                .await
        }

        /// Write the given `bytes` to a `flipped` display starting from `position` mapping each byte using the provided `map` function.
        ///
        /// See [`TM1637::display_slice_rev_mapped`] and [`crate::mappings::flip_mirror`]
        pub async fn display_slice_flipped_mapped(
            &mut self,
            position: usize,
            bytes: &[u8],
            mut map: impl FnMut(u8) -> u8,
        ) -> Result<(), Error<ERR>> {
            self.display_slice_rev_mapped(position, bytes, |byte| {
                crate::mappings::flip_mirror(map(byte))
            })
            .await
        }

        pub async fn display_slice_flipped_dotted_mapped(
            &mut self,
            position: usize,
            bytes: &[u8],
            dots: &[bool],
            mut map: impl FnMut(u8) -> u8,
        ) -> Result<(), Error<ERR>> {
            self.display_slice_rev_dotted_mapped(position, bytes, dots, |byte| {
                crate::mappings::flip_mirror(map(byte))
            })
            .await
        }

        /// Move the given `bytes` in `direction` across the display starting and ending at `position`.
        ///
        /// See [`TM1637::move_slice_overlapping_mapped`].
        pub async fn move_slice_overlapping(
            &mut self,
            position: usize,
            bytes: &[u8],
            delay_ms: u32,
            direction: Direction,
        ) -> Result<(), Error<ERR>> {
            self.move_slice_overlapping_mapped(
                position,
                bytes,
                delay_ms,
                direction,
                Identity::identity,
            )
            .await
        }

        /// Move the given `bytes` in `direction` across the display starting and ending at `position` mapping each byte using the provided `map` function.
        ///
        /// See [`TM1637::display_slice_mapped_unchecked`].
        pub async fn move_slice_overlapping_mapped(
            &mut self,
            position: usize,
            bytes: &[u8],
            delay_ms: u32,
            direction: Direction,
            map: impl FnMut(u8) -> u8 + Clone,
        ) -> Result<(), Error<ERR>> {
            for window in crate::mappings::windows_overlapping::<N>(bytes, direction) {
                self.display_slice_mapped_unchecked(position, &window, map.clone())
                    .await?;

                self.delay.delay_ms(delay_ms).await;
            }

            Ok(())
        }

        pub async fn move_slice_overlapping_dotted_mapped(
            &mut self,
            position: usize,
            bytes: &[u8],
            dots: &[bool],
            delay_ms: u32,
            direction: Direction,
            map: impl FnMut(u8) -> u8 + Clone,
        ) -> Result<(), Error<ERR>> {
            for window in crate::mappings::windows_overlapping::<N>(bytes, direction) {
                self.display_slice_mapped_dotted_unchecked(position, &window, dots, map.clone())
                    .await?;

                self.delay.delay_ms(delay_ms).await;
            }

            Ok(())
        }

        /// Move the given `bytes` in `direction` across a `flipped` display starting and ending at `position`.
        ///
        /// See [`TM1637::move_slice_overlapping_flipped_mapped`].
        pub async fn move_slice_overlapping_flipped(
            &mut self,
            position: usize,
            bytes: &[u8],
            delay_ms: u32,
            direction: Direction,
        ) -> Result<(), Error<ERR>> {
            self.move_slice_overlapping_flipped_mapped(
                position,
                bytes,
                delay_ms,
                direction,
                Identity::identity,
            )
            .await
        }

        /// Move the given `bytes` in `direction` across a `flipped` display starting and ending at `position` mapping each byte using the provided `map` function.
        ///
        /// See [`TM1637::display_slice_flipped_mapped`].
        pub async fn move_slice_overlapping_flipped_mapped(
            &mut self,
            position: usize,
            bytes: &[u8],
            delay_ms: u32,
            direction: Direction,
            map: impl FnMut(u8) -> u8 + Clone,
        ) -> Result<(), Error<ERR>> {
            for window in crate::mappings::windows_overlapping::<N>(bytes, direction) {
                self.display_slice_flipped_mapped(position, &window, map.clone())
                    .await?;

                self.delay.delay_ms(delay_ms).await;
            }

            Ok(())
        }

        pub async fn move_slice_overlapping_flipped_dotted_mapped(
            &mut self,
            position: usize,
            bytes: &[u8],
            dots: &[bool],
            delay_ms: u32,
            direction: Direction,
            map: impl FnMut(u8) -> u8 + Clone,
        ) -> Result<(), Error<ERR>> {
            for window in crate::mappings::windows_overlapping::<N>(bytes, direction) {
                self.display_slice_flipped_dotted_mapped(position, &window, dots, map.clone())
                    .await?;

                self.delay.delay_ms(delay_ms).await;
            }

            Ok(())
        }

        /// Move the given `bytes` in `direction` across the display starting from `position`.
        ///
        /// See [`TM1637::move_slice_to_end_mapped`].
        pub async fn move_slice_to_end(
            &mut self,
            position: usize,
            bytes: &[u8],
            delay_ms: u32,
            direction: Direction,
        ) -> Result<(), Error<ERR>> {
            self.move_slice_to_end_mapped(position, bytes, delay_ms, direction, Identity::identity)
                .await
        }

        /// Move the given `bytes` in `direction` across the display starting from `position` mapping each byte using the provided `map` function.
        ///
        /// See [`TM1637::display_slice_mapped_unchecked`].
        pub async fn move_slice_to_end_mapped(
            &mut self,
            position: usize,
            bytes: &[u8],
            delay_ms: u32,
            direction: Direction,
            map: impl FnMut(u8) -> u8 + Clone,
        ) -> Result<(), Error<ERR>> {
            if bytes.len() <= N {
                return self
                    .display_slice_mapped_unchecked(position, bytes, map)
                    .await;
            }

            for window in crate::mappings::windows::<N>(bytes, direction) {
                self.display_slice_mapped_unchecked(position, window, map.clone())
                    .await?;

                self.delay.delay_ms(delay_ms).await;
            }

            Ok(())
        }

        pub async fn move_slice_to_end_dotted_mapped(
            &mut self,
            position: usize,
            bytes: &[u8],
            dots: &[bool],
            delay_ms: u32,
            direction: Direction,
            map: impl FnMut(u8) -> u8 + Clone,
        ) -> Result<(), Error<ERR>> {
            if bytes.len() <= N {
                return self
                    .display_slice_mapped_dotted_unchecked(position, bytes, dots, map)
                    .await;
            }

            for window in crate::mappings::windows::<N>(bytes, direction) {
                self.display_slice_mapped_dotted_unchecked(position, window, dots, map.clone())
                    .await?;

                self.delay.delay_ms(delay_ms).await;
            }

            Ok(())
        }

        /// Move the given `bytes` in `direction` across a `flipped` display starting from `position`.
        ///
        /// See [`TM1637::move_slice_to_end_flipped_mapped`].
        pub async fn move_slice_to_end_flipped(
            &mut self,
            position: usize,
            bytes: &[u8],
            delay_ms: u32,
            direction: Direction,
        ) -> Result<(), Error<ERR>> {
            self.move_slice_to_end_flipped_mapped(
                position,
                bytes,
                delay_ms,
                direction,
                Identity::identity,
            )
            .await
        }

        /// Move the given `bytes` in `direction` across a `flipped` display starting from `position` mapping each byte using the provided `map` function.
        ///
        /// See [`TM1637::display_slice_flipped_mapped`].
        pub async fn move_slice_to_end_flipped_mapped(
            &mut self,
            position: usize,
            bytes: &[u8],
            delay_ms: u32,
            direction: Direction,
            map: impl FnMut(u8) -> u8 + Clone,
        ) -> Result<(), Error<ERR>> {
            if bytes.len() <= N {
                return self
                    .display_slice_flipped_mapped(position, bytes, map)
                    .await;
            }

            for window in crate::mappings::windows::<N>(bytes, direction) {
                self.display_slice_flipped_mapped(position, window, map.clone())
                    .await?;

                self.delay.delay_ms(delay_ms).await;
            }

            Ok(())
        }

        pub async fn move_slice_to_end_flipped_dotted_mapped(
            &mut self,
            position: usize,
            bytes: &[u8],
            dots: &[bool],
            delay_ms: u32,
            direction: Direction,
            map: impl FnMut(u8) -> u8 + Clone,
        ) -> Result<(), Error<ERR>> {
            if bytes.len() <= N {
                return self
                    .display_slice_flipped_dotted_mapped(position, bytes, dots, map)
                    .await;
            }

            for window in crate::mappings::windows::<N>(bytes, direction) {
                self.display_slice_flipped_dotted_mapped(position, window, dots, map.clone())
                    .await?;

                self.delay.delay_ms(delay_ms).await;
            }

            Ok(())
        }

        /// Write the given `str` to the display starting from `position` mapping each byte using [`from_ascii_byte`](crate::mappings::from_ascii_byte).
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
        /// tm.display_str(0, "Err").ok();
        /// ```
        ///
        /// On a `4-digit display`, this will look like this:
        ///
        /// ```text
        /// +---+ +---+ +---+ +---+
        /// | E | | r | | r | |   |
        /// +---+ +---+ +---+ +---+
        /// ```
        pub async fn display_str(&mut self, position: usize, str: &str) -> Result<(), Error<ERR>> {
            self.display(
                position,
                str.as_bytes()
                    .iter()
                    .copied()
                    .map(crate::mappings::from_ascii_byte),
            )
            .await
        }

        /// Write the given `bytes` to a `flipped` display starting from `position` mapping each byte using [`from_ascii_byte`](crate::mappings::from_ascii_byte).
        ///
        /// See [`TM1637::display_slice_rev_mapped`] and [`crate::mappings::flip_mirror`].
        pub async fn display_str_flipped(
            &mut self,
            position: usize,
            str: &str,
        ) -> Result<(), Error<ERR>> {
            self.display_slice_rev_mapped(position, str.as_bytes(), |byte| {
                crate::mappings::flip_mirror(crate::mappings::from_ascii_byte(byte))
            })
            .await
        }

        /// Move the given `str` in `direction` across the display starting and ending at `position` mapping each byte using [`from_ascii_byte`](crate::mappings::from_ascii_byte).
        ///
        /// See [`TM1637::move_slice_overlapping_mapped`].
        ///
        /// # Example
        ///
        /// Move the string `"HELLO "` on a `4-digit display`:
        ///
        /// ```rust, ignore
        /// let mut tm = TM1637Builder::new(clk_pin, dio_pin, delay)
        ///    .brightness(Brightness::L3)
        ///    .build::<4>();
        ///
        /// tm.init().ok();
        ///
        /// tm.move_slice_overlapping_mapped(0, "HELLO ", 500, Direction::LeftToRight).ok();
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
        pub async fn move_str_overlapping(
            &mut self,
            position: usize,
            str: &str,
            delay_ms: u32,
            direction: Direction,
        ) -> Result<(), Error<ERR>> {
            self.move_slice_overlapping_mapped(
                position,
                str.as_bytes(),
                delay_ms,
                direction,
                crate::mappings::from_ascii_byte,
            )
            .await
        }

        /// Move the given `str` in `direction` across a `flipped` display starting and ending at `position` mapping each byte using [`from_ascii_byte`](crate::mappings::from_ascii_byte).
        ///
        /// See [`TM1637::move_slice_overlapping_flipped_mapped`].
        pub async fn move_str_overlapping_flipped(
            &mut self,
            position: usize,
            str: &str,
            delay_ms: u32,
            direction: Direction,
        ) -> Result<(), Error<ERR>> {
            self.move_slice_overlapping_flipped_mapped(
                position,
                str.as_bytes(),
                delay_ms,
                direction,
                crate::mappings::from_ascii_byte,
            )
            .await
        }

        /// Move the given `str` in `direction` across the display starting from `position` mapping each byte using [`from_ascii_byte`](crate::mappings::from_ascii_byte).
        ///
        /// See [`TM1637::move_slice_to_end_mapped`].
        pub async fn move_str_to_end(
            &mut self,
            position: usize,
            str: &str,
            delay_ms: u32,
            direction: Direction,
        ) -> Result<(), Error<ERR>> {
            self.move_slice_to_end_mapped(
                position,
                str.as_bytes(),
                delay_ms,
                direction,
                crate::mappings::from_ascii_byte,
            )
            .await
        }

        /// Move the given `str` in `direction` across a `flipped` display starting from `position` mapping each byte using [`from_ascii_byte`](crate::mappings::from_ascii_byte).
        ///
        /// See [`TM1637::move_slice_to_end_flipped_mapped`].
        pub async fn move_str_to_end_flipped(
            &mut self,
            position: usize,
            str: &str,
            delay_ms: u32,
            direction: Direction,
        ) -> Result<(), Error<ERR>> {
            self.move_slice_to_end_flipped_mapped(
                position,
                str.as_bytes(),
                delay_ms,
                direction,
                crate::mappings::from_ascii_byte,
            )
            .await
        }

        /// High-level API for static or animated display operations.
        pub fn options(&mut self) -> InitDisplayOptions<'_, N, Token, CLK, DIO, DELAY> {
            InitDisplayOptions { device: self }
        }
    }
}
