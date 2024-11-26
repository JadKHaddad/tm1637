//! Asynchronous demo module.
//!
//! This module is only available when the `demo` and `async` features of this
//! library are activated.

use embedded_hal::digital::OutputPin;
use embedded_hal_async::delay::DelayNs;

use crate::{asynch::TM1637, mappings::DigitBits, ConditionalInputPin, Error};

/// Asynchronous demo.
#[derive(Debug)]
pub struct Demo<CLK, DIO, DELAY, ERR>
where
    CLK: OutputPin<Error = ERR>,
    DIO: OutputPin<Error = ERR>,
    DELAY: DelayNs,
{
    device: TM1637<4, CLK, DIO, DELAY>,
    delay: DELAY,
}
impl<CLK, DIO, DELAY, ERR> Demo<CLK, DIO, DELAY, ERR>
where
    ERR: core::fmt::Debug,
    CLK: OutputPin<Error = ERR>,
    DIO: OutputPin<Error = ERR> + ConditionalInputPin<ERR>,
    DELAY: DelayNs,
{
    /// Create a new demo instance.
    pub fn new(device: TM1637<4, CLK, DIO, DELAY>, delay: DELAY) -> Self {
        Self { device, delay }
    }

    /// Create a timer that counts down from 9 to 0 at the first position.
    pub async fn timer(&mut self) -> Result<(), Error<ERR>> {
        for i in (0..=9).rev() {
            self.device
                .write_segments_raw(0, &[DigitBits::from_digit(i) as u8])
                .await?;
            self.delay.delay_ms(1000).await;
        }

        self.device
            .write_segments_raw(
                0,
                &[
                    DigitBits::Zero,
                    DigitBits::Zero,
                    DigitBits::Zero,
                    DigitBits::Zero,
                ]
                .map(|d| d as u8),
            )
            .await?;

        for _ in 0..5 {
            self.delay.delay_ms(300).await;
            self.device.off().await?;
            self.delay.delay_ms(300).await;
            self.device.on().await?;
        }

        self.delay.delay_ms(300).await;

        self.device.clear().await?;

        Ok(())
    }
}
