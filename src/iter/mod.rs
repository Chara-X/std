//! [std::iter]
mod empty;
mod extend;
mod from_fn;
mod into_iterator;
mod iterator;
mod once;
mod once_with;
mod repeat;
mod repeat_with;
pub use self::empty::*;
pub use self::extend::*;
pub use self::from_fn::*;
pub use self::into_iterator::*;
pub use self::iterator::*;
pub use self::once::*;
pub use self::once_with::*;
pub use self::repeat::*;
pub use self::repeat_with::*;
