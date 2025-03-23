use std::{hash, num, ptr};
/// [ptr::NonNull]
pub struct NonNull<T>
where
    T: ?Sized,
{
    pointer: *const T,
}

impl<T> NonNull<T> {
    /// [ptr::NonNull::dangling]
    pub const fn dangling() -> NonNull<T> {
        NonNull {
            pointer: ptr::dangling(),
        }
    }
}
impl<T> NonNull<T>
where
    T: ?Sized,
{
    /// [ptr::NonNull::new]
    pub const fn new(ptr: *mut T) -> Option<NonNull<T>> {
        if !ptr.is_null() {
            Some(NonNull { pointer: ptr })
        } else {
            None
        }
    }
    /// [ptr::NonNull::addr]
    pub fn addr(self) -> num::NonZero<usize> {
        num::NonZero::new(self.as_ptr().addr()).unwrap()
    }
    /// [ptr::NonNull::with_addr]
    pub fn with_addr(self, addr: num::NonZero<usize>) -> NonNull<T> {
        NonNull {
            pointer: self.as_ptr().with_addr(addr.get()),
        }
    }
    /// [ptr::NonNull::is_aligned]
    pub fn is_aligned(self) -> bool
    where
        T: Sized,
    {
        self.pointer.is_aligned()
    }
    /// [ptr::NonNull::read]
    pub const unsafe fn read(self) -> T
    where
        T: Sized,
    {
        unsafe { ptr::read(self.pointer) }
    }
    /// [ptr::NonNull::write]
    pub const unsafe fn write(self, val: T)
    where
        T: Sized,
    {
        unsafe { ptr::write(self.pointer as *mut T, val) }
    }
    /// [ptr::NonNull::replace]
    pub unsafe fn replace(self, src: T) -> T
    where
        T: Sized,
    {
        unsafe { ptr::replace(self.pointer as *mut T, src) }
    }
    /// [ptr::NonNull::swap]
    pub const unsafe fn swap(self, with: NonNull<T>)
    where
        T: Sized,
    {
        unsafe { ptr::swap(self.pointer as *mut T, with.pointer as *mut T) }
    }
    /// [ptr::NonNull::as_ptr]
    pub const fn as_ptr(self) -> *mut T {
        self.pointer as *mut T
    }
    /// [ptr::NonNull::as_ref]
    pub const unsafe fn as_ref<'a>(&self) -> &'a T {
        unsafe { &*self.pointer }
    }
    /// [ptr::NonNull::as_mut]
    pub const unsafe fn as_mut<'a>(&mut self) -> &'a mut T {
        unsafe { &mut *(self.pointer as *mut T) }
    }
    /// [ptr::NonNull::offset]
    pub const unsafe fn offset(self, count: isize) -> NonNull<T>
    where
        T: Sized,
    {
        NonNull {
            pointer: unsafe { self.pointer.offset(count) },
        }
    }
    /// [ptr::NonNull::cast]
    pub const fn cast<U>(self) -> NonNull<U> {
        NonNull {
            pointer: self.pointer as *const U,
        }
    }
}
impl<T> NonNull<[T]> {
    /// [ptr::NonNull::slice_from_raw_parts]
    pub const fn slice_from_raw_parts(data: NonNull<T>, len: usize) -> NonNull<[T]> {
        NonNull {
            pointer: ptr::slice_from_raw_parts(data.pointer, len),
        }
    }
    /// [ptr::NonNull::len]
    pub const fn len(self) -> usize {
        self.pointer.len()
    }
}
impl<T> hash::Hash for NonNull<T>
where
    T: ?Sized,
{
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.pointer.hash(state);
    }
}
