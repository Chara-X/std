#![allow(unsafe_code)]
/// [std::cell::UnsafeCell]
pub struct UnsafeCell<T>
where
    T: ?Sized,
{
    value: T,
}
impl<T> UnsafeCell<T> {
    /// [std::cell::UnsafeCell::new]
    pub const fn new(value: T) -> UnsafeCell<T> {
        UnsafeCell { value }
    }
    /// [std::cell::UnsafeCell::into_inner]
    pub fn into_inner(self) -> T {
        self.value
    }
}
impl<T> UnsafeCell<T>
where
    T: ?Sized,
{
    /// [std::cell::UnsafeCell::get]
    pub const fn get(&self) -> *mut T {
        self as *const UnsafeCell<T> as *const T as *mut T
    }
}
unsafe impl<T> Sync for UnsafeCell<T> where T: ?Sized {}
