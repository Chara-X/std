//! [std::marker]
mod copy;
mod phantom_data;
mod phantom_pinned;
mod send;
mod sized;
mod sync;
mod unpin;
pub use self::copy::*;
pub use self::phantom_data::*;
pub use self::phantom_pinned::*;
pub use self::send::*;
pub use self::sized::*;
pub use self::sync::*;
pub use self::unpin::*;
