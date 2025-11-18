//! Installment payment types for regional payment flows.

mod extended_plan;
mod fixed_plan;
mod mexico_plan;
mod no_installments;

// --- Marker traits ---

/// Marker trait for installment payment types.
#[expect(dead_code)]
pub(crate) trait Installments {}

// --- Types ---

pub use extended_plan::ExtendedPlan;
pub use fixed_plan::FixedPlan;
pub use mexico_plan::MexicoPlan;
pub use no_installments::NoInstallments;
