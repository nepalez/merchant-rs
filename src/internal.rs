//! The module keeps sealed internal traits and types used exclusively
//! by types in this crate to DRY the code and prevent errors.

mod exposed;
pub(crate) mod sanitized;
pub(crate) mod validated;

// Internal traits
pub(crate) use exposed::Exposed;
