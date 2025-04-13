use super::*;
use std::marker;
/// [std::iter::empty]
pub const fn empty<T>() -> Empty<T> {
    Empty(marker::PhantomData)
}
/// [std::iter::Empty]
pub struct Empty<T>(marker::PhantomData<T>);
impl<T> Iterator for Empty<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}
