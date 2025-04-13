use super::*;
use std::option;
/// [std::iter::once]
pub fn once<T>(value: T) -> Once<T> {
    Once(Some(value).into_iter())
}
/// [std::iter::Once]
pub struct Once<T>(option::IntoIter<T>);
impl<T> Iterator for Once<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}
