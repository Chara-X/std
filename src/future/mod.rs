//! [std::future]
mod future;
mod into_future;
mod pending;
mod poll_fn;
mod ready;
pub use self::future::*;
pub use self::into_future::*;
pub use self::pending::*;
pub use self::poll_fn::*;
pub use self::ready::*;
