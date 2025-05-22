//! Noop implementation of embedded-hal traits for testing purposes.

#[derive(Debug)]
pub struct Noop;

mod digital {
    use embedded_hal::digital::{Error, ErrorKind, ErrorType, InputPin, OutputPin};

    use super::Noop;

    #[derive(Debug)]
    pub struct Err;

    impl Error for Err {
        fn kind(&self) -> ErrorKind {
            unreachable!()
        }
    }

    impl ErrorType for Noop {
        type Error = Err;
    }

    impl OutputPin for Noop {
        fn set_low(&mut self) -> Result<(), Self::Error> {
            Ok(())
        }

        fn set_high(&mut self) -> Result<(), Self::Error> {
            Ok(())
        }
    }

    impl InputPin for Noop {
        fn is_high(&mut self) -> Result<bool, Self::Error> {
            Ok(true)
        }

        fn is_low(&mut self) -> Result<bool, Self::Error> {
            Ok(false)
        }
    }
}

mod delay {
    use super::Noop;

    impl ::embedded_hal_async::delay::DelayNs for Noop {
        async fn delay_ns(&mut self, _: u32) {}
    }

    impl ::embedded_hal::delay::DelayNs for Noop {
        fn delay_ns(&mut self, _: u32) {}
    }
}
