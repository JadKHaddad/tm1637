//! High-level API for display operations.

use crate::{
    exact_size::ExactSizeChainExt, mappings::SegmentBits, maybe_flipped::MaybeFlipped, numbers,
    str::StrParser, tokens::NotFlipped, TM1637,
};

pub mod circles;
pub mod windows;

mod clock;
mod repeat;
mod scroll;

pub use clock::*;
pub use repeat::*;
pub use scroll::*;

/// High-level API for display operations.
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct DisplayOptions<'d, const N: usize, T, CLK, DIO, DELAY, I, M> {
    pub(crate) device: &'d mut TM1637<N, T, CLK, DIO, DELAY>,
    pub(crate) position: usize,
    pub(crate) iter: I,
    pub(crate) _flip: M,
}

impl<'d, 'b, const N: usize, T, CLK, DIO, DELAY, I, M>
    DisplayOptions<'d, N, T, CLK, DIO, DELAY, I, M>
{
    /// Create a new [`DisplayOptions`] instance.
    pub fn new(
        device: &'d mut TM1637<N, T, CLK, DIO, DELAY>,
    ) -> DisplayOptions<'d, N, T, CLK, DIO, DELAY, core::iter::Empty<u8>, NotFlipped> {
        DisplayOptions {
            device,
            position: 0,
            iter: core::iter::empty(),
            _flip: NotFlipped,
        }
    }

    /// Set the position on the display from which to start displaying the bytes.
    pub const fn position(mut self, position: usize) -> Self {
        self.position = position;
        self
    }

    /// Add a slice of bytes.
    pub fn slice(
        self,
        bytes: &'b [u8],
    ) -> DisplayOptions<
        'd,
        N,
        T,
        CLK,
        DIO,
        DELAY,
        impl DoubleEndedIterator<Item = u8> + ExactSizeIterator + 'b,
        M,
    >
    where
        I: DoubleEndedIterator<Item = u8> + ExactSizeIterator + 'b,
    {
        DisplayOptions {
            device: self.device,
            position: self.position,
            iter: self.iter.exact_size_chain(bytes.iter().copied()),
            _flip: self._flip,
        }
    }

    /// Add a string.
    pub fn str(
        self,
        str: &'b str,
    ) -> DisplayOptions<
        'd,
        N,
        T,
        CLK,
        DIO,
        DELAY,
        impl DoubleEndedIterator<Item = u8> + ExactSizeIterator + 'b,
        M,
    >
    where
        I: DoubleEndedIterator<Item = u8> + ExactSizeIterator + 'b,
    {
        DisplayOptions {
            device: self.device,
            position: self.position,
            iter: self.iter.exact_size_chain(StrParser::new(str)),
            _flip: self._flip,
        }
    }

    /// Add an iterator of bytes.
    ///
    /// # Example
    ///
    /// Manually map each byte in a slice into a human readable character and set the dot at the 2nd position.
    ///
    /// ```rust
    /// use tm1637_embedded_hal::{mappings::SegmentBits, mock::Noop, str::StrParser, TM1637Builder};
    ///
    /// let mut tm = TM1637Builder::new(Noop, Noop, Noop).build_blocking::<4>();
    ///
    /// tm.options()
    ///     .iter(StrParser::new("HELLO").enumerate().map(move |(i, b)| {
    ///         if i == 1 {
    ///             b | SegmentBits::Dot as u8
    ///         } else {
    ///             b
    ///         }
    ///     }))
    ///     .display()
    ///     .ok();
    ///
    /// // Equivalent to
    ///
    /// tm.options()
    ///    .str("HELLO")
    ///    .dot(1)
    ///    .display()
    ///    .ok();
    /// ```
    pub fn iter<It>(
        self,
        iter: It,
    ) -> DisplayOptions<
        'd,
        N,
        T,
        CLK,
        DIO,
        DELAY,
        impl DoubleEndedIterator<Item = u8> + ExactSizeIterator,
        M,
    >
    where
        I: DoubleEndedIterator<Item = u8> + ExactSizeIterator,
        It: DoubleEndedIterator<Item = u8> + ExactSizeIterator,
    {
        DisplayOptions {
            device: self.device,
            position: self.position,
            iter: self.iter.exact_size_chain(iter),
            _flip: self._flip,
        }
    }

    /// Prepare to display a digital clock.
    ///
    /// See [`ClockDisplayOptions`].
    pub fn clock(self) -> ClockDisplayOptions<'d, N, T, CLK, DIO, DELAY, I, M> {
        ClockDisplayOptions::new(self)
    }

    /// Use scroll animation options.
    pub fn scroll(self) -> ScrollDisplayOptions<'d, N, T, CLK, DIO, DELAY, I, M> {
        ScrollDisplayOptions::new_with_defaults(self)
    }

    /// Use repeat animation options.
    ///
    /// Display all bytes of the given iterator on the same position.
    ///
    /// See [`RepeatDisplayOptions`].
    pub fn repeat(self) -> RepeatDisplayOptions<'d, N, T, CLK, DIO, DELAY, I, M> {
        RepeatDisplayOptions::new_with_defaults(self)
    }

    /// Add a dynamic dot to the display at the specified position.
    ///
    /// ## Dynamic
    ///
    /// The dot is tied to the byte at the specified position and will move with it.
    pub fn dot(
        self,
        position: usize,
    ) -> DisplayOptions<
        'd,
        N,
        T,
        CLK,
        DIO,
        DELAY,
        impl DoubleEndedIterator<Item = u8> + ExactSizeIterator,
        M,
    >
    where
        I: DoubleEndedIterator<Item = u8> + ExactSizeIterator,
    {
        DisplayOptions {
            device: self.device,
            position: self.position,
            iter: self.iter.enumerate().map(move |(i, b)| {
                if i == position {
                    b | SegmentBits::Dot as u8
                } else {
                    b
                }
            }),
            _flip: self._flip,
        }
    }

    /// Remove the dot from the display at the specified position.
    pub fn remove_dot(
        self,
        position: usize,
    ) -> DisplayOptions<
        'd,
        N,
        T,
        CLK,
        DIO,
        DELAY,
        impl DoubleEndedIterator<Item = u8> + ExactSizeIterator,
        M,
    >
    where
        I: DoubleEndedIterator<Item = u8> + ExactSizeIterator,
    {
        DisplayOptions {
            device: self.device,
            position: self.position,
            iter: self.iter.enumerate().map(move |(i, b)| {
                if i == position {
                    b & !(SegmentBits::Dot as u8)
                } else {
                    b
                }
            }),
            _flip: self._flip,
        }
    }

    /// Set the dot at the specified position.
    pub fn set_dot(
        self,
        position: usize,
        dot: bool,
    ) -> DisplayOptions<
        'd,
        N,
        T,
        CLK,
        DIO,
        DELAY,
        impl DoubleEndedIterator<Item = u8> + ExactSizeIterator,
        M,
    >
    where
        I: DoubleEndedIterator<Item = u8> + ExactSizeIterator,
    {
        DisplayOptions {
            device: self.device,
            position: self.position,
            iter: self.iter.enumerate().map(move |(i, b)| {
                if i == position {
                    if dot {
                        b | SegmentBits::Dot as u8
                    } else {
                        b & !(SegmentBits::Dot as u8)
                    }
                } else {
                    b
                }
            }),
            _flip: self._flip,
        }
    }

    /// Add dots to all positions in the display.
    pub fn dots(
        self,
    ) -> DisplayOptions<
        'd,
        N,
        T,
        CLK,
        DIO,
        DELAY,
        impl DoubleEndedIterator<Item = u8> + ExactSizeIterator,
        M,
    >
    where
        I: DoubleEndedIterator<Item = u8> + ExactSizeIterator,
    {
        DisplayOptions {
            device: self.device,
            position: self.position,
            iter: self.iter.map(|b| b | SegmentBits::Dot as u8),
            _flip: self._flip,
        }
    }

    /// Remove dots from all positions in the display.
    pub fn remove_dots(
        self,
    ) -> DisplayOptions<
        'd,
        N,
        T,
        CLK,
        DIO,
        DELAY,
        impl DoubleEndedIterator<Item = u8> + ExactSizeIterator,
        M,
    >
    where
        I: DoubleEndedIterator<Item = u8> + ExactSizeIterator,
    {
        DisplayOptions {
            device: self.device,
            position: self.position,
            iter: self.iter.map(|b| b & !(SegmentBits::Dot as u8)),
            _flip: self._flip,
        }
    }

    /// Map the bytes using the provided function.
    ///
    /// # Example
    ///
    /// Manually map each byte in a slice into a human readable character.
    ///
    /// ```rust
    /// use tm1637_embedded_hal::{mappings::from_ascii_byte, mock::Noop, TM1637Builder};
    ///
    /// let mut tm = TM1637Builder::new(Noop, Noop, Noop).build_blocking::<4>();
    ///
    /// tm.options()
    ///     .slice(b"HELLO")
    ///     .map(from_ascii_byte)
    ///     .display()
    ///     .ok();
    ///
    /// // Equivalent** to
    ///
    /// tm.options()
    ///    .str("HELLO")
    ///    .display()
    ///    .ok();
    /// ```
    /// ** The [`DisplayOptions::str`] method uses [`StrParser`] internally.
    pub fn map<F: FnMut(u8) -> u8>(
        self,
        f: F,
    ) -> DisplayOptions<
        'd,
        N,
        T,
        CLK,
        DIO,
        DELAY,
        impl DoubleEndedIterator<Item = u8> + ExactSizeIterator,
        M,
    >
    where
        I: DoubleEndedIterator<Item = u8> + ExactSizeIterator,
    {
        DisplayOptions {
            device: self.device,
            position: self.position,
            iter: self.iter.map(f),
            _flip: self._flip,
        }
    }

    /// Flip the display.
    pub fn flip(
        self,
    ) -> DisplayOptions<
        'd,
        N,
        T,
        CLK,
        DIO,
        DELAY,
        impl DoubleEndedIterator<Item = u8> + ExactSizeIterator,
        impl MaybeFlipped<N>,
    >
    where
        I: DoubleEndedIterator<Item = u8> + ExactSizeIterator,
        M: MaybeFlipped<N>,
    {
        DisplayOptions {
            device: self.device,
            position: self.position,
            iter: self.iter,
            _flip: M::flip(),
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
        options::DisplayOptions,
        ConditionalInputPin, Error, Identity,
    };

    #[::duplicate::duplicate_item(
        NUM_POS ;
        [4] ;
        [6] ;
    )]
    impl<CLK, DIO, DELAY, ERR, I, M> DisplayOptions<'_, NUM_POS, Token, CLK, DIO, DELAY, I, M>
    where
        CLK: OutputPin<Error = ERR>,
        DIO: OutputPin<Error = ERR> + ConditionalInputPin<ERR>,
        DELAY: DelayTrait,
        I: DoubleEndedIterator<Item = u8> + ExactSizeIterator,
        M: MaybeFlipped<NUM_POS>,
    {
        /// Release the `device` and return the calculated position and bytes.
        pub fn calculate(self) -> (usize, impl Iterator<Item = u8>) {
            let (position, bytes) = M::calculate(self.position, self.iter);

            Align::<NUM_POS>::align(position, bytes)
        }

        /// Display the bytes on a `flipped` or `non-flipped` display.
        pub async fn display(self) -> Result<(), Error<ERR>> {
            let (position, bytes) = M::calculate(self.position, self.iter);

            let (position, bytes) = Align::<NUM_POS>::align(position, bytes);

            self.device.display(position, bytes).await
        }
    }
}

#[::duplicate::duplicate_item(
    function    type_   link;
    [u8]        [u8]    ["[`u8`](crate::numbers::u8)"];
    [u8_2]      [u8]    ["[`u8_2`](crate::numbers::u8_2)"];
    [r_u8_2]    [u8]    ["[`r_u8_2`](crate::numbers::r_u8_2)"];
    [u16_3]     [u16]   ["[`u16_3`](crate::numbers::u16_3)"];
    [r_u16_3]   [u16]   ["[`r_u16_3`](crate::numbers::r_u16_3)"];
    [u16_4]     [u16]   ["[`u16_4`](crate::numbers::u16_4)"];
    [r_u16_4]   [u16]   ["[`r_u16_4`](crate::numbers::r_u16_4)"];
    [u32_5]     [u32]   ["[`u32_5`](crate::numbers::u32_5)"];
    [r_u32_5]   [u32]   ["[`r_u32_5`](crate::numbers::r_u32_5)"];
    [u32_6]     [u32]   ["[`u32_6`](crate::numbers::u32_6)"];
    [r_u32_6]   [u32]   ["[`r_u32_6`](crate::numbers::r_u32_6)"];
    [i8_2]      [i8]    ["[`i8_2`](crate::numbers::i8_2)"];
    [i16_3]     [i16]   ["[`i16_3`](crate::numbers::i16_3)"];
    [r_i16_3]   [i16]   ["[`r_i16_3`](crate::numbers::r_i16_3)"];
    [i16_4]     [i16]   ["[`i16_4`](crate::numbers::i16_4)"];
    [r_i16_4]   [i16]   ["[`r_i16_4`](crate::numbers::r_i16_4)"];
    [i32_5]     [i32]   ["[`i32_5`](crate::numbers::i32_5)"];
    [r_i32_5]   [i32]   ["[`r_i32_5`](crate::numbers::r_i32_5)"];
    [i32_6]     [i32]   ["[`i32_6`](crate::numbers::i32_6)"];
    [r_i32_6]   [i32]   ["[`r_i32_6`](crate::numbers::r_i32_6)"];
)]
impl<'d, const N: usize, T, CLK, DIO, DELAY, I, M> DisplayOptions<'d, N, T, CLK, DIO, DELAY, I, M>
where
    I: DoubleEndedIterator<Item = u8> + ExactSizeIterator,
{
    #[doc = "See "]
    #[doc = link]
    pub fn function(
        self,
        n: type_,
    ) -> DisplayOptions<
        'd,
        N,
        T,
        CLK,
        DIO,
        DELAY,
        impl DoubleEndedIterator<Item = u8> + ExactSizeIterator,
        M,
    > {
        DisplayOptions {
            device: self.device,
            position: self.position,
            iter: self.iter.exact_size_chain(numbers::function(n).into_iter()),
            _flip: self._flip,
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate std;
    use std::vec;
    use std::vec::Vec;

    use crate::{mappings::str_from_byte, mock::Noop, TM1637Builder};

    #[test]
    fn dot_is_dynamically_tied_to_byte() {
        let mut tm = TM1637Builder::new(Noop, Noop, Noop).build_blocking::<4>();

        let (_, iter) = tm.options().str("HELLO").dot(1).dot(3).calculate();
        let collected = iter.map(str_from_byte).collect::<Vec<_>>();

        assert_eq!(vec!["H", "E.", "L", "L."], collected);

        let (_, iter) = tm.options().str("HELLO").dot(1).dot(3).flip().calculate();
        let collected = iter.map(str_from_byte).collect::<Vec<_>>();

        assert_eq!(vec!["7.", "7", "3.", "H"], collected);
    }
}
