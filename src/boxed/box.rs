use std::alloc;
use std::ops;
use std::pin;
use std::ptr;
/// [std::boxed::Box]
pub struct Box<T: ?Sized>(ptr::NonNull<T>);
impl<T> Box<T> {
    /// [std::boxed::Box::new]
    pub fn new(x: T) -> Box<T> {
        let ptr = unsafe { alloc::alloc(alloc::Layout::new::<T>()) as *mut T };
        unsafe { ptr::write(ptr, x) };
        Box(ptr::NonNull::new(ptr).unwrap())
    }
    /// [std::boxed::Box::pin]
    pub fn pin(x: T) -> pin::Pin<Box<T>> {
        unsafe { pin::Pin::new_unchecked(Box::new(x)) }
    }
}
impl<T> ops::Deref for Box<T>
where
    T: ?Sized,
{
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { self.0.as_ref() }
    }
}
impl<T> ops::DerefMut for Box<T>
where
    T: ?Sized,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.0.as_mut() }
    }
}
impl<T> Drop for Box<T>
where
    T: ?Sized,
{
    fn drop(&mut self) {
        unsafe {
            alloc::dealloc(
                self.0.as_ptr().cast(),
                alloc::Layout::for_value(self.0.as_ref()),
            )
        };
    }
}
