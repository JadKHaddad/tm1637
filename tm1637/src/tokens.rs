use crate::mode::Mode;

/// Token for `async` operations.
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Async;

impl Mode for Async {}

/// Token for `blocking` operations.
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Blocking;

impl Mode for Blocking {}

/// Token for a `non-flipped` display.
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct NotFlipped;

/// Token for a `flipped` display.
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Flipped;
