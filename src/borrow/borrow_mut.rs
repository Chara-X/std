use super::*;
/// [std::borrow::BorrowMut]
pub trait BorrowMut<Borrowed>: Borrow<Borrowed>
where
    Borrowed: ?Sized,
{
    /// [std::borrow::BorrowMut::borrow_mut]
    fn borrow_mut(&mut self) -> &mut Borrowed;
}
