use super::TM1637;

pub enum ErasedTM1637<T, CLK, DIO, DELAY> {
    Four(TM1637<4, T, CLK, DIO, DELAY>),
    Six(TM1637<6, T, CLK, DIO, DELAY>),
}

impl<T, CLK, DIO, DELAY> From<TM1637<4, T, CLK, DIO, DELAY>> for ErasedTM1637<T, CLK, DIO, DELAY> {
    fn from(value: TM1637<4, T, CLK, DIO, DELAY>) -> Self {
        Self::Four(value)
    }
}

impl<T, CLK, DIO, DELAY> From<TM1637<6, T, CLK, DIO, DELAY>> for ErasedTM1637<T, CLK, DIO, DELAY> {
    fn from(value: TM1637<6, T, CLK, DIO, DELAY>) -> Self {
        Self::Six(value)
    }
}

#[::duplicate::duplicate_item(
    module        async     await               Token                         DelayTrait;
    [asynch]      [async]   [await.identity()]  [crate::tokens::Async]        [::embedded_hal_async::delay::DelayNs];
    [blocking]    []        [identity()]        [crate::tokens::Blocking]     [::embedded_hal::delay::DelayNs];
)]
pub mod module {
    use embedded_hal::digital::OutputPin;

    use crate::{
        options::erased::ErasedDisplayOptions, tokens::NotFlipped, Brightness, ConditionalInputPin,
        Error, Identity,
    };

    use super::ErasedTM1637;

    impl<CLK, DIO, DELAY, ERR> ErasedTM1637<Token, CLK, DIO, DELAY>
    where
        CLK: OutputPin<Error = ERR>,
        DIO: OutputPin<Error = ERR> + ConditionalInputPin<ERR>,
        DELAY: DelayTrait,
    {
        pub async fn init(&mut self) -> Result<(), crate::Error<ERR>> {
            match self {
                Self::Four(tm) => tm.init().await,
                Self::Six(tm) => tm.init().await,
            }
        }

        pub async fn on(&mut self) -> Result<(), Error<ERR>> {
            match self {
                Self::Four(tm) => tm.on().await,
                Self::Six(tm) => tm.on().await,
            }
        }

        pub async fn off(&mut self) -> Result<(), Error<ERR>> {
            match self {
                Self::Four(tm) => tm.off().await,
                Self::Six(tm) => tm.off().await,
            }
        }

        pub async fn clear(&mut self) -> Result<(), Error<ERR>> {
            match self {
                Self::Four(tm) => tm.clear().await,
                Self::Six(tm) => tm.clear().await,
            }
        }

        pub async fn set_brightness(&mut self, brightness: Brightness) -> Result<(), Error<ERR>> {
            match self {
                Self::Four(tm) => tm.set_brightness(brightness).await,
                Self::Six(tm) => tm.set_brightness(brightness).await,
            }
        }

        pub async fn display(
            &mut self,
            position: usize,
            bytes: impl Iterator<Item = u8>,
        ) -> Result<(), Error<ERR>> {
            match self {
                Self::Four(tm) => tm.display(position, bytes).await,
                Self::Six(tm) => tm.display(position, bytes).await,
            }
        }

        pub async fn display_slice(
            &mut self,
            position: usize,
            bytes: &[u8],
        ) -> Result<(), Error<ERR>> {
            match self {
                Self::Four(tm) => tm.display_slice(position, bytes).await,
                Self::Six(tm) => tm.display_slice(position, bytes).await,
            }
        }

        pub fn options(
            &mut self,
        ) -> ErasedDisplayOptions<
            '_,
            Token,
            CLK,
            DIO,
            DELAY,
            impl DoubleEndedIterator<Item = u8> + ExactSizeIterator,
            NotFlipped,
        > {
            match self {
                Self::Four(tm) => ErasedDisplayOptions::Four(tm.options()),
                Self::Six(tm) => ErasedDisplayOptions::Six(tm.options()),
            }
        }
    }
}
