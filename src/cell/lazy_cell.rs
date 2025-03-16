use super::*;
use std::ops;
/// [std::cell::LazyCell]
pub struct LazyCell<T, F = fn() -> T> {
    value: UnsafeCell<State<T, F>>,
}
impl<T, F> LazyCell<T, F>
where
    F: FnOnce() -> T,
{
    /// [std::cell::LazyCell::new]
    pub const fn new(f: F) -> LazyCell<T, F> {
        LazyCell {
            value: UnsafeCell::new(State::Uninit(f)),
        }
    }
}
impl<T, F> ops::Deref for LazyCell<T, F>
where
    F: FnMut() -> T,
{
    type Target = T;
    fn deref(&self) -> &Self::Target {
        match unsafe { &mut *self.value.get() } {
            State::Uninit(f) => unsafe {
                *self.value.get() = State::Init(f());
                match &*self.value.get() {
                    State::Init(data) => data,
                    _ => unreachable!(),
                }
            },
            State::Init(data) => data,
        }
    }
}
enum State<T, F> {
    Uninit(F),
    Init(T),
}
