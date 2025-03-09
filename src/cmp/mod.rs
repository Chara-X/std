//! [std::cmp]
mod eq;
mod max;
mod max_by;
mod max_by_key;
mod min;
mod min_by;
mod min_by_key;
mod ord;
mod partial_eq;
mod partial_ord;
mod reverse;
pub use self::eq::*;
pub use self::max::*;
pub use self::max_by::*;
pub use self::max_by_key::*;
pub use self::min::*;
pub use self::min_by::*;
pub use self::min_by_key::*;
pub use self::ord::*;
pub use self::partial_eq::*;
pub use self::partial_ord::*;
pub use self::reverse::*;
