//! Installment payment types for regional payment flows.

#[allow(clippy::module_inception)]
mod installments;
mod installments_br;
mod installments_gcc;
mod installments_in;
mod installments_jp;
mod no_installments;

// --- Types ---

pub use installments::Installments;
pub use installments_br::InstallmentsBR;
pub use installments_gcc::InstallmentsGCC;
pub use installments_in::InstallmentsIN;
pub use installments_jp::InstallmentsJP;
pub use no_installments::NoInstallments;

// --- Marker Traits ---

/// Marker trait for types that can be used as installment payment options.
pub(crate) trait InstallmentsMarker {}

impl InstallmentsMarker for NoInstallments {}
impl InstallmentsMarker for Installments {}
impl InstallmentsMarker for InstallmentsBR {}
impl InstallmentsMarker for InstallmentsIN {}
impl InstallmentsMarker for InstallmentsJP {}
impl InstallmentsMarker for InstallmentsGCC {}
