//! The module keeps sealed internal traits and types used exclusively
//! by types in this crate to DRY the code and prevent errors.

mod masked;
mod validated;

pub(crate) mod sanitized;

pub(crate) use masked::Masked;
pub(crate) use validated::Validated;
