use super::*;
/// [std::iter::FromIterator]
pub trait FromIterator<A>: Sized {
    /// [std::iter::FromIterator::from_iter]
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self;
}
