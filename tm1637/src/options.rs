use crate::{Direction, NotFlipped, StrParser, WindowsStyle, TM1637};

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
    pub fn put_slice(
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
    pub fn put_str(
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
            iter: StrParser::new(str),
            _flip: NotFlipped,
        }
    }

    /// Prepare to display a loading animation.
    pub fn loading() {
        unimplemented!()
    }
}

impl<'d, T, CLK, DIO, DELAY> InitDisplayOptions<'d, 4, T, CLK, DIO, DELAY> {
    /// Prepare to display a digital clock.
    pub fn clock(self) -> ClockDisplayOptions<'d, T, CLK, DIO, DELAY> {
        ClockDisplayOptions::new(self.device)
    }
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

/// High-level API for animations.
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct AnimatedDisplayOptions<'d, const N: usize, T, CLK, DIO, DELAY, F, D> {
    options: DisplayOptions<'d, N, T, CLK, DIO, DELAY, F, D>,
    delay_ms: u32,
    direction: Direction,
    style: WindowsStyle,
}

#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ClockDisplayOptions<'d, T, CLK, DIO, DELAY> {
    device: &'d mut TM1637<4, T, CLK, DIO, DELAY>,
    hour: u8,
    minute: u8,
    dot: bool,
    bytes: [u8; 4],
}

impl<'d, 'b, T, CLK, DIO, DELAY> ClockDisplayOptions<'d, T, CLK, DIO, DELAY> {
    pub fn new(device: &'d mut TM1637<4, T, CLK, DIO, DELAY>) -> Self {
        Self {
            device,
            hour: 0,
            minute: 0,
            dot: false,
            bytes: [0; 4],
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
        &'b mut self,
    ) -> DisplayOptions<
        'd,
        4,
        T,
        CLK,
        DIO,
        DELAY,
        impl DoubleEndedIterator<Item = u8> + ExactSizeIterator + 'b,
        NotFlipped,
    >
    where
        'b: 'd,
    {
        self.bytes = crate::formatters::clock_to_4digits(self.hour, self.minute, self.dot);

        DisplayOptions {
            device: self.device,
            position: 0,
            iter: self.bytes.iter().copied(),
            _flip: NotFlipped,
        }
    }
}

#[::duplicate::duplicate_item(
    module        async     await               Token             DelayTrait                             AnimationIter;
    [asynch]      [async]   [await.identity()]  [crate::Async]    [::embedded_hal_async::delay::DelayNs] [::futures::Stream];
    [blocking]    []        [identity()]        [crate::Blocking] [::embedded_hal::delay::DelayNs]       [Iterator];
)]
pub mod module {
    use ::embedded_hal::digital::OutputPin;
    use ::futures::StreamExt; // hmm

    use crate::{
        mappings::windows_new_api, AnimatedDisplayOptions, ConditionalInputPin, Direction,
        DisplayOptions, Error, Flipped, Identity, MaybeFlipped, WindowsStyle,
    };

    impl<'d, const N: usize, CLK, DIO, DELAY, ERR, F, M>
        DisplayOptions<'d, N, Token, CLK, DIO, DELAY, F, M>
    where
        CLK: OutputPin<Error = ERR>,
        DIO: OutputPin<Error = ERR> + ConditionalInputPin<ERR>,
        DELAY: DelayTrait,
        F: DoubleEndedIterator<Item = u8> + ExactSizeIterator,
        for<'a> &'a mut F: Iterator<Item = u8>,
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

        /// Use animation options.
        pub const fn animate(self) -> AnimatedDisplayOptions<'d, N, Token, CLK, DIO, DELAY, F, M> {
            AnimatedDisplayOptions {
                options: self,
                delay_ms: 500,
                direction: Direction::LeftToRight,
                style: WindowsStyle::Overlapping,
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

    impl<const N: usize, CLK, DIO, DELAY, ERR, F, M>
        AnimatedDisplayOptions<'_, N, Token, CLK, DIO, DELAY, F, M>
    where
        CLK: OutputPin<Error = ERR>,
        DIO: OutputPin<Error = ERR> + ConditionalInputPin<ERR>,
        DELAY: DelayTrait,
        F: DoubleEndedIterator<Item = u8> + ExactSizeIterator,
        for<'a> &'a mut F: Iterator<Item = u8>,
        M: MaybeFlipped<N>,
    {
        /// Set the delay in milliseconds between each animation step.
        pub const fn delay_ms(mut self, delay_ms: u32) -> Self {
            self.delay_ms = delay_ms;
            self
        }

        /// Set the animation direction.
        pub const fn direction(mut self, direction: Direction) -> Self {
            self.direction = direction;
            self
        }

        /// Set the animation direction to [`Direction::LeftToRight`].
        pub const fn left(mut self) -> Self {
            self.direction = Direction::LeftToRight;
            self
        }

        /// Set the animation direction to [`Direction::RightToLeft`].
        pub const fn right(mut self) -> Self {
            self.direction = Direction::RightToLeft;
            self
        }

        /// Set the animation style.
        pub const fn style(mut self, style: WindowsStyle) -> Self {
            self.style = style;
            self
        }

        pub fn steps(&mut self) -> impl AnimationIter<Item = Result<(), Error<ERR>>> + '_ {
            let original_position = self.options.position;

            let (position, _) = M::calculate(original_position, &mut self.options.iter);

            let windows = windows_new_api::<N>(&mut self.options.iter, self.direction, self.style)
                .map(move |window| {
                    let (_, bytes) = M::calculate(original_position, window);

                    bytes
                });

            self.options
                .device
                .animate(position, self.delay_ms, windows)
        }

        pub async fn run(mut self) -> usize {
            self.steps().count().await
        }
    }
}
