#![allow(unsafe_code)]
use super::*;
use std::{mem, ops};
/// [std::cell::RefCell]
pub struct RefCell<T>
where
    T: ?Sized,
{
    borrow: Cell<BorrowFlag>,
    value: UnsafeCell<T>,
}
impl<T> RefCell<T> {
    /// [std::cell::RefCell::new]
    pub const fn new(value: T) -> RefCell<T> {
        RefCell {
            borrow: Cell::new(BorrowFlag::Reading(0)),
            value: UnsafeCell::new(value),
        }
    }
    /// [std::cell::RefCell::replace]
    pub fn replace(&self, t: T) -> T {
        mem::replace(&mut *self.borrow_mut(), t)
    }
    /// [std::cell::RefCell::swap]
    pub fn swap(&self, other: &RefCell<T>) {
        mem::swap(&mut *self.borrow_mut(), &mut *other.borrow_mut())
    }
    /// [std::cell::RefCell::into_inner]
    pub fn into_inner(self) -> T {
        self.value.into_inner()
    }
}
impl<T> RefCell<T>
where
    T: ?Sized,
{
    /// [std::cell::RefCell::borrow]
    pub fn borrow(&self) -> Ref<'_, T> {
        match self.borrow.get() {
            BorrowFlag::Reading(n) => {
                self.borrow.replace(BorrowFlag::Reading(n + 1));
                Ref {
                    borrow: &self.borrow,
                    value: unsafe { &*self.value.get() },
                }
            }
            _ => panic!(),
        }
    }
    /// [std::cell::RefCell::borrow_mut]
    pub fn borrow_mut(&self) -> RefMut<'_, T> {
        match self.borrow.get() {
            BorrowFlag::Reading(0) => {
                self.borrow.replace(BorrowFlag::Writing);
                RefMut {
                    borrow: &self.borrow,
                    value: unsafe { &mut *self.value.get() },
                }
            }
            _ => panic!(),
        }
    }
}
impl<T> RefCell<T>
where
    T: Default,
{
    /// [std::cell::RefCell::take]
    pub fn take(&self) -> T {
        self.replace(Default::default())
    }
}
/// [std::cell::Ref]
pub struct Ref<'b, T>
where
    T: 'b + ?Sized,
{
    borrow: &'b Cell<BorrowFlag>,
    value: &'b T,
}
impl<'b, T> Ref<'b, T>
where
    T: ?Sized,
{
    /// [std::cell::Ref::clone]
    pub fn clone(orig: &Ref<'b, T>) -> Ref<'b, T> {
        match orig.borrow.get() {
            BorrowFlag::Reading(n) => {
                orig.borrow.replace(BorrowFlag::Reading(n + 1));
                Ref {
                    borrow: orig.borrow,
                    value: orig.value,
                }
            }
            _ => panic!(),
        }
    }
}
impl<T> ops::Deref for Ref<'_, T>
where
    T: ?Sized,
{
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.value
    }
}
impl<T> Drop for Ref<'_, T>
where
    T: ?Sized,
{
    fn drop(&mut self) {
        match self.borrow.get() {
            BorrowFlag::Reading(n) => {
                self.borrow.replace(BorrowFlag::Reading(n - 1));
            }
            _ => panic!(),
        }
    }
}
/// [std::cell::RefMut]
pub struct RefMut<'b, T>
where
    T: 'b + ?Sized,
{
    borrow: &'b Cell<BorrowFlag>,
    value: &'b mut T,
}
impl<T> ops::Deref for RefMut<'_, T>
where
    T: ?Sized,
{
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.value
    }
}
impl<T> ops::DerefMut for RefMut<'_, T>
where
    T: ?Sized,
{
    fn deref_mut(&mut self) -> &mut T {
        self.value
    }
}
impl<T> Drop for RefMut<'_, T>
where
    T: ?Sized,
{
    fn drop(&mut self) {
        match self.borrow.get() {
            BorrowFlag::Writing => {
                self.borrow.replace(BorrowFlag::Reading(0));
            }
            _ => panic!(),
        }
    }
}
#[derive(Copy, Clone)]
enum BorrowFlag {
    Reading(isize),
    Writing,
}
