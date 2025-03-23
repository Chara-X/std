/// [std::ops::FnOnce]
pub trait FnOnce<Args> {
    /// [std::ops::FnOnce::Output]
    type Output;
    /// [std::ops::FnOnce::call_once]
    fn call_once(self, args: Args) -> Self::Output;
}
