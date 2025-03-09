use super::*;
use std::cmp;
/// [cmp::PartialOrd]
pub trait PartialOrd<Rhs = Self>: PartialEq<Rhs>
where
    Rhs: ?Sized,
{
    /// [cmp::PartialOrd::partial_cmp]
    fn partial_cmp(&self, other: &Rhs) -> Option<cmp::Ordering>;
}
