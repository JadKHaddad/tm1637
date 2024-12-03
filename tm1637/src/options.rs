use crate::{
    formatters::clock_to_4digits,
    mappings::from_ascii_byte,
    scroll::{ScrollDirection, ScrollStyle},
    tokens::NotFlipped,
    TM1637,
};

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

    /// Prepare to display a digital clock.
    pub fn clock(self) -> ClockDisplayOptions<'d, N, T, CLK, DIO, DELAY> {
        ClockDisplayOptions::new(self.device)
    }

    /// Prepare to display a loading animation.
    pub fn loading() {
        // TODO
        unimplemented!()
    }

    // TODO: all formatters go here
}

/// High-level API for display operations.
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct DisplayOptions<'d, const N: usize, T, CLK, DIO, DELAY, F, M> {
    device: &'d mut TM1637<N, T, CLK, DIO, DELAY>,
    position: usize,
    iter: F,
    _flip: M,
}

/// High-level API for scroll animations.
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ScrollDisplayOptions<'d, const N: usize, T, CLK, DIO, DELAY, F, D> {
    options: DisplayOptions<'d, N, T, CLK, DIO, DELAY, F, D>,
    delay_ms: u32,
    direction: ScrollDirection,
    style: ScrollStyle,
}

#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct RepeatDisplayOptions<'d, const N: usize, T, CLK, DIO, DELAY, F, D> {
    options: DisplayOptions<'d, N, T, CLK, DIO, DELAY, F, D>,
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
    dot: bool,
}

impl<'d, const N: usize, T, CLK, DIO, DELAY> ClockDisplayOptions<'d, N, T, CLK, DIO, DELAY> {
    pub fn new(device: &'d mut TM1637<N, T, CLK, DIO, DELAY>) -> Self {
        Self {
            device,
            hour: 0,
            minute: 0,
            dot: false,
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

    pub const fn dot(mut self, dot: bool) -> Self {
        self.dot = dot;
        self
    }

    pub const fn set_dot(mut self) -> Self {
        self.dot = true;
        self
    }

    pub const fn unset_dot(mut self) -> Self {
        self.dot = false;
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
            iter: clock_to_4digits(self.hour, self.minute, self.dot).into_iter(),
            _flip: NotFlipped,
        }
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
        scroll::{ScrollDirection, ScrollStyle},
        tokens::Flipped,
        windows::windows_iter,
        ConditionalInputPin, DisplayOptions, Error, Identity, MaybeFlipped, ScrollDisplayOptions,
    };

    use super::{RepeatDisplayOptions, Scroller};

    impl<'d, const N: usize, CLK, DIO, DELAY, ERR, F, M>
        DisplayOptions<'d, N, Token, CLK, DIO, DELAY, F, M>
    where
        CLK: OutputPin<Error = ERR>,
        DIO: OutputPin<Error = ERR> + ConditionalInputPin<ERR>,
        DELAY: DelayTrait,
        F: DoubleEndedIterator<Item = u8> + ExactSizeIterator,
        M: MaybeFlipped<N>,
    {
        /// Set the position on the display from which to start displaying the bytes.
        pub const fn position(mut self, position: usize) -> Self {
            self.position = position;
            self
        }

        /// Display the bytes on a `flipped` or `non-flipped` display.
        pub async fn display(self) -> Result<(), Error<ERR>> {
            let (position, bytes) = M::calculate(self.position, self.iter);

            self.device.display(position, bytes).await
        }

        /// Use scroll animation options.
        pub const fn scroll(self) -> ScrollDisplayOptions<'d, N, Token, CLK, DIO, DELAY, F, M> {
            ScrollDisplayOptions {
                options: self,
                delay_ms: 500,
                direction: ScrollDirection::LeftToRight,
                style: ScrollStyle::Circular,
            }
        }

        pub const fn repeat(self) -> RepeatDisplayOptions<'d, N, Token, CLK, DIO, DELAY, F, M> {
            RepeatDisplayOptions {
                options: self,
                delay_ms: 500,
            }
        }

        /// Flip the display.
        pub fn flip(
            self,
        ) -> DisplayOptions<
            'd,
            N,
            Token,
            CLK,
            DIO,
            DELAY,
            impl DoubleEndedIterator<Item = u8> + ExactSizeIterator,
            Flipped,
        > {
            DisplayOptions {
                device: self.device,
                position: self.position,
                iter: self.iter,
                _flip: Flipped,
            }
        }
    }

    impl<'d, const N: usize, CLK, DIO, DELAY, ERR, I, M, InI>
        Scroller<'d, N, Token, CLK, DIO, DELAY, I, M>
    where
        CLK: OutputPin<Error = ERR>,
        DIO: OutputPin<Error = ERR> + ConditionalInputPin<ERR>,
        DELAY: DelayTrait,
        I: Iterator<Item = InI> + 'd,
        InI: DoubleEndedIterator<Item = u8> + ExactSizeIterator,
        M: MaybeFlipped<N>,
    {
        pub fn steps(self) -> impl ScrollIter<Item = Result<(), Error<ERR>>> + 'd {
            let original_position = self.position;

            let iter = self.iter.map(move |item| {
                let (_, bytes) = M::calculate(original_position, item.into_iter());

                bytes
            });

            let position = M::position(original_position, self.inner_iter_len);
            self.device.scroll(position, self.delay_ms, iter)
        }

        pub async fn run(self) -> usize {
            self.steps().count().await
        }
    }

    impl<'d, const N: usize, CLK, DIO, DELAY, ERR, F, M>
        ScrollDisplayOptions<'d, N, Token, CLK, DIO, DELAY, F, M>
    where
        CLK: OutputPin<Error = ERR>,
        DIO: OutputPin<Error = ERR> + ConditionalInputPin<ERR>,
        DELAY: DelayTrait,
        F: DoubleEndedIterator<Item = u8> + ExactSizeIterator,
        M: MaybeFlipped<N>,
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
            Token,
            CLK,
            DIO,
            DELAY,
            impl Iterator<Item = impl DoubleEndedIterator<Item = u8> + ExactSizeIterator>,
            M,
        > {
            let iter = windows_iter::<N>(self.options.iter, self.direction, self.style)
                .map(|i| i.into_iter());

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

    impl<'d, const N: usize, CLK, DIO, DELAY, ERR, F, M>
        RepeatDisplayOptions<'d, N, Token, CLK, DIO, DELAY, F, M>
    where
        CLK: OutputPin<Error = ERR>,
        DIO: OutputPin<Error = ERR> + ConditionalInputPin<ERR>,
        DELAY: DelayTrait,
        F: DoubleEndedIterator<Item = u8> + ExactSizeIterator,
        M: MaybeFlipped<N>,
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
            Token,
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
}
