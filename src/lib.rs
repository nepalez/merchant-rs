use crate::types::installments;

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

/// Marker trait for types that can be used as installment payment options.
pub(crate) trait InstallmentsMarker {}

impl InstallmentsMarker for installments::NoInstallments {}
impl InstallmentsMarker for installments::Installments {}
impl InstallmentsMarker for installments::InstallmentsBR {}
impl InstallmentsMarker for installments::InstallmentsIN {}
impl InstallmentsMarker for installments::InstallmentsJP {}
impl InstallmentsMarker for installments::InstallmentsGCC {}

/// Root trait for payment gateway adapters.
#[allow(private_bounds)]
pub trait Gateway: Send + Sync {
    /// The payment method supported by this gateway.
    type PaymentMethod: types::PaymentMethod;

    /// The installment payment options supported by this gateway.
    type Installments: InstallmentsMarker;
}
