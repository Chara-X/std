use super::*;
use std::{pin, task};
/// [std::future::ready]
pub fn ready<T>(t: T) -> Ready<T> {
    Ready(Some(t))
}
/// [std::future::Ready]
pub struct Ready<T>(Option<T>);
impl<T> Unpin for Ready<T> {}
impl<T> Future for Ready<T> {
    type Output = T;
    fn poll(mut self: pin::Pin<&mut Self>, cx: &mut task::Context<'_>) -> task::Poll<T> {
        task::Poll::Ready(self.0.take().unwrap())
    }
}
