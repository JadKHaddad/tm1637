use crate::{Direction, Identity, NotFlipped, WindowsStyle, TM1637};

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
    ) -> DisplayOptions<'d, 'b, N, T, CLK, DIO, DELAY, impl FnMut(u8) -> u8 + Clone, NotFlipped>
    {
        DisplayOptions {
            device: self.device,
            position: 0,
            bytes,
            dots: [0; N],
            map: Identity::identity,
            _flip: NotFlipped,
        }
    }

    /// Prepare to display a string.
    pub fn put_str(
        self,
        str: &'b str,
    ) -> DisplayOptions<'d, 'b, N, T, CLK, DIO, DELAY, impl FnMut(u8) -> u8 + Clone, NotFlipped>
    {
        DisplayOptions {
            device: self.device,
            position: 0,
            bytes: str.as_bytes(),
            dots: [0; N],
            map: crate::mappings::from_ascii_byte,
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
        ClockDisplayOptions {
            device: self.device,
            hour: 0,
            minute: 0,
            bytes: [0; 4],
        }
    }
}

/// High-level API for display operations.
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct DisplayOptions<'d, 'b, const N: usize, T, CLK, DIO, DELAY, F, M> {
    device: &'d mut TM1637<N, T, CLK, DIO, DELAY>,
    position: usize,
    bytes: &'b [u8],
    dots: [u8; N],
    map: F,
    _flip: M,
}

/// High-level API for animations.
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct AnimatedDisplayOptions<'d, 'b, const N: usize, T, CLK, DIO, DELAY, F, D> {
    options: DisplayOptions<'d, 'b, N, T, CLK, DIO, DELAY, F, D>,
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
    bytes: [u8; 4],
}

impl<'d, 'b, T, CLK, DIO, DELAY> ClockDisplayOptions<'d, T, CLK, DIO, DELAY> {
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
        &'b mut self,
    ) -> DisplayOptions<'d, 'b, 4, T, CLK, DIO, DELAY, impl FnMut(u8) -> u8 + Clone, NotFlipped>
    where
        'b: 'd,
    {
        self.bytes = crate::formatters::clock_to_4digits(self.hour, self.minute, false);

        DisplayOptions {
            device: self.device,
            position: 0,
            bytes: &self.bytes,
            dots: [0; 4],
            map: Identity::identity,
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
        mappings::{windows, zip_or, SegmentBits},
        AnimatedDisplayOptions, ConditionalInputPin, Direction, DisplayOptions, Error, Flipped,
        Identity, MaybeFlipped, WindowsStyle,
    };

    impl<'d, 'b, const N: usize, CLK, DIO, DELAY, ERR, F, M>
        DisplayOptions<'d, 'b, N, Token, CLK, DIO, DELAY, F, M>
    where
        CLK: OutputPin<Error = ERR>,
        DIO: OutputPin<Error = ERR> + ConditionalInputPin<ERR>,
        DELAY: DelayTrait,
        F: FnMut(u8) -> u8 + Clone,
        M: MaybeFlipped<N>,
    {
        pub const fn dot(mut self, position: usize, state: bool) -> Self {
            self.dots[position] = match state {
                true => SegmentBits::SegPoint as u8,
                false => 0,
            };

            self
        }

        pub const fn set_dot(mut self, position: usize) -> Self {
            self.dots[position] = SegmentBits::SegPoint as u8;
            self
        }

        pub const fn unset_dot(mut self, position: usize) -> Self {
            self.dots[position] = 0;
            self
        }

        /// Set the position on the display from which to start displaying the bytes.
        pub const fn position(mut self, position: usize) -> Self {
            self.position = position;
            self
        }

        /// Display the bytes on a `flipped` or `non-flipped` display.
        pub async fn display(self) -> Result<(), Error<ERR>> {
            let (position, bytes) =
                M::calculate(self.position, self.bytes.iter().copied().map(self.map));

            self.device
                .display(position, zip_or(bytes, self.dots.iter().copied()))
                .await
        }

        /// Use animation options.
        pub const fn animate(
            self,
        ) -> AnimatedDisplayOptions<'d, 'b, N, Token, CLK, DIO, DELAY, F, M> {
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
        ) -> DisplayOptions<'d, 'b, N, Token, CLK, DIO, DELAY, impl FnMut(u8) -> u8 + Clone, Flipped>
        {
            DisplayOptions {
                device: self.device,
                position: self.position,
                bytes: self.bytes,
                dots: self.dots,
                map: self.map,
                _flip: Flipped,
            }
        }
    }

    impl<const N: usize, CLK, DIO, DELAY, ERR, F, M>
        AnimatedDisplayOptions<'_, '_, N, Token, CLK, DIO, DELAY, F, M>
    where
        CLK: OutputPin<Error = ERR>,
        DIO: OutputPin<Error = ERR> + ConditionalInputPin<ERR>,
        DELAY: DelayTrait,
        F: FnMut(u8) -> u8 + Clone,
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
            let dots = self.options.dots.iter().copied();
            let map = self.options.map.clone();

            let windows = windows::<N>(self.options.bytes, self.direction, self.style)
                .map(move |window| zip_or(window.map(map.clone()), dots.clone()));

            self.options
                .device
                .animate(self.options.position, self.delay_ms, windows)
        }

        pub async fn run(mut self) -> usize {
            self.steps().count().await
        }
    }
}
