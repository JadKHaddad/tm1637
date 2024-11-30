#[::duplicate::duplicate_item(
    Name                    module        async     await               Token             Return                                                       DelayTrait;
    [AsyncMaybeFlipped]     [asynch]      [async]   [await.identity()]  [crate::Async]    [impl ::core::future::Future<Output=Result<(), Error<ERR>>>] [::embedded_hal_async::delay::DelayNs];
    [BlockingMaybeFlipped]  [blocking]    []        [identity()]        [crate::Blocking] [Result<(), Error<ERR>>]                                     [::embedded_hal::delay::DelayNs];
)]
pub mod module {
    use embedded_hal::digital::OutputPin;

    use crate::{ConditionalInputPin, Direction, Error, Flipped, Identity, NotFlipped, TM1637};

    /// Display operations for `flipped` or `non-flipped` displays.
    pub trait Name<const N: usize, T, CLK, DIO, DELAY, ERR>
    where
        CLK: OutputPin<Error = ERR>,
        DIO: OutputPin<Error = ERR> + ConditionalInputPin<ERR>,
        DELAY: DelayTrait,
    {
        /// Write the given `bytes` to the display starting from `position` mapping each byte using the provided `map` function.
        fn display_slice_dotted_mapped(
            device: &mut TM1637<N, T, CLK, DIO, DELAY>,
            position: usize,
            bytes: &[u8],
            dots: &[bool],
            map: impl FnMut(u8) -> u8,
        ) -> Return;

        /// Move the given `bytes` in `direction` across the display starting and ending at `position` mapping each byte using the provided `map` function.
        fn move_slice_overlapping_dotted_mapped(
            device: &mut TM1637<N, T, CLK, DIO, DELAY>,
            position: usize,
            bytes: &[u8],
            dots: &[bool],
            delay_ms: u32,
            direction: Direction,
            map: impl FnMut(u8) -> u8 + Clone,
        ) -> Return;

        /// Move the given `bytes` in `direction` across the display starting from `position` mapping each byte using the provided `map` function.
        fn move_slice_to_end_dotted_mapped(
            device: &mut TM1637<N, T, CLK, DIO, DELAY>,
            position: usize,
            bytes: &[u8],
            dots: &[bool],
            delay_ms: u32,
            direction: Direction,
            map: impl FnMut(u8) -> u8 + Clone,
        ) -> Return;
    }

    impl<const N: usize, CLK, DIO, DELAY, ERR> Name<N, Token, CLK, DIO, DELAY, ERR>
        for NotFlipped<Token>
    where
        CLK: OutputPin<Error = ERR>,
        DIO: OutputPin<Error = ERR> + ConditionalInputPin<ERR>,
        DELAY: DelayTrait,
    {
        async fn display_slice_dotted_mapped(
            device: &mut TM1637<N, Token, CLK, DIO, DELAY>,
            position: usize,
            bytes: &[u8],
            dots: &[bool],
            map: impl FnMut(u8) -> u8,
        ) -> Result<(), Error<ERR>> {
            device
                .display_slice_dotted_mapped(position, bytes, dots, map)
                .await
        }

        async fn move_slice_overlapping_dotted_mapped(
            device: &mut TM1637<N, Token, CLK, DIO, DELAY>,
            position: usize,
            bytes: &[u8],
            dots: &[bool],
            delay_ms: u32,
            direction: Direction,
            map: impl FnMut(u8) -> u8 + Clone,
        ) -> Result<(), Error<ERR>> {
            device
                .move_slice_overlapping_dotted_mapped(
                    position, bytes, dots, delay_ms, direction, map,
                )
                .await
        }

        async fn move_slice_to_end_dotted_mapped(
            device: &mut TM1637<N, Token, CLK, DIO, DELAY>,
            position: usize,
            bytes: &[u8],
            dots: &[bool],
            delay_ms: u32,
            direction: Direction,
            map: impl FnMut(u8) -> u8 + Clone,
        ) -> Result<(), Error<ERR>> {
            device
                .move_slice_to_end_dotted_mapped(position, bytes, dots, delay_ms, direction, map)
                .await
        }
    }

    impl<const N: usize, CLK, DIO, DELAY, ERR> Name<N, Token, CLK, DIO, DELAY, ERR> for Flipped<Token>
    where
        CLK: OutputPin<Error = ERR>,
        DIO: OutputPin<Error = ERR> + ConditionalInputPin<ERR>,
        DELAY: DelayTrait,
    {
        async fn display_slice_dotted_mapped(
            device: &mut TM1637<N, Token, CLK, DIO, DELAY>,
            position: usize,
            bytes: &[u8],
            dots: &[bool],
            map: impl FnMut(u8) -> u8,
        ) -> Result<(), Error<ERR>> {
            device
                .display_slice_flipped_dotted_mapped(position, bytes, dots, map)
                .await
        }

        async fn move_slice_overlapping_dotted_mapped(
            device: &mut TM1637<N, Token, CLK, DIO, DELAY>,
            position: usize,
            bytes: &[u8],
            dots: &[bool],
            delay_ms: u32,
            direction: Direction,
            map: impl FnMut(u8) -> u8 + Clone,
        ) -> Result<(), Error<ERR>> {
            device
                .move_slice_overlapping_flipped_dotted_mapped(
                    position, bytes, dots, delay_ms, direction, map,
                )
                .await
        }

        async fn move_slice_to_end_dotted_mapped(
            device: &mut TM1637<N, Token, CLK, DIO, DELAY>,
            position: usize,
            bytes: &[u8],
            dots: &[bool],
            delay_ms: u32,
            direction: Direction,
            map: impl FnMut(u8) -> u8 + Clone,
        ) -> Result<(), Error<ERR>> {
            device
                .move_slice_to_end_flipped_dotted_mapped(
                    position, bytes, dots, delay_ms, direction, map,
                )
                .await
        }
    }
}
