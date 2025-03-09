use super::*;
/// [std::borrow::ToOwned]
pub trait ToOwned {
    /// [std::borrow::ToOwned::Owned]
    type Owned: Borrow<Self>;
    /// [std::borrow::ToOwned::to_owned]
    fn to_owned(&self) -> Self::Owned;
}
