/// [std::array::from_fn]
pub fn from_fn<T, const N: usize, F>(mut cb: F) -> [T; N]
where
    F: FnMut(usize) -> T,
{
    let mut arr = [(); N].map(|_| cb(0));
    arr.iter_mut()
        .enumerate()
        .for_each(|(i, elem)| *elem = cb(i));
    arr
}
