use crate::{AnimationStyle, Direction, Identity, NotFlipped, TM1637};

/// Starting point for a High-level API for display operations.
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct InitDisplayOptions<'d, const N: usize, T, CLK, DIO, DELAY> {
    pub(crate) device: &'d mut TM1637<N, T, CLK, DIO, DELAY>,
}

impl<'d, 'b, const N: usize, T, CLK, DIO, DELAY> InitDisplayOptions<'d, N, T, CLK, DIO, DELAY> {
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
            dots: [false; N],
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
            dots: [false; N],
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
    pub(crate) device: &'d mut TM1637<N, T, CLK, DIO, DELAY>,
    pub(crate) position: usize,
    pub(crate) bytes: &'b [u8],
    pub(crate) dots: [bool; N],
    pub(crate) map: F,
    pub(crate) _flip: M,
}

/// High-level API for animations.
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct AnimatedDisplayOptions<'d, 'b, const N: usize, T, CLK, DIO, DELAY, F, D> {
    options: DisplayOptions<'d, 'b, N, T, CLK, DIO, DELAY, F, D>,
    delay_ms: u32,
    direction: Direction,
    style: AnimationStyle,
}

#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ClockDisplayOptions<'d, T, CLK, DIO, DELAY> {
    pub(crate) device: &'d mut TM1637<4, T, CLK, DIO, DELAY>,
    pub(crate) hour: u8,
    pub(crate) minute: u8,
    bytes: [u8; 4],
}

impl<'d, 'b, T, CLK, DIO, DELAY> ClockDisplayOptions<'d, T, CLK, DIO, DELAY> {
    /// Set the hour.
    pub fn hour(mut self, hour: u8) -> Self {
        self.hour = hour;
        self
    }

    /// Set the minute.
    pub fn minute(mut self, minute: u8) -> Self {
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
            dots: [false; 4],
            map: Identity::identity,
            _flip: NotFlipped,
        }
    }
}

#[::duplicate::duplicate_item(
    module        async     await               Token             DelayTrait;
    [asynch]      [async]   [await.identity()]  [crate::Async]    [::embedded_hal_async::delay::DelayNs];
    [blocking]    []        [identity()]        [crate::Blocking] [::embedded_hal::delay::DelayNs];
)]
pub mod module {
    use crate::{
        AnimatedDisplayOptions, AnimationStyle, ConditionalInputPin, Direction, DisplayOptions,
        Error, Flipped, Identity, MaybeFlipped, NotFlipped,
    };
    use ::embedded_hal::digital::OutputPin;

    impl<const N: usize, CLK, DIO, DELAY, ERR, F>
        DisplayOptions<'_, '_, N, Token, CLK, DIO, DELAY, F, NotFlipped>
    where
        CLK: OutputPin<Error = ERR>,
        DIO: OutputPin<Error = ERR> + ConditionalInputPin<ERR>,
        DELAY: DelayTrait,
        F: FnMut(u8) -> u8 + Clone + 'static,
    {
        /// See [`TM1637::display_unchecked`](crate::TM1637::display_unchecked).
        pub async fn display_unchecked(self) -> Result<(), Error<ERR>> {
            self.device
                .display_unchecked(
                    self.position,
                    crate::mappings::zip_dots(
                        self.bytes.iter().copied().map(self.map),
                        self.dots.iter().copied(),
                    ),
                )
                .await
        }
    }

    impl<'d, 'b, const N: usize, CLK, DIO, DELAY, ERR, F, M>
        DisplayOptions<'d, 'b, N, Token, CLK, DIO, DELAY, F, M>
    where
        CLK: OutputPin<Error = ERR>,
        DIO: OutputPin<Error = ERR> + ConditionalInputPin<ERR>,
        DELAY: DelayTrait,
        F: FnMut(u8) -> u8 + Clone + 'static,
        M: MaybeFlipped<N>,
    {
        pub fn dot(mut self, position: usize, state: bool) -> Self {
            self.dots[position] = state;
            self
        }

        pub fn set_dot(mut self, position: usize) -> Self {
            self.dots[position] = true;
            self
        }

        pub fn unset_dot(mut self, position: usize) -> Self {
            self.dots[position] = false;
            self
        }

        /// Set the position on the display from which to start displaying the bytes.
        pub fn position(mut self, position: usize) -> Self {
            self.position = position;
            self
        }

        /// Display the bytes on a `flipped` or `non-flipped` display.
        pub async fn display(self) -> Result<(), Error<ERR>> {
            let (position, bytes) = M::calculate(
                self.position,
                self.bytes.iter().copied().map(self.map), /* Here we can add the dots */
            );

            self.device.display(position, bytes).await
        }

        /// Use animation options.
        pub fn animate(self) -> AnimatedDisplayOptions<'d, 'b, N, Token, CLK, DIO, DELAY, F, M> {
            AnimatedDisplayOptions {
                options: self,
                delay_ms: 500,
                direction: Default::default(),
                style: Default::default(),
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

    // impl<const N: usize, CLK, DIO, DELAY, ERR, F, M>
    //     AnimatedDisplayOptions<'_, '_, N, Token, CLK, DIO, DELAY, F, M>
    // where
    //     CLK: OutputPin<Error = ERR>,
    //     DIO: OutputPin<Error = ERR> + ConditionalInputPin<ERR>,
    //     DELAY: DelayTrait,
    //     F: FnMut(u8) -> u8 + Clone + 'static,
    //     M: FlipTrait<N, Token, CLK, DIO, DELAY, ERR>,
    // {
    //     /// Set the delay in milliseconds between each animation step.
    //     pub fn delay_ms(mut self, delay_ms: u32) -> Self {
    //         self.delay_ms = delay_ms;
    //         self
    //     }

    //     /// Set the animation direction.
    //     pub fn direction(mut self, direction: Direction) -> Self {
    //         self.direction = direction;
    //         self
    //     }

    //     /// Set the animation direction to [`Direction::LeftToRight`].
    //     pub fn left(mut self) -> Self {
    //         self.direction = Direction::LeftToRight;
    //         self
    //     }

    //     /// Set the animation direction to [`Direction::RightToLeft`].
    //     pub fn right(mut self) -> Self {
    //         self.direction = Direction::RightToLeft;
    //         self
    //     }

    //     /// Set the animation style.
    //     pub fn style(mut self, style: AnimationStyle) -> Self {
    //         self.style = style;
    //         self
    //     }

    //     /// Run the animation on a `flipped` or `non-flipped` display.
    //     pub async fn display(self) -> Result<(), Error<ERR>> {
    //         match self.style {
    //             AnimationStyle::Overlapping => {
    //                 M::move_slice_overlapping_dotted_mapped(
    //                     self.options.device,
    //                     self.options.position,
    //                     self.options.bytes,
    //                     &self.options.dots,
    //                     self.delay_ms,
    //                     self.direction,
    //                     self.options.map,
    //                 )
    //                 .await
    //             }
    //             AnimationStyle::NonOverlapping => {
    //                 M::move_slice_to_end_dotted_mapped(
    //                     self.options.device,
    //                     self.options.position,
    //                     self.options.bytes,
    //                     &self.options.dots,
    //                     self.delay_ms,
    //                     self.direction,
    //                     self.options.map,
    //                 )
    //                 .await
    //             }
    //         }
    //     }
    // }
}
