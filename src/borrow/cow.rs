use super::*;
use std::ops;
/// [std::borrow::Cow]
pub enum Cow<'a, B>
where
    B: 'a + ToOwned + ?Sized,
{
    /// [std::borrow::Cow::Owned]
    Owned(<B as ToOwned>::Owned),
    /// [std::borrow::Cow::Borrowed]
    Borrowed(&'a B),
}
impl<B> Cow<'_, B>
where
    B: ToOwned + ?Sized,
{
    /// [std::borrow::Cow::to_mut]
    pub fn to_mut(&mut self) -> &mut <B as ToOwned>::Owned {
        match *self {
            Cow::Owned(ref mut owned) => owned,
            Cow::Borrowed(borrowed) => {
                *self = Cow::Owned(borrowed.to_owned());
                match self {
                    Cow::Owned(owned) => owned,
                    Cow::Borrowed(..) => unreachable!(),
                }
            }
        }
    }
}
impl<B> ops::Deref for Cow<'_, B>
where
    B: ToOwned + ?Sized,
{
    type Target = B;
    fn deref(&self) -> &Self::Target {
        match self {
            Cow::Owned(owned) => owned.borrow(),
            Cow::Borrowed(borrowed) => borrowed,
        }
    }
}
