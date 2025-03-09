use super::*;
/// [std::convert::Into<T>]
pub trait Into<T>: Sized {
    /// [std::convert::Into::into]
    fn into(self) -> T;
}
impl<T, U> Into<U> for T
where
    U: From<T>,
{
    fn into(self) -> U {
        U::from(self)
    }
}
