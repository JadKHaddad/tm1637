//! Device definition and implementation.

#[::duplicate::duplicate_item(
    device_           module        async     await               trait_return                                                 delay_trait;
    [AsyncTM1637]     [asynch]      [async]   [await.identity()]  [impl ::core::future::Future<Output=Result<(), Error<ERR>>>] [::embedded_hal_async::delay::DelayNs];
    [BlockingTM1637]  [blocking]    []        [identity()]        [Result<(), Error<ERR>>]                                     [::embedded_hal::delay::DelayNs];
)]
pub mod module {
    mod inner {
        use crate::{AnimationStyle, ConditionalInputPin, Direction, Error, Identity};
        use ::embedded_hal::digital::OutputPin;

        /// TODO
        #[derive(Debug)]
        pub struct DisplayOptions<'d, 'b, const N: usize, CLK, DIO, DELAY, F, M> {
            pub(crate) device: &'d mut crate::device_<N, CLK, DIO, DELAY>,
            pub(crate) position: usize,
            pub(crate) bytes: &'b [u8],
            pub(crate) map: F,
            pub(crate) _flip: M,
        }

        impl<'d, 'b, const N: usize, CLK, DIO, DELAY, ERR, F, M>
            DisplayOptions<'d, 'b, N, CLK, DIO, DELAY, F, M>
        where
            CLK: OutputPin<Error = ERR>,
            DIO: OutputPin<Error = ERR> + ConditionalInputPin<ERR>,
            DELAY: delay_trait,
            F: FnMut(u8) -> u8 + Clone + 'static,
            M: MaybeFlipped<N, CLK, DIO, DELAY, ERR>,
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
            pub fn animate(self) -> AnimatedDisplayOptions<'d, 'b, N, CLK, DIO, DELAY, F, M> {
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
            ) -> DisplayOptions<'d, 'b, N, CLK, DIO, DELAY, impl FnMut(u8) -> u8 + Clone, Flipped>
            {
                DisplayOptions {
                    device: self.device,
                    position: self.position,
                    bytes: self.bytes,
                    map: self.map,
                    _flip: Flipped,
                }
            }
        }

        impl<const N: usize, CLK, DIO, DELAY, ERR, F>
            DisplayOptions<'_, '_, N, CLK, DIO, DELAY, F, NotFlipped>
        where
            CLK: OutputPin<Error = ERR>,
            DIO: OutputPin<Error = ERR> + ConditionalInputPin<ERR>,
            DELAY: delay_trait,
            F: FnMut(u8) -> u8 + Clone + 'static,
        {
            /// TODO
            pub async fn display_unchecked(self) -> Result<(), Error<ERR>> {
                self.device
                    .display_unchecked(self.position, self.bytes.iter().copied().map(self.map))
                    .await
            }
        }

        #[derive(Debug)]
        pub struct AnimatedDisplayOptions<'d, 'b, const N: usize, CLK, DIO, DELAY, F, D> {
            options: DisplayOptions<'d, 'b, N, CLK, DIO, DELAY, F, D>,
            delay_ms: u32,
            direction: Direction,
            style: AnimationStyle,
        }

        impl<const N: usize, CLK, DIO, DELAY, ERR, F, M>
            AnimatedDisplayOptions<'_, '_, N, CLK, DIO, DELAY, F, M>
        where
            CLK: OutputPin<Error = ERR>,
            DIO: OutputPin<Error = ERR> + ConditionalInputPin<ERR>,
            DELAY: delay_trait,
            F: FnMut(u8) -> u8 + Clone + 'static,
            M: MaybeFlipped<N, CLK, DIO, DELAY, ERR>,
        {
            pub fn delay_ms(mut self, delay_ms: u32) -> Self {
                self.delay_ms = delay_ms;
                self
            }

            pub fn direction(mut self, direction: Direction) -> Self {
                self.direction = direction;
                self
            }

            pub fn left(mut self) -> Self {
                self.direction = Direction::LeftToRight;
                self
            }

            pub fn right(mut self) -> Self {
                self.direction = Direction::RightToLeft;
                self
            }

            pub fn style(mut self, style: AnimationStyle) -> Self {
                self.style = style;
                self
            }

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

        pub trait MaybeFlipped<const N: usize, CLK, DIO, DELAY, ERR>
        where
            CLK: OutputPin<Error = ERR>,
            DIO: OutputPin<Error = ERR> + ConditionalInputPin<ERR>,
            DELAY: delay_trait,
        {
            fn display_slice_mapped(
                device: &mut crate::device_<N, CLK, DIO, DELAY>,
                position: usize,
                bytes: &[u8],
                map: impl FnMut(u8) -> u8,
            ) -> trait_return;

            fn move_slice_overlapping_mapped(
                device: &mut crate::device_<N, CLK, DIO, DELAY>,
                position: usize,
                bytes: &[u8],
                delay_ms: u32,
                direction: Direction,
                map: impl FnMut(u8) -> u8 + Clone,
            ) -> trait_return;

            fn move_slice_to_end_mapped(
                device: &mut crate::device_<N, CLK, DIO, DELAY>,
                position: usize,
                bytes: &[u8],
                delay_ms: u32,
                direction: Direction,
                map: impl FnMut(u8) -> u8 + Clone,
            ) -> trait_return;
        }

        #[derive(Debug)]
        /// TODO
        pub struct NotFlipped;

        impl<const N: usize, CLK, DIO, DELAY, ERR> MaybeFlipped<N, CLK, DIO, DELAY, ERR> for NotFlipped
        where
            CLK: OutputPin<Error = ERR>,
            DIO: OutputPin<Error = ERR> + ConditionalInputPin<ERR>,
            DELAY: delay_trait,
        {
            async fn display_slice_mapped(
                device: &mut crate::device_<N, CLK, DIO, DELAY>,
                position: usize,
                bytes: &[u8],
                map: impl FnMut(u8) -> u8,
            ) -> Result<(), Error<ERR>> {
                device.display_slice_mapped(position, bytes, map).await
            }

            async fn move_slice_overlapping_mapped(
                device: &mut crate::device_<N, CLK, DIO, DELAY>,
                position: usize,
                bytes: &[u8],
                delay_ms: u32,
                direction: Direction,
                map: impl FnMut(u8) -> u8 + Clone,
            ) -> Result<(), Error<ERR>> {
                device
                    .move_slice_overlapping_mapped(position, bytes, delay_ms, direction, map)
                    .await
            }

            async fn move_slice_to_end_mapped(
                device: &mut crate::device_<N, CLK, DIO, DELAY>,
                position: usize,
                bytes: &[u8],
                delay_ms: u32,
                direction: Direction,
                map: impl FnMut(u8) -> u8 + Clone,
            ) -> Result<(), Error<ERR>> {
                device
                    .move_slice_to_end_mapped(position, bytes, delay_ms, direction, map)
                    .await
            }
        }

        #[derive(Debug)]
        /// TODO
        pub struct Flipped;

        impl<const N: usize, CLK, DIO, DELAY, ERR> MaybeFlipped<N, CLK, DIO, DELAY, ERR> for Flipped
        where
            CLK: OutputPin<Error = ERR>,
            DIO: OutputPin<Error = ERR> + ConditionalInputPin<ERR>,
            DELAY: delay_trait,
        {
            async fn display_slice_mapped(
                device: &mut crate::device_<N, CLK, DIO, DELAY>,
                position: usize,
                bytes: &[u8],
                map: impl FnMut(u8) -> u8,
            ) -> Result<(), Error<ERR>> {
                device
                    .display_slice_flipped_mapped(position, bytes, map)
                    .await
            }

            async fn move_slice_overlapping_mapped(
                device: &mut crate::device_<N, CLK, DIO, DELAY>,
                position: usize,
                bytes: &[u8],
                delay_ms: u32,
                direction: Direction,
                map: impl FnMut(u8) -> u8 + Clone,
            ) -> Result<(), Error<ERR>> {
                device
                    .move_slice_overlapping_flipped_mapped(
                        position, bytes, delay_ms, direction, map,
                    )
                    .await
            }

            async fn move_slice_to_end_mapped(
                device: &mut crate::device_<N, CLK, DIO, DELAY>,
                position: usize,
                bytes: &[u8],
                delay_ms: u32,
                direction: Direction,
                map: impl FnMut(u8) -> u8 + Clone,
            ) -> Result<(), Error<ERR>> {
                device
                    .move_slice_to_end_flipped_mapped(position, bytes, delay_ms, direction, map)
                    .await
            }
        }
    }

    pub use inner::*;
}
