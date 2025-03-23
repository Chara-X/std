/// [std::slice::from_ref]
pub const fn from_ref<T>(s: &T) -> &[T] {
    unsafe { &*(s as *const T).cast::<[T; 1]>() }
}
