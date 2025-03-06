use std::ops;
/// [std::pin::Pin]
pub struct Pin<Ptr> {
    pointer: Ptr,
}
impl<T> Pin<&'static T>
where
    T: ?Sized,
{
    /// [std::pin::Pin::static_ref]
    pub const fn static_ref(r: &'static T) -> Pin<&'static T> {
        Pin { pointer: r }
    }
}
impl<T> Pin<&'static mut T>
where
    T: ?Sized,
{
    /// [std::pin::Pin::static_mut]
    pub fn static_mut(r: &'static mut T) -> Pin<&'static mut T> {
        Pin { pointer: r }
    }
}
impl<Ptr> Pin<Ptr>
where
    Ptr: ops::Deref,
    <Ptr as ops::Deref>::Target: Unpin,
{
    /// [std::pin::Pin::new]
    pub const fn new(pointer: Ptr) -> Pin<Ptr> {
        Pin { pointer }
    }
    /// [std::pin::Pin::into_inner]
    pub fn into_inner(pin: Pin<Ptr>) -> Ptr {
        pin.pointer
    }
}
impl<Ptr> Pin<Ptr>
where
    Ptr: ops::Deref,
{
    /// [std::pin::Pin::as_ref]
    pub fn as_ref(&self) -> Pin<&<Ptr as ops::Deref>::Target> {
        Pin {
            pointer: &*self.pointer,
        }
    }
}
impl<Ptr> Pin<Ptr>
where
    Ptr: ops::DerefMut,
{
    /// [std::pin::Pin::as_mut]
    pub fn as_mut(&mut self) -> Pin<&mut <Ptr as ops::Deref>::Target> {
        Pin {
            pointer: &mut *self.pointer,
        }
    }
    /// [std::pin::Pin::set]
    pub fn set(&mut self, value: <Ptr as ops::Deref>::Target)
    where
        <Ptr as ops::Deref>::Target: Sized,
    {
        *self.pointer = value
    }
}
impl<'a, T> Pin<&'a T>
where
    T: ?Sized,
{
    /// [std::pin::Pin::get_ref]
    pub const fn get_ref(self) -> &'a T {
        self.pointer
    }
}
impl<'a, T> Pin<&'a mut T>
where
    T: ?Sized,
{
    /// [std::pin::Pin::get_mut]
    pub fn get_mut(self) -> &'a mut T
    where
        T: Unpin,
    {
        self.pointer
    }
}
impl<Ptr> ops::Deref for Pin<Ptr>
where
    Ptr: ops::Deref,
{
    type Target = Ptr::Target;
    fn deref(&self) -> &Ptr::Target {
        self.as_ref().get_ref()
    }
}
impl<Ptr> ops::DerefMut for Pin<Ptr>
where
    Ptr: ops::DerefMut,
    <Ptr as ops::Deref>::Target: Unpin,
{
    fn deref_mut(&mut self) -> &mut Ptr::Target {
        self.as_mut().get_mut()
    }
}
