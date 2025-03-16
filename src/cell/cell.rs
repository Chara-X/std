use super::*;
use std::mem;
/// [std::cell::Cell]
pub struct Cell<T>
where
    T: ?Sized,
{
    value: UnsafeCell<T>,
}
impl<T> Cell<T> {
    /// [std::cell::Cell::new]
    pub const fn new(value: T) -> Cell<T> {
        Cell {
            value: UnsafeCell::new(value),
        }
    }
    /// [std::cell::Cell::replace]
    pub fn replace(&self, val: T) -> T {
        mem::replace(unsafe { &mut *self.value.get() }, val)
    }
    /// [std::cell::Cell::swap]
    pub fn swap(&self, other: &Cell<T>) {
        mem::swap(unsafe { &mut *self.value.get() }, unsafe {
            &mut *other.value.get()
        });
    }
    /// [std::cell::Cell::into_inner]
    pub fn into_inner(self) -> T {
        self.value.into_inner()
    }
}
impl<T> Cell<T>
where
    T: Copy,
{
    /// [std::cell::Cell::get]
    pub fn get(&self) -> T {
        unsafe { *self.value.get() }
    }
}
impl<T> Cell<T>
where
    T: Default,
{
    /// [std::cell::Cell::take]
    pub fn take(&self) -> T {
        self.replace(Default::default())
    }
}
