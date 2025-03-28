//! TODO: Docs

use super::DisplayOptions;

/// TODO: document this enum
#[derive(Debug)]
pub enum ErasedDisplayOptions<'d, T, CLK, DIO, DELAY, I, M> {
    /// TODO: Document this variant
    Four(DisplayOptions<'d, 4, T, CLK, DIO, DELAY, I, M>),
    /// TODO: Document this variant
    Six(DisplayOptions<'d, 6, T, CLK, DIO, DELAY, I, M>),
}

impl<'d, T, CLK, DIO, DELAY, I, M> From<DisplayOptions<'d, 4, T, CLK, DIO, DELAY, I, M>>
    for ErasedDisplayOptions<'d, T, CLK, DIO, DELAY, I, M>
{
    fn from(value: DisplayOptions<'d, 4, T, CLK, DIO, DELAY, I, M>) -> Self {
        Self::Four(value)
    }
}

impl<'d, T, CLK, DIO, DELAY, I, M> From<DisplayOptions<'d, 6, T, CLK, DIO, DELAY, I, M>>
    for ErasedDisplayOptions<'d, T, CLK, DIO, DELAY, I, M>
{
    fn from(value: DisplayOptions<'d, 6, T, CLK, DIO, DELAY, I, M>) -> Self {
        Self::Six(value)
    }
}
