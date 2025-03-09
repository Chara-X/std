use super::*;
use std::cmp;
/// [cmp::Ord]
pub trait Ord: Eq + PartialOrd {
    /// [cmp::Ord::cmp]
    fn cmp(&self, other: &Self) -> cmp::Ordering;
}
