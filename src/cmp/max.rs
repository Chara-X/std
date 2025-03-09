use super::*;
/// [std::cmp::max]
pub fn max<T>(v1: T, v2: T) -> T
where
    T: Ord,
{
    max_by(v1, v2, Ord::cmp)
}
