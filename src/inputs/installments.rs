//! Insecure input types for installment payments.

mod extended_plan;
mod fixed_plan;
mod mexico_plan;

pub use extended_plan::ExtendedPlan;
pub use fixed_plan::FixedPlan;
pub use mexico_plan::MexicoPlan;
