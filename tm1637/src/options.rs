use crate::{
    formatters::clock_to_4digits,
    mappings::{from_ascii_byte, RotatingCircleBits, SegmentBits},
    rotating_circle::RotatingStyle,
    scroll::{ScrollDirection, ScrollStyle},
    tokens::{Flipped, NotFlipped},
    windows::windows_iter,
    TM1637,
};

// TODO: seperate the options into modules and use the dublicated stuff only for functions that uses the display. See Display options for example.

/// Starting point for a High-level API for display operations.
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct InitDisplayOptions<'d, const N: usize, T, CLK, DIO, DELAY> {
    device: &'d mut TM1637<N, T, CLK, DIO, DELAY>,
}

impl<'d, 'b, const N: usize, T, CLK, DIO, DELAY> InitDisplayOptions<'d, N, T, CLK, DIO, DELAY> {
    pub fn new(device: &'d mut TM1637<N, T, CLK, DIO, DELAY>) -> Self {
        Self { device }
    }

    /// Prepare to display a slice of bytes.
    pub fn slice(
        self,
        bytes: &'b [u8],
    ) -> DisplayOptions<
        'd,
        N,
        T,
        CLK,
        DIO,
        DELAY,
        impl DoubleEndedIterator<Item = u8> + ExactSizeIterator + 'b,
        NotFlipped,
    > {
        DisplayOptions {
            device: self.device,
            position: 0,
            iter: bytes.iter().copied(),
            _flip: NotFlipped,
        }
    }

    /// Prepare to display a string.
    pub fn str(
        self,
        str: &'b str,
    ) -> DisplayOptions<
        'd,
        N,
        T,
        CLK,
        DIO,
        DELAY,
        impl DoubleEndedIterator<Item = u8> + ExactSizeIterator + 'b,
        NotFlipped,
    > {
        DisplayOptions {
            device: self.device,
            position: 0,
            iter: str.as_bytes().iter().copied().map(from_ascii_byte),
            _flip: NotFlipped,
        }
    }

    /// Prepare to display an iterator of bytes.
    ///
    /// # Example
    ///
    /// Manually map each byte in a slice into a human readable character and set the dot at the 2nd position.
    ///
    /// ```rust, ignore
    /// tm.options()
    ///     .iter(
    ///         b"HELLO"
    ///             .iter()
    ///             .copied()
    ///             .map(mappings::from_ascii_byte)
    ///             .enumerate()
    ///             .map(move |(i, b)| {
    ///                 if i == 1 {
    ///                     b | SegmentBits::Dot as u8
    ///                 } else {
    ///                     b
    ///                 }
    ///             }),
    ///     )
    ///     .display()
    ///     .ok();
    /// ```
    ///
    /// This example is equivalent to
    ///
    /// ```rust, ignore
    /// tm.options()
    ///    .str("HELLO")
    ///    .dot(1)
    ///    .display()
    ///    .ok();
    /// ```
    pub fn iter<It: DoubleEndedIterator<Item = u8> + ExactSizeIterator>(
        self,
        iter: It,
    ) -> DisplayOptions<
        'd,
        N,
        T,
        CLK,
        DIO,
        DELAY,
        impl DoubleEndedIterator<Item = u8> + ExactSizeIterator,
        NotFlipped,
    > {
        DisplayOptions {
            device: self.device,
            position: 0,
            iter,
            _flip: NotFlipped,
        }
    }

    /// Prepare to display a digital clock.
    pub fn clock(self) -> ClockDisplayOptions<'d, N, T, CLK, DIO, DELAY> {
        ClockDisplayOptions::new(self.device)
    }

    /// Prepare to display a rotating circle animation.
    pub fn rotating_circle(self) -> RotatingCircleOptions<'d, N, T, CLK, DIO, DELAY, NotFlipped> {
        RotatingCircleOptions::new(self.device, NotFlipped)
    }

    // TODO: all formatters go here
    // TODO: maybe rotating circle over 2 cells 3 or 4 cells etc..
    //  --- ---
    // |       |
    // |       |
    //  --- ---
    //  --- --- ---
    // |           |
    // |           |
    //  --- --- ---
}

/// High-level API for display operations.
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct DisplayOptions<'d, const N: usize, T, CLK, DIO, DELAY, I, M> {
    device: &'d mut TM1637<N, T, CLK, DIO, DELAY>,
    position: usize,
    pub(crate) iter: I,
    _flip: M,
}

/// High-level API for scroll animations.
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ScrollDisplayOptions<'d, const N: usize, T, CLK, DIO, DELAY, I, D> {
    options: DisplayOptions<'d, N, T, CLK, DIO, DELAY, I, D>,
    delay_ms: u32,
    direction: ScrollDirection,
    style: ScrollStyle,
}

#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct RepeatDisplayOptions<'d, const N: usize, T, CLK, DIO, DELAY, I, D> {
    options: DisplayOptions<'d, N, T, CLK, DIO, DELAY, I, D>,
    delay_ms: u32,
}

#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Scroller<'d, const N: usize, T, CLK, DIO, DELAY, I, M> {
    device: &'d mut TM1637<N, T, CLK, DIO, DELAY>,
    inner_iter_len: usize,
    position: usize,
    delay_ms: u32,
    iter: I,
    _flip: M,
}

#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ClockDisplayOptions<'d, const N: usize, T, CLK, DIO, DELAY> {
    device: &'d mut TM1637<N, T, CLK, DIO, DELAY>,
    hour: u8,
    minute: u8,
}

impl<'d, const N: usize, T, CLK, DIO, DELAY, I, M> DisplayOptions<'d, N, T, CLK, DIO, DELAY, I, M> {
    /// Set the position on the display from which to start displaying the bytes.
    pub const fn position(mut self, position: usize) -> Self {
        self.position = position;
        self
    }

    /// Use scroll animation options.
    pub const fn scroll(self) -> ScrollDisplayOptions<'d, N, T, CLK, DIO, DELAY, I, M> {
        ScrollDisplayOptions {
            options: self,
            delay_ms: 500,
            direction: ScrollDirection::LeftToRight,
            style: ScrollStyle::Circular,
        }
    }

    pub const fn repeat(self) -> RepeatDisplayOptions<'d, N, T, CLK, DIO, DELAY, I, M> {
        RepeatDisplayOptions {
            options: self,
            delay_ms: 500,
        }
    }

    /// Add a dynamic dot to the display at the specified position.
    ///
    /// ## Dynamic
    ///
    /// The dot is tied to the byte at the specified position and will move with it.
    pub fn dot(
        self,
        position: usize,
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
        DisplayOptions {
            device: self.device,
            position: self.position,
            iter: self.iter.enumerate().map(move |(i, b)| {
                if i == position {
                    b | SegmentBits::Dot as u8
                } else {
                    b
                }
            }),
            _flip: self._flip,
        }
    }

    /// Remove the dot from the display at the specified position.
    pub fn remove_dot(
        self,
        position: usize,
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
        DisplayOptions {
            device: self.device,
            position: self.position,
            iter: self.iter.enumerate().map(move |(i, b)| {
                if i == position {
                    b & !(SegmentBits::Dot as u8)
                } else {
                    b
                }
            }),
            _flip: self._flip,
        }
    }

    /// Set the dot at the specified position.
    pub fn set_dot(
        self,
        position: usize,
        dot: bool,
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
        DisplayOptions {
            device: self.device,
            position: self.position,
            iter: self.iter.enumerate().map(move |(i, b)| {
                if i == position {
                    if dot {
                        b | SegmentBits::Dot as u8
                    } else {
                        b & !(SegmentBits::Dot as u8)
                    }
                } else {
                    b
                }
            }),
            _flip: self._flip,
        }
    }

    /// Add dots to all positions in the display.
    pub fn dots(
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
        DisplayOptions {
            device: self.device,
            position: self.position,
            iter: self.iter.map(|b| b | SegmentBits::Dot as u8),
            _flip: self._flip,
        }
    }

    /// Remove dots from all positions in the display.
    pub fn remove_dots(
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
        DisplayOptions {
            device: self.device,
            position: self.position,
            iter: self.iter.map(|b| b & !(SegmentBits::Dot as u8)),
            _flip: self._flip,
        }
    }

    /// Map the bytes using the provided function.
    ///
    ///  # Example
    ///
    /// Manually map each byte in a slice into a human readable character.
    ///
    /// ```rust, ignore
    /// tm.options()
    ///     .slice(b"HELLO")
    ///     .map(mappings::from_ascii_byte)
    ///     .display()
    ///     .ok();
    /// ```
    ///
    /// This example is equivalent to
    ///
    /// ```rust, ignore
    /// tm.options()
    ///    .str("HELLO")
    ///    .display()
    ///    .ok();
    /// ```
    pub fn map<F: FnMut(u8) -> u8>(
        self,
        f: F,
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
        DisplayOptions {
            device: self.device,
            position: self.position,
            iter: self.iter.map(f),
            _flip: self._flip,
        }
    }

    /// Flip the display.
    pub fn flip(
        self,
    ) -> DisplayOptions<
        'd,
        N,
        T,
        CLK,
        DIO,
        DELAY,
        impl DoubleEndedIterator<Item = u8> + ExactSizeIterator,
        Flipped,
    >
    where
        I: DoubleEndedIterator<Item = u8> + ExactSizeIterator,
    {
        DisplayOptions {
            device: self.device,
            position: self.position,
            iter: self.iter,
            _flip: Flipped,
        }
    }
}

impl<'d, const N: usize, T, CLK, DIO, DELAY> ClockDisplayOptions<'d, N, T, CLK, DIO, DELAY> {
    pub fn new(device: &'d mut TM1637<N, T, CLK, DIO, DELAY>) -> Self {
        Self {
            device,
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

    /// Finish setting the clock and display it.
    pub fn finish(
        &mut self,
    ) -> DisplayOptions<
        N,
        T,
        CLK,
        DIO,
        DELAY,
        impl DoubleEndedIterator<Item = u8> + ExactSizeIterator,
        NotFlipped,
    > {
        DisplayOptions {
            device: self.device,
            position: 0,
            // add the dot using the `dot` method on the display options
            iter: clock_to_4digits(self.hour, self.minute, false).into_iter(),
            _flip: NotFlipped,
        }
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct RotatingCircleOptions<'d, const N: usize, T, CLK, DIO, DELAY, M> {
    device: &'d mut TM1637<N, T, CLK, DIO, DELAY>,
    position: usize,
    delay_ms: u32,
    style: RotatingStyle,
    _flip: M,
}

impl<'d, const N: usize, T, CLK, DIO, DELAY, I, M>
    ScrollDisplayOptions<'d, N, T, CLK, DIO, DELAY, I, M>
where
    I: DoubleEndedIterator<Item = u8> + ExactSizeIterator,
{
    /// Set the delay in milliseconds between each animation step.
    pub const fn delay_ms(mut self, delay_ms: u32) -> Self {
        self.delay_ms = delay_ms;
        self
    }

    /// Set the animation direction.
    pub const fn direction(mut self, direction: ScrollDirection) -> Self {
        self.direction = direction;
        self
    }

    /// Set the animation direction to [`ScrollDirection::LeftToRight`].
    pub const fn left(mut self) -> Self {
        self.direction = ScrollDirection::LeftToRight;
        self
    }

    /// Set the animation direction to [`ScrollDirection::RightToLeft`].
    pub const fn right(mut self) -> Self {
        self.direction = ScrollDirection::RightToLeft;
        self
    }

    /// Set the animation style.
    pub const fn style(mut self, style: ScrollStyle) -> Self {
        self.style = style;
        self
    }

    pub fn finish(
        self,
    ) -> Scroller<
        'd,
        N,
        T,
        CLK,
        DIO,
        DELAY,
        impl Iterator<Item = impl DoubleEndedIterator<Item = u8> + ExactSizeIterator>,
        M,
    > {
        let iter =
            windows_iter::<N>(self.options.iter, self.direction, self.style).map(|i| i.into_iter());

        Scroller {
            device: self.options.device,
            inner_iter_len: N,
            position: self.options.position,
            delay_ms: self.delay_ms,
            iter,
            _flip: self.options._flip,
        }
    }
}

impl<'d, const N: usize, T, CLK, DIO, DELAY, I, M>
    RepeatDisplayOptions<'d, N, T, CLK, DIO, DELAY, I, M>
where
    I: DoubleEndedIterator<Item = u8> + ExactSizeIterator,
{
    /// Set the delay in milliseconds between each animation step.
    pub const fn delay_ms(mut self, delay_ms: u32) -> Self {
        self.delay_ms = delay_ms;
        self
    }

    pub fn finish(
        self,
    ) -> Scroller<
        'd,
        N,
        T,
        CLK,
        DIO,
        DELAY,
        impl Iterator<Item = impl DoubleEndedIterator<Item = u8> + ExactSizeIterator>,
        M,
    > {
        let iter = self.options.iter.map(move |i| [i]).map(|i| i.into_iter());

        Scroller {
            device: self.options.device,
            inner_iter_len: 1,
            position: self.options.position,
            delay_ms: self.delay_ms,
            iter,
            _flip: self.options._flip,
        }
    }
}

impl<'d, const N: usize, T, CLK, DIO, DELAY, M>
    RotatingCircleOptions<'d, N, T, CLK, DIO, DELAY, M>
{
    pub fn new(device: &'d mut TM1637<N, T, CLK, DIO, DELAY>, flip: M) -> Self {
        Self {
            device,
            position: 0,
            delay_ms: 500,
            style: RotatingStyle::Clockwise,
            _flip: flip,
        }
    }

    /// Set the starting position of the rotating circle.
    pub const fn position(mut self, position: usize) -> Self {
        self.position = position;
        self
    }

    /// Set the delay in milliseconds between each animation step.
    pub const fn delay_ms(mut self, delay_ms: u32) -> Self {
        self.delay_ms = delay_ms;
        self
    }

    /// Set the rotating style.
    pub const fn style(mut self, style: RotatingStyle) -> Self {
        self.style = style;
        self
    }

    pub fn flip(self) -> RotatingCircleOptions<'d, N, T, CLK, DIO, DELAY, Flipped> {
        RotatingCircleOptions {
            device: self.device,
            position: self.position,
            delay_ms: self.delay_ms,
            style: self.style,
            _flip: Flipped,
        }
    }

    pub fn finish(
        self,
    ) -> Scroller<
        'd,
        N,
        T,
        CLK,
        DIO,
        DELAY,
        impl Iterator<Item = impl DoubleEndedIterator<Item = u8> + ExactSizeIterator>,
        M,
    > {
        let bytes = match self.style {
            RotatingStyle::Clockwise => RotatingCircleBits::all_u8_reversed(),
            RotatingStyle::CounterClockwise => RotatingCircleBits::all_u8(),
        };

        RepeatDisplayOptions {
            options: DisplayOptions {
                device: self.device,
                position: self.position,
                iter: bytes.into_iter(),
                _flip: self._flip,
            },
            delay_ms: self.delay_ms,
        }
        .finish()
    }
}

#[::duplicate::duplicate_item(
    module        async     await               Token                     DelayTrait                             ScrollIter;
    [asynch]      [async]   [await.identity()]  [crate::tokens::Async]    [::embedded_hal_async::delay::DelayNs] [::futures::Stream];
    [blocking]    []        [identity()]        [crate::tokens::Blocking] [::embedded_hal::delay::DelayNs]       [Iterator];
)]
pub mod module {
    use ::embedded_hal::digital::OutputPin;
    use ::futures::StreamExt; // hmm

    use crate::{ConditionalInputPin, DisplayOptions, Error, Identity, MaybeFlipped};

    use super::Scroller;

    impl<const N: usize, CLK, DIO, DELAY, ERR, I, M> DisplayOptions<'_, N, Token, CLK, DIO, DELAY, I, M>
    where
        CLK: OutputPin<Error = ERR>,
        DIO: OutputPin<Error = ERR> + ConditionalInputPin<ERR>,
        DELAY: DelayTrait,
        I: DoubleEndedIterator<Item = u8> + ExactSizeIterator,
        M: MaybeFlipped<N>,
    {
        /// Calculate the position and bytes that will be displayed.
        pub fn calculate(self) -> (usize, impl Iterator<Item = u8>) {
            M::calculate(self.position, self.iter)
        }

        /// Display the bytes on a `flipped` or `non-flipped` display.
        pub async fn display(self) -> Result<(), Error<ERR>> {
            let (position, bytes) = M::calculate(self.position, self.iter);

            self.device.display(position, bytes).await
        }
    }

    impl<'d, const N: usize, CLK, DIO, DELAY, ERR, I, M, InI>
        Scroller<'d, N, Token, CLK, DIO, DELAY, I, M>
    where
        ERR: 'd,
        CLK: OutputPin<Error = ERR>,
        DIO: OutputPin<Error = ERR> + ConditionalInputPin<ERR>,
        DELAY: DelayTrait,
        I: Iterator<Item = InI> + 'd,
        InI: DoubleEndedIterator<Item = u8> + ExactSizeIterator + 'd,
        M: MaybeFlipped<N> + 'd,
    {
        fn _calculate(
            position: usize,
            iter: I,
            inner_iter_len: usize,
        ) -> (usize, impl Iterator<Item = impl Iterator<Item = u8>>) {
            let original_position = position;

            let iter = iter.map(move |item| {
                let (_, bytes) = M::calculate(original_position, item.into_iter());

                bytes
            });

            let position = M::position(original_position, inner_iter_len);

            (position, iter)
        }

        pub fn calculate(self) -> (usize, impl Iterator<Item = impl Iterator<Item = u8>>) {
            Self::_calculate(self.position, self.iter, self.inner_iter_len)
        }

        pub fn steps(self) -> impl ScrollIter<Item = Result<(), Error<ERR>>> + 'd {
            let (position, iter) = Self::_calculate(self.position, self.iter, self.inner_iter_len);

            self.device.scroll(position, self.delay_ms, iter)
        }

        pub async fn run(self) -> usize {
            self.steps().count().await
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate std;
    use std::vec;
    use std::vec::Vec;

    use crate::{mappings::str_from_byte, test::Noop, TM1637Builder};

    #[test]
    fn dot_is_dynamically_tied_to_byte() {
        let mut tm = TM1637Builder::new(Noop, Noop, Noop).build_blocking::<4>();

        let (_, iter) = tm.options().str("HELLO").dot(1).dot(3).calculate();
        let collected = iter.map(str_from_byte).collect::<Vec<_>>();

        assert_eq!(vec!["H", "E.", "L", "L.", "0"], collected);

        let (_, iter) = tm.options().str("HELLO").dot(1).dot(3).flip().calculate();
        let collected = iter.map(str_from_byte).collect::<Vec<_>>();

        assert_eq!(vec!["7.", "7", "3.", "H"], collected);
    }
}
