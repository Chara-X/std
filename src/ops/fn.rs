use super::*;
/// [std::ops::Fn]
pub trait Fn<Args>: FnMut<Args> {
    /// [std::ops::Fn::call]
    fn call(&self, args: Args) -> Self::Output;
}
