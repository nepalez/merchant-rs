mod as_unsafe_ref;
mod error;
mod internal;

mod enums;
mod inputs;

pub mod flows;
pub mod types;

pub use as_unsafe_ref::AsUnsafeRef;
pub use enums::*;
pub use error::Error;
pub use inputs::*;

/// Marker trait for types that can be used as payment amount distribution.
pub(crate) trait DistributionMarker {}
impl DistributionMarker for types::NoDistribution {}
impl DistributionMarker for Option<types::Recipients> {}

/// Root trait for payment gateway adapters.
#[allow(private_bounds)]
pub trait Gateway: Send + Sync {
    /// The payment method supported by this gateway.
    type PaymentMethod: types::PaymentMethod;

    /// The amount distribution model supported by this gateway.
    type AmountDistribution: DistributionMarker;

    /// The installment payment options supported by this gateway.
    type Installments: types::InstallmentsMarker;
}
