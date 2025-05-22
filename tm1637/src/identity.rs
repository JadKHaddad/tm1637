/// Identity trait.
///
/// Used to trick the compiler while using [`duplicate_item`](::duplicate::duplicate_item) to implement `async` and `blocking` versions of the same module.
/// Using this trait, we can write normal rust code that can also be formatted by `rustfmt`.
pub trait Identity: Sized {
    fn identity(self) -> Self {
        self
    }
}

impl<T: Sized> Identity for T {}
