mod as_unsafe_ref;
mod error;
mod internal;

mod enums;
pub mod flows;
pub mod inputs;
pub mod types;

pub use as_unsafe_ref::AsUnsafeRef;
pub use enums::*;
pub use error::Error;

/// Root trait for payment gateway adapters.
#[allow(private_bounds)]
pub trait Gateway: Send + Sync {
    /// The payment method supported by this gateway.
    type PaymentMethod: types::PaymentMethod;
}
