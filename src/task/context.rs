use super::*;
/// [std::task::Context]
pub struct Context<'a> {
    waker: &'a Waker,
}
impl<'a> Context<'a> {
    /// [std::task::Context::from_waker]
    pub const fn from_waker(waker: &'a Waker) -> Self {
        Self { waker }
    }
    /// [std::task::Context::waker]
    pub const fn waker(&self) -> &'a Waker {
        self.waker
    }
}
