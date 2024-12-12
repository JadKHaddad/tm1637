use super::{scroll::Scroller, DisplayOptions};

/// High-level API for repeat animations.
///
/// Display all bytes of the given iterator on the same position.
///
/// # Example
///
/// Repeat the string "HELLO" on the display.
///
/// ```rust
/// use tm1637_embedded_hal::{mock::Noop, TM1637Builder};
///
/// let mut tm = TM1637Builder::new(Noop, Noop, Noop).build_blocking::<4>();
///
/// tm.options().str("HELLO").repeat().finish().run();
/// ```
///
/// The display will show:
///
/// ```text
/// +---+ +---+ +---+ +---+
/// | H | |   | |   | |   |
/// +---+ +---+ +---+ +---+
///
/// +---+ +---+ +---+ +---+
/// | E | |   | |   | |   |
/// +---+ +---+ +---+ +---+
///
/// +---+ +---+ +---+ +---+
/// | L | |   | |   | |   |
/// +---+ +---+ +---+ +---+
///
/// +---+ +---+ +---+ +---+
/// | L | |   | |   | |   |
/// +---+ +---+ +---+ +---+
///
/// +---+ +---+ +---+ +---+
/// | O | |   | |   | |   |
/// +---+ +---+ +---+ +---+
/// ```
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct RepeatDisplayOptions<'d, const N: usize, T, CLK, DIO, DELAY, I, D> {
    pub(crate) options: DisplayOptions<'d, N, T, CLK, DIO, DELAY, I, D>,
    pub(crate) delay_ms: u32,
}

impl<'d, const N: usize, T, CLK, DIO, DELAY, I, M>
    RepeatDisplayOptions<'d, N, T, CLK, DIO, DELAY, I, M>
where
    I: DoubleEndedIterator<Item = u8> + ExactSizeIterator,
{
    /// Set the delay in milliseconds between each animation step.
    pub const fn delay_ms(mut self, delay_ms: u32) -> Self {
        self.delay_ms = delay_ms;
        self
    }

    /// Finish setting the repeat animation.
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
