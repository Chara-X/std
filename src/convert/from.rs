/// [std::convert::From<T>]
pub trait From<T>: Sized {
    /// [std::convert::From::from]
    fn from(value: T) -> Self;
}
