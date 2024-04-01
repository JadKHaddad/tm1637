//! Device functionality in async and blocking modes.

#[cfg(feature = "async")]
pub mod asynchronous;
#[cfg(feature = "blocking")]
pub mod blocking;

/// A bit. For private use.
pub(crate) enum Bit {
    /// Zero.
    ZERO,
    /// One.
    ONE,
}

/// Error type for TM1637 devices.
pub enum TM1637Error<ERR> {
    /// Acknowledge error. The device did not acknowledge the sent byte.
    Ack,
    /// Digital error.
    Digital(ERR),
}

impl<ERR> core::fmt::Debug for TM1637Error<ERR>
where
    ERR: core::fmt::Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Digital(err) => write!(f, "Digital error: {err:?}"),
            Self::Ack => write!(f, "Device did not acknowledge the sent byte",),
        }
    }
}

/// Main functionality for TM1637 devices.
pub(crate) trait BaseTM1637<CLK, DIO, DELAY> {
    fn clk(&self) -> &CLK;

    fn clk_mut(&mut self) -> &mut CLK;

    fn dio(&self) -> &DIO;

    fn dio_mut(&mut self) -> &mut DIO;

    fn delay(&self) -> &DELAY;

    fn delay_mut(&mut self) -> &mut DELAY;

    fn delay_us(&self) -> u32;

    fn address_count(&self) -> u8;
}
