use super::*;
/// [std::cmp::min]
pub fn min<T>(v1: T, v2: T) -> T
where
    T: Ord,
{
    min_by(v1, v2, Ord::cmp)
}
