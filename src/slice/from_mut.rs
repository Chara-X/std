/// [std::slice::from_mut]
pub const fn from_mut<T>(s: &mut T) -> &mut [T] {
    unsafe { &mut *(s as *mut T).cast::<[T; 1]>() }
}
