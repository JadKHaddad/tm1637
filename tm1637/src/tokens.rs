use core::marker::PhantomData;

/// Token for `async` operations.
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Async;

/// Token for `blocking` operations.
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Blocking;

/// Token for a `non-flipped` display.
///
/// Can be `async` or `blocking`.
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct NotFlipped<T>(PhantomData<T>);

impl<T> NotFlipped<T> {
    /// Create a new [`NotFlipped`] token.
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

impl<T> Default for NotFlipped<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Token for a `flipped` display.
///
/// Can be `async` or `blocking`.
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Flipped<T>(PhantomData<T>);

impl<T> Flipped<T> {
    /// Create a new [`Flipped`] token.
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

impl<T> Default for Flipped<T> {
    fn default() -> Self {
        Self::new()
    }
}
