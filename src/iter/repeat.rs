/// [std::iter::repeat]
pub fn repeat<T: Clone>(elt: T) -> Repeat<T> {
    Repeat { element: elt }
}
/// [std::iter::Repeat]
pub struct Repeat<A> {
    element: A,
}
impl<A> Iterator for Repeat<A>
where
    A: Clone,
{
    type Item = A;
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.element.clone())
    }
}
