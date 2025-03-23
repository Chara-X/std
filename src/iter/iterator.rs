/// [std::iter::Iterator]
pub trait Iterator {
    /// [std::iter::Iterator::Item]
    type Item;
    /// [std::iter::Iterator::next]
    fn next(&mut self) -> Option<Self::Item>;
}
