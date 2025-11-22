//! Japan-specific installment payment type.

use crate::Error;
use crate::internal::Validated;
use crate::types::{InstallmentPlanId, installments::NoInstallments};

/// Installment payment options for Japan.
///
/// Japan (分割払い - Bunkatsu-barai) supports multiple installment payment types
/// beyond standard fixed-count plans.
///
/// - **Regular**: Fixed installments (2-99 payments)
/// - **Revolving**: Revolving credit plan
/// - **Bonus**: Two payments per year (typically July and December bonuses)
#[derive(Clone, Debug, Default)]
pub enum InstallmentsJP {
    /// Single payment (no installments).
    #[default]
    TotalPayment,
    /// A fixed number of installments (2-99).
    FixedPlan { count: u8 },
    /// Revolving credit plan.
    ///
    /// The customer makes flexible monthly payments with interest charges.
    RevolvingPlan,
    /// Bonus payment plan.
    ///
    /// Two payments per year, typically aligned with Japanese bonus seasons
    /// (summer in July, winter in December).
    BonusPlan,
    /// Gateway-specific stored installment plan.
    StoredPlan { id: InstallmentPlanId },
}

impl Validated for InstallmentsJP {
    fn validate(self) -> Result<Self, Error> {
        match self {
            Self::FixedPlan { count } if count < 2 => Err(Error::InvalidInput(
                "Installment count must be at least 2".to_string(),
            )),
            _ => Ok(self),
        }
    }
}

impl From<NoInstallments> for InstallmentsJP {
    fn from(_: NoInstallments) -> Self {
        Self::default()
    }
}

impl<'a> From<crate::Installments<'a>> for InstallmentsJP {
    fn from(input: crate::Installments<'a>) -> Self {
        match input {
            crate::Installments::TotalPayment => Self::TotalPayment,
            crate::Installments::FixedPlan { count } => Self::FixedPlan { count },
            crate::Installments::StoredPlan { id } => Self::StoredPlan {
                id: InstallmentPlanId::try_from(id).expect("valid plan id"),
            },
        }
    }
}

impl<'a> TryFrom<crate::InstallmentsJP<'a>> for InstallmentsJP {
    type Error = Error;

    fn try_from(input: crate::InstallmentsJP<'a>) -> Result<Self, Self::Error> {
        match input {
            crate::InstallmentsJP::TotalPayment => Ok(Self::TotalPayment),
            crate::InstallmentsJP::FixedPlan { count } => Self::FixedPlan { count }.validate(),
            crate::InstallmentsJP::RevolvingPlan => Ok(Self::RevolvingPlan),
            crate::InstallmentsJP::BonusPlan => Ok(Self::BonusPlan),
            crate::InstallmentsJP::StoredPlan { id } => Ok(Self::StoredPlan { id: id.try_into()? }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod from_no_installments {
        use super::*;

        #[test]
        fn converts_to_total_payment() {
            let result = InstallmentsJP::from(NoInstallments);
            assert!(matches!(result, InstallmentsJP::TotalPayment));
        }
    }

    mod from_base_installments_input {
        use super::*;

        #[test]
        fn converts_total_payment() {
            let result = InstallmentsJP::from(crate::Installments::TotalPayment);
            assert!(matches!(result, InstallmentsJP::TotalPayment));
        }

        #[test]
        fn converts_fixed_plan() {
            let result = InstallmentsJP::from(crate::Installments::FixedPlan { count: 6 });
            assert!(matches!(result, InstallmentsJP::FixedPlan { count: 6 }));
        }

        #[test]
        fn converts_stored_plan() {
            let result = InstallmentsJP::from(crate::Installments::StoredPlan { id: "INS54434" });
            assert!(matches!(result, InstallmentsJP::StoredPlan { .. }));
        }
    }

    mod from_jp_installments_input {
        use super::*;

        #[test]
        fn constructs_total_payment() {
            let result = InstallmentsJP::try_from(crate::InstallmentsJP::TotalPayment).unwrap();
            assert!(matches!(result, InstallmentsJP::TotalPayment));
        }

        #[test]
        fn constructs_fixed_plan() {
            let result =
                InstallmentsJP::try_from(crate::InstallmentsJP::FixedPlan { count: 6 }).unwrap();
            assert!(matches!(result, InstallmentsJP::FixedPlan { count: 6 }));
        }

        #[test]
        fn constructs_revolving_plan() {
            let result = InstallmentsJP::try_from(crate::InstallmentsJP::RevolvingPlan).unwrap();
            assert!(matches!(result, InstallmentsJP::RevolvingPlan));
        }

        #[test]
        fn constructs_bonus_plan() {
            let result = InstallmentsJP::try_from(crate::InstallmentsJP::BonusPlan).unwrap();
            assert!(matches!(result, InstallmentsJP::BonusPlan));
        }

        #[test]
        fn constructs_stored_plan() {
            let result =
                InstallmentsJP::try_from(crate::InstallmentsJP::StoredPlan { id: "INS54434" })
                    .unwrap();
            assert!(matches!(result, InstallmentsJP::StoredPlan { .. }));
        }

        #[test]
        fn rejects_count_zero() {
            let result = InstallmentsJP::try_from(crate::InstallmentsJP::FixedPlan { count: 0 });
            assert!(matches!(result, Err(Error::InvalidInput(_))));
        }

        #[test]
        fn rejects_count_one() {
            let result = InstallmentsJP::try_from(crate::InstallmentsJP::FixedPlan { count: 1 });
            assert!(matches!(result, Err(Error::InvalidInput(_))));
        }

        #[test]
        fn rejects_empty_plan_id() {
            let result = InstallmentsJP::try_from(crate::InstallmentsJP::StoredPlan { id: "" });
            assert!(matches!(result, Err(Error::InvalidInput(_))));
        }
    }
}
