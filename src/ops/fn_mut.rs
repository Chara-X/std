use super::*;
/// [std::ops::FnMut]
pub trait FnMut<Args>: FnOnce<Args> {
    /// [std::ops::FnMut::call_mut]
    fn call_mut(&mut self, args: Args) -> Self::Output;
}
