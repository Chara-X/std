/// [std::ptr::null]
pub const fn null<T>() -> *const T {
    0 as *const T
}
