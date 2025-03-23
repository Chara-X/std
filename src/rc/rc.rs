use std::{alloc, cell, mem, ops, pin, ptr};
/// [std::rc::Rc]
pub struct Rc<T: ?Sized> {
    counter: ptr::NonNull<Counter>,
    value: ptr::NonNull<T>,
}
impl<T> Rc<T> {
    /// [std::rc::Rc::new]
    pub fn new(value: T) -> Rc<T> {
        Rc {
            counter: Box::leak(Box::new(Counter {
                strong: cell::Cell::new(1),
                weak: cell::Cell::new(0),
            }))
            .into(),
            value: Box::leak(Box::new(value)).into(),
        }
    }
    /// [std::rc::Rc::pin]
    pub fn pin(value: T) -> pin::Pin<Rc<T>> {
        unsafe { pin::Pin::new_unchecked(Rc::new(value)) }
    }
    /// [std::rc::Rc::into_inner]
    pub fn into_inner(this: Rc<T>) -> Option<T> {
        if Rc::strong_count(&this) == 1 {
            let this = mem::ManuallyDrop::new(this);
            unsafe { this.counter.as_ref() }
                .strong
                .set(unsafe { this.counter.as_ref() }.strong.get() - 1);
            let val = unsafe { ptr::read(this.value.as_ptr()) };
            if unsafe { this.counter.as_ref() }.weak.get() == 0 {
                unsafe {
                    ptr::drop_in_place(this.counter.as_ptr());
                    alloc::dealloc(
                        this.counter.as_ptr().cast(),
                        alloc::Layout::for_value(this.counter.as_ref()),
                    );
                }
            }
            unsafe {
                alloc::dealloc(
                    this.value.as_ptr().cast(),
                    alloc::Layout::for_value(this.value.as_ref()),
                );
            }
            Some(val)
        } else {
            None
        }
    }
}
impl<T: ?Sized> Rc<T> {
    /// [std::rc::Rc::strong_count]
    pub fn strong_count(this: &Rc<T>) -> usize {
        unsafe { this.counter.as_ref() }.strong.get()
    }
    /// [std::rc::Rc::weak_count]
    pub fn weak_count(this: &Rc<T>) -> usize {
        unsafe { this.counter.as_ref() }.weak.get()
    }
    /// [std::rc::Rc::downgrade]
    pub fn downgrade(this: &Rc<T>) -> Weak<T> {
        unsafe { this.counter.as_ref() }
            .weak
            .set(unsafe { this.counter.as_ref() }.weak.get() + 1);
        Weak {
            counter: this.counter,
            value: this.value,
        }
    }
    /// [std::rc::Rc::ptr_eq]
    pub fn ptr_eq(this: &Rc<T>, other: &Rc<T>) -> bool {
        ptr::addr_eq(this.counter.as_ptr(), other.counter.as_ptr())
    }
}
impl<T> ops::Deref for Rc<T>
where
    T: ?Sized,
{
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { self.value.as_ref() }
    }
}
impl<T> Clone for Rc<T>
where
    T: ?Sized,
{
    fn clone(&self) -> Self {
        unsafe { self.counter.as_ref() }
            .strong
            .set(unsafe { self.counter.as_ref() }.strong.get() + 1);
        Rc {
            counter: self.counter,
            value: self.value,
        }
    }
}
impl<T> Drop for Rc<T>
where
    T: ?Sized,
{
    fn drop(&mut self) {
        unsafe { self.counter.as_ref() }
            .strong
            .set(unsafe { self.counter.as_ref() }.strong.get() - 1);
        if unsafe { self.counter.as_ref() }.strong.get() == 0 {
            if unsafe { self.counter.as_ref() }.weak.get() == 0 {
                unsafe {
                    ptr::drop_in_place(self.counter.as_ptr());
                    alloc::dealloc(
                        self.counter.as_ptr().cast(),
                        alloc::Layout::for_value(self.counter.as_ref()),
                    );
                }
            }
            unsafe {
                ptr::drop_in_place(self.value.as_ptr());
                alloc::dealloc(
                    self.value.as_ptr().cast(),
                    alloc::Layout::for_value(self.value.as_ref()),
                );
            }
        }
    }
}
/// [std::rc::Weak]
pub struct Weak<T: ?Sized> {
    counter: ptr::NonNull<Counter>,
    value: ptr::NonNull<T>,
}
impl<T: ?Sized> Weak<T> {
    /// [std::rc::Weak::strong_count]
    pub fn strong_count(this: &Self) -> usize {
        unsafe { this.counter.as_ref() }.strong.get()
    }
    /// [std::rc::Weak::weak_count]
    pub fn weak_count(this: &Self) -> usize {
        unsafe { this.counter.as_ref() }.weak.get()
    }
    /// [std::rc::Weak::upgrade]
    pub fn upgrade(&self) -> Option<Rc<T>> {
        let strong = unsafe { self.counter.as_ref() }.strong.get();
        if strong > 0 {
            unsafe { self.counter.as_ref() }.strong.set(strong + 1);
            Some(Rc {
                counter: self.counter,
                value: self.value,
            })
        } else {
            None
        }
    }
    /// [std::rc::Weak::ptr_eq]
    pub fn ptr_eq(&self, other: &Weak<T>) -> bool {
        ptr::addr_eq(self.counter.as_ptr(), other.counter.as_ptr())
    }
}
impl<T> Clone for Weak<T>
where
    T: ?Sized,
{
    fn clone(&self) -> Self {
        unsafe { self.counter.as_ref() }
            .weak
            .set(unsafe { self.counter.as_ref() }.weak.get() + 1);
        Weak {
            counter: self.counter,
            value: self.value,
        }
    }
}
impl<T> Drop for Weak<T>
where
    T: ?Sized,
{
    fn drop(&mut self) {
        unsafe { self.counter.as_ref() }
            .weak
            .set(unsafe { self.counter.as_ref() }.weak.get() - 1);
        if unsafe { self.counter.as_ref() }.strong.get() == 0
            && unsafe { self.counter.as_ref() }.weak.get() == 0
        {
            unsafe {
                ptr::drop_in_place(self.counter.as_ptr());
                alloc::dealloc(
                    self.counter.as_ptr().cast(),
                    alloc::Layout::for_value(self.counter.as_ref()),
                );
            }
        }
    }
}
struct Counter {
    strong: cell::Cell<usize>,
    weak: cell::Cell<usize>,
}
