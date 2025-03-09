/// [std::borrow::Borrow]
pub trait Borrow<Borrowed>
where
    Borrowed: ?Sized,
{
    /// [std::borrow::Borrow::borrow]
    fn borrow(&self) -> &Borrowed;
}
