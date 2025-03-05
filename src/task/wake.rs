use std::sync;
/// [std::task::Wake]
pub trait Wake {
    /// [std::task::Wake::wake]
    fn wake(self: sync::Arc<Self>);
}
