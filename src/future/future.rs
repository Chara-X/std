use std::{pin, task};
/// [std::future::Future]
pub trait Future {
    /// [std::future::Future::Output]
    type Output;
    /// [std::future::Future::poll]
    fn poll(self: pin::Pin<&mut Self>, cx: &mut task::Context<'_>) -> task::Poll<Self::Output>;
}
