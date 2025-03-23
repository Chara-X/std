use std::fmt;
/// [std::error]
pub trait Error: fmt::Debug + fmt::Display {}
