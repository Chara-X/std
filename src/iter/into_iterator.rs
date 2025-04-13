use super::*;
/// [std::iter::IntoIterator]
pub trait IntoIterator {
    /// [std::iter::IntoIterator::Item]
    type Item;
    /// [std::iter::IntoIterator::IntoIter]
    type IntoIter: Iterator<Item = Self::Item>;
    /// [std::iter::IntoIterator::into_iter]
    fn into_iter(self) -> Self::IntoIter;
}
