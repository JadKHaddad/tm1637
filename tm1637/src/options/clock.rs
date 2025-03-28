use crate::formatters::clock_to_4digits;

use super::DisplayOptions;

/// High-level API for setting a clock.
///
/// # Example
///
/// Display the time `14:28` on the display.
///
/// ```rust
/// use tm1637_embedded_hal::{mock::Noop, TM1637Builder};
///
/// let mut tm = TM1637Builder::new(Noop, Noop, Noop).build_blocking::<4>();
///
/// tm.options()
///     .clock()
///     .hour(14)
///     .minute(28)
///     .finish()
///     // Set the colon between the hours and minutes.
///     .dot(1)
///     .display()
///     .ok();
/// ```
///
/// The display will show:
///
/// ```text
/// +---+ +---+ +---+ +---+
/// | 1 | | 4 |:| 2 | | 8 |
/// +---+ +---+ +---+ +---+
/// ```
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ClockDisplayOptions<'d, const N: usize, T, CLK, DIO, DELAY, I, M> {
    options: DisplayOptions<'d, N, T, CLK, DIO, DELAY, I, M>,
    hour: u8,
    minute: u8,
}

impl<'d, const N: usize, T, CLK, DIO, DELAY, I, M>
    ClockDisplayOptions<'d, N, T, CLK, DIO, DELAY, I, M>
{
    /// Create a new [`ClockDisplayOptions`] instance.
    pub const fn new(options: DisplayOptions<'d, N, T, CLK, DIO, DELAY, I, M>) -> Self {
        Self {
            options,
            hour: 0,
            minute: 0,
        }
    }

    /// Set the hour.
    pub const fn hour(mut self, hour: u8) -> Self {
        self.hour = hour;
        self
    }

    /// Set the minute.
    pub const fn minute(mut self, minute: u8) -> Self {
        self.minute = minute;
        self
    }

    /// Finish setting the clock.
    pub fn finish(
        self,
    ) -> DisplayOptions<
        'd,
        N,
        T,
        CLK,
        DIO,
        DELAY,
        impl DoubleEndedIterator<Item = u8> + ExactSizeIterator,
        M,
    >
    where
        I: DoubleEndedIterator<Item = u8> + ExactSizeIterator,
    {
        self.options
            .iter(clock_to_4digits(self.hour, self.minute, false).into_iter())
    }
}
