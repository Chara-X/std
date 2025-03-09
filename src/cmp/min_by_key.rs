use super::*;
/// [std::cmp::min_by_key]
pub fn min_by_key<T, F, K>(v1: T, v2: T, mut f: F) -> T
where
    F: FnMut(&T) -> K,
    K: Ord,
{
    min_by(v1, v2, |v1, v2| f(v1).cmp(&f(v2)))
}
