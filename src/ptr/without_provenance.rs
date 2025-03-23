/// [std::ptr::without_provenance]
pub const fn without_provenance<T>(addr: usize) -> *const T {
    addr as *const T
}
