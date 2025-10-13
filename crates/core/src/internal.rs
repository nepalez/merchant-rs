//! The module keeps sealed internal traits and types used exclusively
//! by types in this crate to DRY the code and prevent errors.

mod safe_wrapper;
mod sanitized;
mod secret_string;
mod validated;

pub(crate) use safe_wrapper::SafeWrapper;
pub(crate) use sanitized::Sanitized;
pub(crate) use secret_string::SecretString;
pub(crate) use validated::Validated;
