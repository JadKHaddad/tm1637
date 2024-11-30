use crate::{AnimationStyle, Direction, Identity, NotFlipped, TM1637};

#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct InitDisplayOptions<'d, const N: usize, T, CLK, DIO, DELAY> {
    pub(crate) device: &'d mut TM1637<N, T, CLK, DIO, DELAY>,
}

impl<'d, 'b, const N: usize, T, CLK, DIO, DELAY> InitDisplayOptions<'d, N, T, CLK, DIO, DELAY> {
    pub fn put_slice(
        self,
        bytes: &'b [u8],
    ) -> DisplayOptions<'d, 'b, N, T, CLK, DIO, DELAY, impl FnMut(u8) -> u8 + Clone, NotFlipped<T>>
    {
        DisplayOptions {
            device: self.device,
            position: 0,
            bytes,
            map: Identity::identity,
            _flip: NotFlipped::new(),
        }
    }

    pub fn put_str(
        self,
        str: &'b str,
    ) -> DisplayOptions<'d, 'b, N, T, CLK, DIO, DELAY, impl FnMut(u8) -> u8 + Clone, NotFlipped<T>>
    {
        DisplayOptions {
            device: self.device,
            position: 0,
            bytes: str.as_bytes(),
            map: crate::mappings::from_ascii_byte,
            _flip: NotFlipped::new(),
        }
    }

    pub fn clock() {
        todo!()
    }

    pub fn loading() {
        todo!()
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct DisplayOptions<'d, 'b, const N: usize, T, CLK, DIO, DELAY, F, M> {
    pub(crate) device: &'d mut TM1637<N, T, CLK, DIO, DELAY>,
    pub(crate) position: usize,
    pub(crate) bytes: &'b [u8],
    pub(crate) map: F,
    pub(crate) _flip: M,
}

#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct AnimatedDisplayOptions<'d, 'b, const N: usize, T, CLK, DIO, DELAY, F, D> {
    options: DisplayOptions<'d, 'b, N, T, CLK, DIO, DELAY, F, D>,
    delay_ms: u32,
    direction: Direction,
    style: AnimationStyle,
}

#[::duplicate::duplicate_item(
    module        async     await               Token                 FlipTrait                       DelayTrait;
    [asynch]      [async]   [await.identity()]  [crate::Async]        [crate::AsyncMaybeFlipped]      [::embedded_hal_async::delay::DelayNs];
    [blocking]    []        [identity()]        [crate::Blocking]     [crate::BlockingMaybeFlipped]   [::embedded_hal::delay::DelayNs];
)]
pub mod module {
    use crate::{
        AnimatedDisplayOptions, AnimationStyle, ConditionalInputPin, Direction, DisplayOptions,
        Error, Flipped, Identity, NotFlipped,
    };
    use ::embedded_hal::digital::OutputPin;

    impl<const N: usize, CLK, DIO, DELAY, ERR, F>
        DisplayOptions<'_, '_, N, Token, CLK, DIO, DELAY, F, NotFlipped<Token>>
    where
        CLK: OutputPin<Error = ERR>,
        DIO: OutputPin<Error = ERR> + ConditionalInputPin<ERR>,
        DELAY: DelayTrait,
        F: FnMut(u8) -> u8 + Clone + 'static,
    {
        /// TODO
        pub async fn display_unchecked(self) -> Result<(), Error<ERR>> {
            self.device
                .display_unchecked(self.position, self.bytes.iter().copied().map(self.map))
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
        M: FlipTrait<N, Token, CLK, DIO, DELAY, ERR>,
    {
        /// TODO
        pub fn position(mut self, position: usize) -> Self {
            self.position = position;
            self
        }

        /// TODO
        pub async fn display(self) -> Result<(), Error<ERR>> {
            M::display_slice_mapped(self.device, self.position, self.bytes, self.map).await
        }

        /// TODO
        pub fn animate(self) -> AnimatedDisplayOptions<'d, 'b, N, Token, CLK, DIO, DELAY, F, M> {
            AnimatedDisplayOptions {
                options: self,
                delay_ms: 500,
                direction: Default::default(),
                style: Default::default(),
            }
        }

        /// TODO
        pub fn flip(
            self,
        ) -> DisplayOptions<
            'd,
            'b,
            N,
            Token,
            CLK,
            DIO,
            DELAY,
            impl FnMut(u8) -> u8 + Clone,
            Flipped<Token>,
        > {
            DisplayOptions {
                device: self.device,
                position: self.position,
                bytes: self.bytes,
                map: self.map,
                _flip: Flipped::new(),
            }
        }
    }

    impl<const N: usize, CLK, DIO, DELAY, ERR, F, M>
        AnimatedDisplayOptions<'_, '_, N, Token, CLK, DIO, DELAY, F, M>
    where
        CLK: OutputPin<Error = ERR>,
        DIO: OutputPin<Error = ERR> + ConditionalInputPin<ERR>,
        DELAY: DelayTrait,
        F: FnMut(u8) -> u8 + Clone + 'static,
        M: FlipTrait<N, Token, CLK, DIO, DELAY, ERR>,
    {
        /// TODO
        pub fn delay_ms(mut self, delay_ms: u32) -> Self {
            self.delay_ms = delay_ms;
            self
        }

        /// TODO
        pub fn direction(mut self, direction: Direction) -> Self {
            self.direction = direction;
            self
        }

        /// TODO
        pub fn left(mut self) -> Self {
            self.direction = Direction::LeftToRight;
            self
        }

        /// TODO
        pub fn right(mut self) -> Self {
            self.direction = Direction::RightToLeft;
            self
        }

        /// TODO
        pub fn style(mut self, style: AnimationStyle) -> Self {
            self.style = style;
            self
        }

        /// TODO
        pub async fn display(self) -> Result<(), Error<ERR>> {
            match self.style {
                AnimationStyle::Overlapping => {
                    M::move_slice_overlapping_mapped(
                        self.options.device,
                        self.options.position,
                        self.options.bytes,
                        self.delay_ms,
                        self.direction,
                        self.options.map,
                    )
                    .await
                }
                AnimationStyle::ToEnd => {
                    M::move_slice_to_end_mapped(
                        self.options.device,
                        self.options.position,
                        self.options.bytes,
                        self.delay_ms,
                        self.direction,
                        self.options.map,
                    )
                    .await
                }
            }
        }
    }
}
