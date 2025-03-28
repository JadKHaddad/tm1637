use crate::{
    maybe_flipped::MaybeFlipped,
    options::{repeat::RepeatDisplayOptions, scroll::Scroller, DisplayOptions},
    TM1637,
};

use super::{bits::RotatingCircleBits, RotatingDirection};

/// Default rotating circle options.
///
/// The animation consists of a single circle that rotates clockwise or counter-clockwise on a given position.
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct RotatingCircleOptions<'d, const N: usize, T, CLK, DIO, DELAY, M> {
    device: &'d mut TM1637<N, T, CLK, DIO, DELAY>,
    position: usize,
    delay_ms: u32,
    direction: RotatingDirection,
    _flip: M,
}

impl<'d, const N: usize, T, CLK, DIO, DELAY, M>
    RotatingCircleOptions<'d, N, T, CLK, DIO, DELAY, M>
{
    /// Create a new [`RotatingCircleOptions`] instance.
    pub const fn new(
        device: &'d mut TM1637<N, T, CLK, DIO, DELAY>,
        position: usize,
        delay_ms: u32,
        direction: RotatingDirection,
        flip: M,
    ) -> Self {
        Self {
            device,
            position,
            delay_ms,
            direction,
            _flip: flip,
        }
    }

    /// Create a new [`RotatingCircleOptions`] instance with default settings.
    pub const fn new_with_defaults(device: &'d mut TM1637<N, T, CLK, DIO, DELAY>, flip: M) -> Self {
        Self::new(device, 0, 500, RotatingDirection::Clockwise, flip)
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

    /// Set the rotating direction.
    pub const fn direction(mut self, direction: RotatingDirection) -> Self {
        self.direction = direction;
        self
    }

    /// Flip the display.
    pub fn flip(self) -> RotatingCircleOptions<'d, N, T, CLK, DIO, DELAY, impl MaybeFlipped<N>>
    where
        M: MaybeFlipped<N>,
    {
        RotatingCircleOptions {
            device: self.device,
            position: self.position,
            delay_ms: self.delay_ms,
            direction: self.direction,
            _flip: M::flip(),
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
        let bytes = match self.direction {
            RotatingDirection::Clockwise => RotatingCircleBits::all_u8_reversed(),
            RotatingDirection::CounterClockwise => RotatingCircleBits::all_u8(),
        };

        RepeatDisplayOptions::new(
            DisplayOptions {
                device: self.device,
                position: self.position,
                iter: bytes.into_iter(),
                _flip: self._flip,
            },
            self.delay_ms,
        )
        .finish()
    }
}
