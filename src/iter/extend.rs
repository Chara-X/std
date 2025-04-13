use super::*;
/// [std::iter::Extend]
pub trait Extend<A> {
    /// [std::iter::Extend::extend]
    fn extend<T: IntoIterator<Item = A>>(&mut self, iter: T);
}
