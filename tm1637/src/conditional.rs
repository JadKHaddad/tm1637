/// Conditional input pin trait.
///
/// Used for conditional requirement of the [`InputPin`](::embedded_hal::digital::InputPin) trait based on the `ack` feature.
/// - Every [`InputPin`](::embedded_hal::digital::InputPin) implements this trait, if the `ack` feature is enabled.
/// - Everything implements this trait, if the `ack` feature is not enabled.
pub trait ConditionalInputPin<ERR> {
    /// Is the input pin low?
    fn is_low(&mut self) -> Result<bool, ERR> {
        Ok(false)
    }
}

#[cfg(feature = "ack")]
const _: () = {
    use ::embedded_hal::digital::InputPin;

    impl<ERR, T> ConditionalInputPin<ERR> for T
    where
        T: InputPin<Error = ERR>,
    {
        fn is_low(&mut self) -> Result<bool, ERR> {
            self.is_low()
        }
    }
};

#[cfg(not(feature = "ack"))]
impl<ERR, T> ConditionalInputPin<ERR> for T {}
