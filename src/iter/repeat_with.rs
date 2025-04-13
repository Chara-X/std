use super::*;
/// [std::iter::repeat_with]
pub fn repeat_with<A, F>(repeater: F) -> RepeatWith<F>
where
    F: FnMut() -> A,
{
    RepeatWith { repeater }
}
/// [std::iter::RepeatWith]
pub struct RepeatWith<F> {
    repeater: F,
}
impl<A, F> Iterator for RepeatWith<F>
where
    F: FnMut() -> A,
{
    type Item = A;
    fn next(&mut self) -> Option<Self::Item> {
        Some((self.repeater)())
    }
}
