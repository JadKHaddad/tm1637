use crate::formatters::clock_to_4digits;

use super::DisplayOptions;

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
    pub fn new(options: DisplayOptions<'d, N, T, CLK, DIO, DELAY, I, M>) -> Self {
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
