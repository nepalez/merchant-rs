//! The module keeps sealed internal traits and types used exclusively
//! by types in this crate to DRY the code and prevent errors.

mod masked;
mod validated;

mod as_unsafe_ref;
mod payment_source;
pub(crate) mod sanitized;

pub use as_unsafe_ref::AsUnsafeRef;
pub(crate) use masked::Masked;
pub(crate) use payment_source::PaymentSource;
pub(crate) use validated::Validated;
