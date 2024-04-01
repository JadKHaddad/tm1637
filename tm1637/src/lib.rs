#![no_std]
#![deny(unsafe_code)]
// TODO: #![deny(missing_docs)]

/// Our custom `try!` macro aka `?`, to get rid of [`core::convert::From`]/[`core::convert::Into`] used by the `?` operator.
macro_rules! tri {
    ($e:expr $(,)?) => {
        match $e {
            core::result::Result::Ok(value) => value,
            core::result::Result::Err(err) => {
                return core::result::Result::Err(err);
            }
        }
    };
}

/// Our custom `try!` macro aka `?`, to get rid of [`core::convert::From`]/[`core::convert::Into`] used by the `?` operator while still converting digital errors to our custom error type.
macro_rules! tri_digital {
    ($e:expr $(,)?) => {
        match $e {
            core::result::Result::Ok(value) => value,
            core::result::Result::Err(err) => {
                return core::result::Result::Err(TM1637Error::Digital(err));
            }
        }
    };
}

#[cfg(feature = "demo")]
pub mod demo;
pub mod device;
pub mod functionality;
#[cfg(feature = "mappings")]
pub mod mappings;
