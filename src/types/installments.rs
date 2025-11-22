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
