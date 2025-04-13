use super::*;
/// [std::iter::once_with]
pub fn once_with<A, F>(r#gen: F) -> OnceWith<F>
where
    F: FnOnce() -> A,
{
    OnceWith { r#gen: Some(r#gen) }
}
/// [std::iter::OnceWith]
pub struct OnceWith<F> {
    r#gen: Option<F>,
}
impl<A, F> Iterator for OnceWith<F>
where
    F: FnOnce() -> A,
{
    type Item = A;
    fn next(&mut self) -> Option<Self::Item> {
        Some((self.r#gen.take()?)())
    }
}
