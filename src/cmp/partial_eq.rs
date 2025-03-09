/// [std::cmp::PartialEq]
pub trait PartialEq<Rhs = Self>
where
    Rhs: ?Sized,
{
    /// [std::cmp::PartialEq::eq]
    fn eq(&self, other: &Rhs) -> bool;
}
