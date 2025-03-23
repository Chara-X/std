#![deny(missing_docs)]
#![deny(unsafe_code)]
#![allow(unsafe_code)]
#![allow(unused_variables)]
#![allow(clippy::new_without_default)]
#![allow(clippy::len_without_is_empty)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::module_inception)]
#![allow(private_bounds)]
#![allow(clippy::should_implement_trait)]
//! [std]
pub mod array;
pub mod borrow;
pub mod boxed;
pub mod cell;
pub mod clone;
pub mod cmp;
pub mod convert;
pub mod default;
pub mod error;
pub mod future;
pub mod iter;
pub mod marker;
pub mod ops;
pub mod option;
pub mod pin;
pub mod ptr;
pub mod rc;
pub mod result;
pub mod slice;
pub mod task;
