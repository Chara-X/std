use super::*;
/// [std::iter::from_fn]
pub fn from_fn<T, F>(f: F) -> FromFn<F>
where
    F: FnMut() -> Option<T>,
{
    FromFn(f)
}
/// [std::iter::FromFn]
pub struct FromFn<F>(F);
impl<T, F> Iterator for FromFn<F>
where
    F: FnMut() -> Option<T>,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0()
    }
}
