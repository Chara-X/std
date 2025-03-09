use std::cmp;
/// [cmp::min_by]
pub fn min_by<T, F>(v1: T, v2: T, compare: F) -> T
where
    F: FnOnce(&T, &T) -> cmp::Ordering,
{
    match compare(&v1, &v2) {
        cmp::Ordering::Less | cmp::Ordering::Equal => v1,
        cmp::Ordering::Greater => v2,
    }
}
