//! [std::ops]
mod deref;
mod deref_mut;
mod drop;
mod r#fn;
mod fn_mut;
mod fn_once;
pub use self::deref::*;
pub use self::deref_mut::*;
pub use self::drop::*;
pub use self::fn_mut::*;
pub use self::fn_once::*;
pub use self::r#fn::*;
