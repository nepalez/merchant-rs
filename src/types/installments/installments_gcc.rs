//! Gulf countries-specific installment payment type.

use crate::Error;
use crate::internal::Validated;
use crate::types::{InstallmentPlanId, installments::NoInstallments};

/// Installment payment options for Gulf countries.
///
/// Gulf countries (UAE, Saudi Arabia, Kuwait, Qatar, Bahrain, Oman) support
/// installment payments with an optional Shariah compliance parameter.
///
/// The `shariah_compliant` parameter indicates whether the installment plan
/// follows Islamic finance principles (Murabaha, Ijarah, Istisna'a structures).
/// When true, it's used for:
/// - Gateway communication: the gateway applies Shariah-compliant terms
/// - Compliance: ensures adherence to Islamic banking regulations
///
/// When `shariah_compliant` is false, standard interest-based installments apply.
#[derive(Clone, Debug, Default)]
pub enum InstallmentsGCC {
    /// Single payment (no installments).
    #[default]
    TotalPayment,
    /// A fixed number of installments (2-99) with a Shariah compliance option.
    ///
    /// `shariah_compliant` indicates whether the plan follows Islamic finance
    /// principles. If true, the gateway applies Shariah-compliant structures
    /// (Murabaha, Ijarah, Istisna'a). If false, standard interest-based terms apply.
    FixedPlan { count: u8, shariah_compliant: bool },
    /// Gateway-specific stored installment plan.
    ///
    /// Shariah compliance is determined at plan creation time, not per-transaction.
    StoredPlan { id: InstallmentPlanId },
}

impl Validated for InstallmentsGCC {
    fn validate(self) -> Result<Self, Error> {
        match self {
            Self::FixedPlan { count, .. } if count < 2 => Err(Error::InvalidInput(
                "Installment count must be at least 2".to_string(),
            )),
            _ => Ok(self),
        }
    }
}

impl From<NoInstallments> for InstallmentsGCC {
    fn from(_: NoInstallments) -> Self {
        Self::default()
    }
}

impl<'a> From<crate::Installments<'a>> for InstallmentsGCC {
    fn from(input: crate::Installments<'a>) -> Self {
        match input {
            crate::Installments::TotalPayment => Self::TotalPayment,
            crate::Installments::FixedPlan { count } => Self::FixedPlan {
                count,
                shariah_compliant: false,
            },
            crate::Installments::StoredPlan { id } => Self::StoredPlan {
                id: InstallmentPlanId::try_from(id).expect("valid plan id"),
            },
        }
    }
}

impl<'a> TryFrom<crate::InstallmentsGCC<'a>> for InstallmentsGCC {
    type Error = Error;

    fn try_from(input: crate::InstallmentsGCC<'a>) -> Result<Self, Self::Error> {
        match input {
            crate::InstallmentsGCC::TotalPayment => Ok(Self::TotalPayment),
            crate::InstallmentsGCC::FixedPlan {
                count,
                shariah_compliant,
            } => Self::FixedPlan {
                count,
                shariah_compliant,
            }
            .validate(),
            crate::InstallmentsGCC::StoredPlan { id } => {
                Ok(Self::StoredPlan { id: id.try_into()? })
            }
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
            let result = InstallmentsGCC::from(NoInstallments);
            assert!(matches!(result, InstallmentsGCC::TotalPayment));
        }
    }

    mod from_base_installments_input {
        use super::*;

        #[test]
        fn converts_total_payment() {
            let result = InstallmentsGCC::from(crate::Installments::TotalPayment);
            assert!(matches!(result, InstallmentsGCC::TotalPayment));
        }

        #[test]
        fn converts_fixed_plan() {
            let result = InstallmentsGCC::from(crate::Installments::FixedPlan { count: 6 });
            assert!(matches!(
                result,
                InstallmentsGCC::FixedPlan {
                    count: 6,
                    shariah_compliant: false
                }
            ));
        }

        #[test]
        fn converts_stored_plan() {
            let result = InstallmentsGCC::from(crate::Installments::StoredPlan { id: "INS54434" });
            assert!(matches!(result, InstallmentsGCC::StoredPlan { .. }));
        }
    }

    mod from_gcc_installments_input {
        use super::*;

        #[test]
        fn constructs_total_payment() {
            let result = InstallmentsGCC::try_from(crate::InstallmentsGCC::TotalPayment).unwrap();
            assert!(matches!(result, InstallmentsGCC::TotalPayment));
        }

        #[test]
        fn constructs_fixed_plan_non_compliant() {
            let result = InstallmentsGCC::try_from(crate::InstallmentsGCC::FixedPlan {
                count: 6,
                shariah_compliant: false,
            })
            .unwrap();
            assert!(matches!(
                result,
                InstallmentsGCC::FixedPlan {
                    count: 6,
                    shariah_compliant: false
                }
            ));
        }

        #[test]
        fn constructs_fixed_plan_shariah_compliant() {
            let result = InstallmentsGCC::try_from(crate::InstallmentsGCC::FixedPlan {
                count: 12,
                shariah_compliant: true,
            })
            .unwrap();
            assert!(matches!(
                result,
                InstallmentsGCC::FixedPlan {
                    count: 12,
                    shariah_compliant: true
                }
            ));
        }

        #[test]
        fn constructs_stored_plan() {
            let result =
                InstallmentsGCC::try_from(crate::InstallmentsGCC::StoredPlan { id: "INS54434" })
                    .unwrap();
            assert!(matches!(result, InstallmentsGCC::StoredPlan { .. }));
        }

        #[test]
        fn rejects_count_zero() {
            let result = InstallmentsGCC::try_from(crate::InstallmentsGCC::FixedPlan {
                count: 0,
                shariah_compliant: false,
            });
            assert!(matches!(result, Err(Error::InvalidInput(_))));
        }

        #[test]
        fn rejects_count_one() {
            let result = InstallmentsGCC::try_from(crate::InstallmentsGCC::FixedPlan {
                count: 1,
                shariah_compliant: false,
            });
            assert!(matches!(result, Err(Error::InvalidInput(_))));
        }

        #[test]
        fn rejects_empty_plan_id() {
            let result = InstallmentsGCC::try_from(crate::InstallmentsGCC::StoredPlan { id: "" });
            assert!(matches!(result, Err(Error::InvalidInput(_))));
        }
    }
}
