#![allow(unsafe_code)]
use super::*;
/// [std::cell::OnceCell]
pub struct OnceCell<T> {
    value: UnsafeCell<Option<T>>,
}
impl<T> OnceCell<T> {
    /// [std::cell::OnceCell::new]
    pub const fn new() -> OnceCell<T> {
        OnceCell {
            value: UnsafeCell::new(None),
        }
    }
    /// [std::cell::OnceCell::set]
    pub fn set(&self, value: T) -> Result<(), T> {
        if self.get().is_some() {
            return Err(value);
        }
        unsafe { *self.value.get() = Some(value) };
        Ok(())
    }
    /// [std::cell::OnceCell::get]
    pub fn get(&self) -> Option<&T> {
        unsafe { &*self.value.get() }.as_ref()
    }
    /// [std::cell::OnceCell::into_inner]
    pub fn into_inner(self) -> Option<T> {
        self.value.into_inner()
    }
}
