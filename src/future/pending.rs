use std::{marker, pin, task};
/// [std::future::pending]
pub fn pending<T>() -> Pending<T> {
    Pending {
        _data: marker::PhantomData,
    }
}
/// [std::future::Pending]
pub struct Pending<T> {
    _data: marker::PhantomData<T>,
}
impl<T> Future for Pending<T> {
    type Output = T;
    fn poll(self: pin::Pin<&mut Self>, _: &mut task::Context<'_>) -> task::Poll<T> {
        task::Poll::Pending
    }
}
