//! Installment payment types for regional payment flows.

mod no_installments;

// --- Marker traits ---

/// Marker trait for installment payment types.
#[expect(dead_code)]
pub(crate) trait Installments {}

// --- Types ---

pub use no_installments::NoInstallments;
