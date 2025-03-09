use std::cmp;
/// [cmp::max_by]
pub fn max_by<T, F>(v1: T, v2: T, compare: F) -> T
where
    F: FnOnce(&T, &T) -> cmp::Ordering,
{
    match compare(&v1, &v2) {
        cmp::Ordering::Less | cmp::Ordering::Equal => v2,
        cmp::Ordering::Greater => v1,
    }
}
