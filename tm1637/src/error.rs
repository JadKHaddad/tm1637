/// An Error type for the `TM1637` driver.
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[non_exhaustive]
pub enum Error<ERR> {
    #[cfg(feature = "ack")]
    /// Acknowledge error. The display did not acknowledge the sent byte.
    Ack,
    /// Digital error.
    Digital(ERR),
}

impl<ERR> From<ERR> for Error<ERR> {
    fn from(err: ERR) -> Self {
        Error::Digital(err)
    }
}
