mod as_unsafe_ref;
mod enums;
mod error;
mod inputs;
mod internal;

use types::payments::PaymentMarker;

pub mod flows;
pub mod types;

pub use as_unsafe_ref::AsUnsafeRef;
pub use enums::*;
pub use error::Error;
pub use inputs::*;

/// Root trait for payment gateway adapters.
#[allow(private_bounds)]
pub trait Gateway: Send + Sync {
    /// The payment type supported by this gateway.
    type Payment: PaymentMarker;

    /// The installment payment options supported by this gateway.
    type Installments: types::InstallmentsMarker;
}
