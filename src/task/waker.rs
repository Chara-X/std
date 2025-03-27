use super::*;
use std::sync;
/// [std::task::Waker]
pub struct Waker {
    waker: sync::Arc<dyn Wake + Send + Sync + 'static>,
}
impl Waker {
    /// [std::task::Waker::wake]
    pub fn wake(self) {
        self.waker.wake();
    }
    /// [std::task::Waker::wake_by_ref]
    pub fn wake_by_ref(&self) {
        self.waker.clone().wake();
    }
}
impl<W> From<sync::Arc<W>> for Waker
where
    W: Wake + Send + Sync + 'static,
{
    fn from(value: sync::Arc<W>) -> Self {
        Waker { waker: value }
    }
}
impl Clone for Waker {
    fn clone(&self) -> Self {
        Self {
            waker: self.waker.clone(),
        }
    }
}
