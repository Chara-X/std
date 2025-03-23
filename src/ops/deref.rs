/// [std::ops::Deref]
pub trait Deref {
    /// [std::ops::Deref::Target]
    type Target: ?Sized;
    /// [std::ops::Deref::deref]
    fn deref(&self) -> &Self::Target;
}
