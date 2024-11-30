use core::marker::PhantomData;

#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Async;

#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Blocking;

#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct NotFlipped<T>(PhantomData<T>);

impl<T> NotFlipped<T> {
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

impl<T> Default for NotFlipped<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Flipped<T>(PhantomData<T>);

impl<T> Flipped<T> {
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

impl<T> Default for Flipped<T> {
    fn default() -> Self {
        Self::new()
    }
}
