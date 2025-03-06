use std::{pin, task};
/// [std::future::poll_fn]
pub fn poll_fn<T, F>(f: F) -> PollFn<F>
where
    F: FnMut(&mut task::Context<'_>) -> task::Poll<T>,
    F: Unpin,
{
    PollFn { f }
}
/// [std::future::PollFn]
pub struct PollFn<F> {
    f: F,
}
impl<T, F> Future for PollFn<F>
where
    F: FnMut(&mut task::Context<'_>) -> task::Poll<T>,
    F: Unpin,
{
    type Output = T;
    fn poll(self: pin::Pin<&mut Self>, cx: &mut task::Context<'_>) -> task::Poll<T> {
        (self.get_mut().f)(cx)
    }
}
