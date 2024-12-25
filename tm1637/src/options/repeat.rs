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
    options: DisplayOptions<'d, N, T, CLK, DIO, DELAY, I, D>,
    delay_ms: u32,
}

impl<'d, const N: usize, T, CLK, DIO, DELAY, I, M>
    RepeatDisplayOptions<'d, N, T, CLK, DIO, DELAY, I, M>
{
    /// Create a new [`RepeatDisplayOptions`] instance.
    pub fn new(options: DisplayOptions<'d, N, T, CLK, DIO, DELAY, I, M>, delay_ms: u32) -> Self {
        Self { options, delay_ms }
    }

    /// Create a new [`RepeatDisplayOptions`] instance with default settings.
    pub fn new_with_defaults(options: DisplayOptions<'d, N, T, CLK, DIO, DELAY, I, M>) -> Self {
        Self::new(options, 500)
    }

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
    >
    where
        I: DoubleEndedIterator<Item = u8> + ExactSizeIterator,
    {
        let iter = self.options.iter.map(move |i| [i]).map(|i| i.into_iter());

        Scroller::new(
            self.options.device,
            1,
            self.options.position,
            self.delay_ms,
            iter,
            self.options._flip,
        )
    }
}
