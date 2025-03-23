use super::*;
/// [std::ops::DerefMut]
pub trait DerefMut: Deref {
    /// [std::ops::DerefMut::deref_mut]
    fn deref_mut(&mut self) -> &mut Self::Target;
}
