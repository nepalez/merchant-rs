//! Base installment payment type for most regions.

use crate::Error;
use crate::internal::Validated;
use crate::types::{InstallmentPlanId, installments::NoInstallments};

/// Installment payment options for most regions
///
/// Used by gateways that support installment payments in regions like
/// North America, Europe, most of Asia, CIS, Oceania, Africa, and Turkey.
///
/// Supports three payment modes:
/// - Single payment (no installments)
/// - Fixed installment count (2-99)
/// - Gateway-specific stored plan
#[derive(Clone, Debug, Default)]
pub enum Installments {
    /// Single payment (no installments).
    #[default]
    TotalPayment,
    /// A fixed number of installments (2-99).
    FixedPlan { count: u8 },
    /// Gateway-specific stored installment plan.
    StoredPlan { id: InstallmentPlanId },
}

impl Validated for Installments {
    fn validate(self) -> Result<Self, Error> {
        match self {
            Self::FixedPlan { count } if count < 2 => Err(Error::InvalidInput(
                "Installment count must be at least 2".to_string(),
            )),
            _ => Ok(self),
        }
    }
}

impl From<NoInstallments> for Installments {
    fn from(_: NoInstallments) -> Self {
        Self::default()
    }
}

impl<'a> TryFrom<crate::Installments<'a>> for Installments {
    type Error = Error;

    fn try_from(input: crate::Installments<'a>) -> Result<Self, Self::Error> {
        match input {
            crate::Installments::TotalPayment => Ok(Self::TotalPayment),
            crate::Installments::FixedPlan { count } => Self::FixedPlan { count }.validate(),
            crate::Installments::StoredPlan { id } => Ok(Self::StoredPlan { id: id.try_into()? }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_from_no_installments() {
        let installments = Installments::from(NoInstallments);
        assert!(matches!(installments, Installments::TotalPayment));
    }

    mod from_input {
        use super::*;

        #[test]
        fn constructs_total_payment() {
            let result = Installments::try_from(crate::Installments::TotalPayment).unwrap();
            assert!(matches!(result, Installments::TotalPayment));
        }

        #[test]
        fn constructs_fixed_plan() {
            let result =
                Installments::try_from(crate::Installments::FixedPlan { count: 6 }).unwrap();
            assert!(matches!(result, Installments::FixedPlan { count: 6 }));
        }

        #[test]
        fn constructs_stored_plan() {
            let result =
                Installments::try_from(crate::Installments::StoredPlan { id: "INS54434" }).unwrap();
            assert!(matches!(result, Installments::StoredPlan { .. }));
        }

        #[test]
        fn rejects_count_zero() {
            let result = Installments::try_from(crate::Installments::FixedPlan { count: 0 });
            assert!(matches!(result, Err(Error::InvalidInput(_))));
        }

        #[test]
        fn rejects_count_one() {
            let result = Installments::try_from(crate::Installments::FixedPlan { count: 1 });
            assert!(matches!(result, Err(Error::InvalidInput(_))));
        }

        #[test]
        fn rejects_empty_plan_id() {
            let result = Installments::try_from(crate::Installments::StoredPlan { id: "" });
            assert!(matches!(result, Err(Error::InvalidInput(_))));
        }
    }
}
