use crate::{
    exact_size::ExactSizeChainExt,
    formatters::clock_to_4digits,
    mappings::{from_ascii_byte, RotatingCircleBits, SegmentBits},
    numbers,
    rotating_circle::RotatingStyle,
    scroll::{ScrollDirection, ScrollStyle},
    str_parser::StrParser,
    tokens::{Flipped, NotFlipped},
    windows::windows,
    MaybeFlipped, TM1637,
};

// TODO: seperate the options into modules and use the dublicated stuff only for functions that uses the display. See Display options for example.

/// High-level API for display operations.
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct DisplayOptions<'d, const N: usize, T, CLK, DIO, DELAY, I, M> {
    pub(crate) device: &'d mut TM1637<N, T, CLK, DIO, DELAY>,
    pub(crate) position: usize,
    pub(crate) iter: I,
    pub(crate) _flip: M,
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
pub struct ClockDisplayOptions<'d, const N: usize, T, CLK, DIO, DELAY, I, M> {
    options: DisplayOptions<'d, N, T, CLK, DIO, DELAY, I, M>,
    hour: u8,
    minute: u8,
}

impl<'d, 'b, const N: usize, T, CLK, DIO, DELAY, I, M>
    DisplayOptions<'d, N, T, CLK, DIO, DELAY, I, M>
{
    /// Set the position on the display from which to start displaying the bytes.
    pub const fn position(mut self, position: usize) -> Self {
        self.position = position;
        self
    }

    /// Add a slice of bytes.
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
    >
    where
        I: DoubleEndedIterator<Item = u8> + ExactSizeIterator + 'b,
    {
        DisplayOptions {
            device: self.device,
            position: 0,
            iter: self.iter.exact_size_chain(bytes.iter().copied()),
            _flip: NotFlipped,
        }
    }

    /// Add a string.
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
        M,
    >
    where
        I: DoubleEndedIterator<Item = u8> + ExactSizeIterator + 'b,
    {
        DisplayOptions {
            device: self.device,
            position: self.position,
            iter: self.iter.exact_size_chain(StrParser::new(str)),
            _flip: self._flip,
        }
    }

    /// Add an iterator of bytes.
    ///
    /// # Example
    ///
    /// Manually map each byte in a slice into a human readable character and set the dot at the 2nd position.
    ///
    /// ```rust
    /// use tm1637_embedded_hal::{mappings::{from_ascii_byte, SegmentBits}, mock::Noop, tokens::Blocking, TM1637Builder};
    ///
    /// let mut tm = TM1637Builder::new(Noop, Noop, Noop).build::<4, Blocking>();
    ///
    /// tm.options()
    ///     .iter(
    ///         b"HELLO"
    ///             .iter()
    ///             .copied()
    ///             .map(from_ascii_byte)
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
    ///
    /// // Equivalent to
    ///
    /// tm.options()
    ///    .str("HELLO")
    ///    .dot(1)
    ///    .display()
    ///    .ok();
    /// ```
    pub fn iter<It>(
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
        M,
    >
    where
        I: DoubleEndedIterator<Item = u8> + ExactSizeIterator,
        It: DoubleEndedIterator<Item = u8> + ExactSizeIterator,
    {
        DisplayOptions {
            device: self.device,
            position: 0,
            iter: self.iter.exact_size_chain(iter),
            _flip: self._flip,
        }
    }

    /// Prepare to display a digital clock.
    pub fn clock(self) -> ClockDisplayOptions<'d, N, T, CLK, DIO, DELAY, I, M> {
        ClockDisplayOptions::new(self)
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

    /// Use repeat animation options.
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
    /// # Example
    ///
    /// Manually map each byte in a slice into a human readable character.
    ///
    /// ```rust
    /// use tm1637_embedded_hal::{mappings::from_ascii_byte, mock::Noop, tokens::Blocking, TM1637Builder};
    ///
    /// let mut tm = TM1637Builder::new(Noop, Noop, Noop).build::<4, Blocking>();
    ///
    /// tm.options()
    ///     .slice(b"HELLO")
    ///     .map(from_ascii_byte)
    ///     .display()
    ///     .ok();
    ///
    /// // Equivalent to
    ///
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
        impl MaybeFlipped<N>,
    >
    where
        I: DoubleEndedIterator<Item = u8> + ExactSizeIterator,
        M: MaybeFlipped<N>,
    {
        DisplayOptions {
            device: self.device,
            position: self.position,
            iter: self.iter,
            _flip: M::flip(),
        }
    }
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

    /// Finish setting the scroll animation.
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
            windows::<N>(self.options.iter, self.direction, self.style).map(|i| i.into_iter());

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

    /// Finish setting the repeat animation.
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

    /// Flip the display.
    pub fn flip(self) -> RotatingCircleOptions<'d, N, T, CLK, DIO, DELAY, Flipped> {
        RotatingCircleOptions {
            device: self.device,
            position: self.position,
            delay_ms: self.delay_ms,
            style: self.style,
            _flip: Flipped,
        }
    }

    /// Finish setting the rotating circle animation.
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

    use crate::{
        align::{Align, Aligned},
        ConditionalInputPin, DisplayOptions, Error, Identity, MaybeFlipped,
    };

    use super::Scroller;

    #[::duplicate::duplicate_item(
        NUM_POS ;
        [4] ;
        [6] ;
    )]
    impl<CLK, DIO, DELAY, ERR, I, M> DisplayOptions<'_, NUM_POS, Token, CLK, DIO, DELAY, I, M>
    where
        CLK: OutputPin<Error = ERR>,
        DIO: OutputPin<Error = ERR> + ConditionalInputPin<ERR>,
        DELAY: DelayTrait,
        I: DoubleEndedIterator<Item = u8> + ExactSizeIterator,
        M: MaybeFlipped<NUM_POS>,
    {
        /// Release the `device` and return the calculated position and bytes.
        pub fn calculate(self) -> (usize, impl Iterator<Item = u8>) {
            let (position, bytes) = M::calculate(self.position, self.iter);

            Align::<NUM_POS>::align(position, bytes)
        }

        /// Display the bytes on a `flipped` or `non-flipped` display.
        pub async fn display(self) -> Result<(), Error<ERR>> {
            let (position, bytes) = M::calculate(self.position, self.iter);

            let (position, bytes) = Align::<NUM_POS>::align(position, bytes);

            self.device.display(position, bytes).await
        }
    }

    #[::duplicate::duplicate_item(
        NUM_POS ;
        [4] ;
        [6] ;
    )]
    impl<'d, CLK, DIO, DELAY, ERR, I, M, InI> Scroller<'d, NUM_POS, Token, CLK, DIO, DELAY, I, M>
    where
        ERR: 'd,
        CLK: OutputPin<Error = ERR>,
        DIO: OutputPin<Error = ERR> + ConditionalInputPin<ERR>,
        DELAY: DelayTrait,
        I: Iterator<Item = InI> + 'd,
        InI: DoubleEndedIterator<Item = u8> + ExactSizeIterator + 'd,
        M: MaybeFlipped<NUM_POS> + 'd,
    {
        fn _calculate(
            position: usize,
            iter: I,
            inner_iter_len: usize,
        ) -> (usize, impl Iterator<Item = impl Iterator<Item = u8>>) {
            let original_position = position;

            let iter = iter.map(move |item| {
                let (position, bytes) = M::calculate(original_position, item.into_iter());
                let (_, bytes) = Align::<NUM_POS>::align(position, bytes);

                bytes
            });

            let position = M::position(original_position, inner_iter_len);
            let position = Align::<NUM_POS>::position(position);

            (position, iter)
        }

        /// Release the `device` and return the calculated position and bytes.
        pub fn calculate(self) -> (usize, impl Iterator<Item = impl Iterator<Item = u8>>) {
            Self::_calculate(self.position, self.iter, self.inner_iter_len)
        }

        /// Return the scroll animation as an iterator.
        pub fn steps(self) -> impl ScrollIter<Item = Result<(), Error<ERR>>> + 'd {
            let (position, iter) = Self::_calculate(self.position, self.iter, self.inner_iter_len);

            self.device.scroll(position, self.delay_ms, iter)
        }

        /// Run the scroll animation and return the number of steps.
        pub async fn run(self) -> usize {
            self.steps().count().await
        }
    }
}

#[::duplicate::duplicate_item(
    function    type_;
    [u8]        [u8];
    [u8_2]      [u8];
    [r_u8_2]    [u8];
    [u16_3]     [u16];
    [r_u16_3]   [u16];
    [u16_4]     [u16];
    [r_u16_4]   [u16];
    [u32_5]     [u32];
    [r_u32_5]   [u32];
    [u32_6]     [u32];
    [r_u32_6]   [u32];
    [u32_7]     [u32];
    [r_u32_7]   [u32];
    [u32_8]     [u32];
    [r_u32_8]   [u32];
    [i8_2]      [i8];
    [i16_3]     [i16];
    [r_i16_3]   [i16];
    [i16_4]     [i16];
    [r_i16_4]   [i16];
)]
impl<'d, const N: usize, T, CLK, DIO, DELAY, I, M> DisplayOptions<'d, N, T, CLK, DIO, DELAY, I, M>
where
    I: DoubleEndedIterator<Item = u8> + ExactSizeIterator,
{
    pub fn function(
        self,
        n: type_,
    ) -> DisplayOptions<
        'd,
        N,
        T,
        CLK,
        DIO,
        DELAY,
        impl DoubleEndedIterator<Item = u8> + ExactSizeIterator,
        M,
    > {
        DisplayOptions {
            device: self.device,
            position: self.position,
            iter: self.iter.exact_size_chain(numbers::function(n).into_iter()),
            _flip: self._flip,
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate std;
    use std::vec;
    use std::vec::Vec;

    use crate::{mappings::str_from_byte, mock::Noop, TM1637Builder};

    #[test]
    fn dot_is_dynamically_tied_to_byte() {
        let mut tm = TM1637Builder::new(Noop, Noop, Noop).build_blocking::<4>();

        let (_, iter) = tm.options().str("HELLO").dot(1).dot(3).calculate();
        let collected = iter.map(str_from_byte).collect::<Vec<_>>();

        assert_eq!(vec!["H", "E.", "L", "L."], collected);

        let (_, iter) = tm.options().str("HELLO").dot(1).dot(3).flip().calculate();
        let collected = iter.map(str_from_byte).collect::<Vec<_>>();

        assert_eq!(vec!["7.", "7", "3.", "H"], collected);
    }
}
