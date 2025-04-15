#![allow(missing_docs)]
use std::{
    any::{self, Any},
    fmt,
};
/// [std::error]
pub trait Error: fmt::Debug + fmt::Display {
    /// [std::error::Error::source]
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        todo!()
    }
}
impl dyn Error + 'static {
    pub fn is<T: Error + 'static>(&self) -> bool {
        self.type_id() == any::TypeId::of::<T>()
    }
    pub fn downcast_ref<T: Error + 'static>(&self) -> Option<&T> {
        if self.is::<T>() {
            unsafe { Some(&*(self as *const dyn Error as *const T)) }
        } else {
            None
        }
    }
    pub fn downcast_mut<T: Error + 'static>(&mut self) -> Option<&mut T> {
        if self.is::<T>() {
            unsafe { Some(&mut *(self as *mut dyn Error as *mut T)) }
        } else {
            None
        }
    }
}
