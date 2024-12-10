//! Ready to use circle animations.

use crate::{tokens::NotFlipped, TM1637};

mod bits;
mod default_options;
mod direction;

pub use bits::*;
pub use default_options::RotatingCircleOptions;
pub use direction::*;

#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct CirclesDisplayOptions<'d, const N: usize, T, CLK, DIO, DELAY> {
    pub(crate) device: &'d mut TM1637<N, T, CLK, DIO, DELAY>,
}

impl<'d, const N: usize, T, CLK, DIO, DELAY> CirclesDisplayOptions<'d, N, T, CLK, DIO, DELAY> {
    pub fn rotating(self) -> RotatingCircleOptions<'d, N, T, CLK, DIO, DELAY, NotFlipped> {
        RotatingCircleOptions::new(self.device, NotFlipped)
    }
}
