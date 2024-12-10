//! Scroll animation settings.

use crate::TM1637;

mod direction;
mod style;

use super::{windows::windows, DisplayOptions};
pub use direction::ScrollDirection;
pub use style::ScrollStyle;

/// High-level API for scroll animations.
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ScrollDisplayOptions<'d, const N: usize, T, CLK, DIO, DELAY, I, D> {
    pub(crate) options: DisplayOptions<'d, N, T, CLK, DIO, DELAY, I, D>,
    pub(crate) delay_ms: u32,
    pub(crate) direction: ScrollDirection,
    pub(crate) style: ScrollStyle,
}

#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Scroller<'d, const N: usize, T, CLK, DIO, DELAY, I, M> {
    pub(crate) device: &'d mut TM1637<N, T, CLK, DIO, DELAY>,
    pub(crate) inner_iter_len: usize,
    pub(crate) position: usize,
    pub(crate) delay_ms: u32,
    pub(crate) iter: I,
    pub(crate) _flip: M,
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

#[::duplicate::duplicate_item(
    module        async     await               Token                     DelayTrait                             ScrollIter;
    [asynch]      [async]   [await.identity()]  [crate::tokens::Async]    [::embedded_hal_async::delay::DelayNs] [::futures::Stream];
    [blocking]    []        [identity()]        [crate::tokens::Blocking] [::embedded_hal::delay::DelayNs]       [Iterator];
)]
mod module {
    use ::embedded_hal::digital::OutputPin;
    #[allow(unused_imports)]
    use ::futures::StreamExt as _;

    use crate::{
        align::{Align, Aligned},
        maybe_flipped::MaybeFlipped,
        ConditionalInputPin, Error, Identity,
    };

    use super::Scroller;

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
