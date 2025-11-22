//! Brazil-specific installment payment type.

use rust_decimal::Decimal;

use crate::Error;
use crate::internal::Validated;
use crate::types::{InstallmentPlanId, installments::NoInstallments};

/// Installment payment options for Brazil
///
/// Brazil (Parcelamento) supports installment payments with an optional fee parameter.
///
/// The `fee` parameter represents the known installment fee that will be charged
/// to the customer on top of the base amount. When specified, it's used for:
/// - Validation: ensuring total_amount covers base + recipients + fee
/// - Gateway communication: some gateways expect the fee to be explicit
///
/// When `fee` is None, the gateway will calculate the fee internally based on
/// contract terms or stored plan configuration.
#[derive(Clone, Debug, Default)]
pub enum InstallmentsBR {
    /// Single payment (no installments).
    #[default]
    TotalPayment,
    /// Fixed the number of installments (2-99) with an optional fee.
    ///
    /// Fee represents the known installment financing charge. If Some(amount),
    /// this exact fee should be accounted for in total_amount. If None, the
    /// gateway will deduct its fee from the base_amount according to contract.
    FixedPlan { count: u8, fee: Option<Decimal> },
    /// Gateway-specific stored installment plan.
    ///
    /// Fee is pre-calculated and stored in the plan by the gateway.
    StoredPlan { id: InstallmentPlanId },
}

impl Validated for InstallmentsBR {
    fn validate(self) -> Result<Self, Error> {
        match self {
            Self::FixedPlan { count, .. } if count < 2 => Err(Error::InvalidInput(
                "Installment count must be at least 2".to_string(),
            )),
            Self::FixedPlan { fee: Some(fee), .. } if fee < Decimal::ZERO => Err(
                Error::InvalidInput("Installment fee must be non-negative".to_string()),
            ),
            _ => Ok(self),
        }
    }
}

impl From<NoInstallments> for InstallmentsBR {
    fn from(_: NoInstallments) -> Self {
        Self::default()
    }
}

impl<'a> From<crate::Installments<'a>> for InstallmentsBR {
    fn from(input: crate::Installments<'a>) -> Self {
        match input {
            crate::Installments::TotalPayment => Self::TotalPayment,
            crate::Installments::FixedPlan { count } => Self::FixedPlan { count, fee: None },
            crate::Installments::StoredPlan { id } => Self::StoredPlan {
                id: InstallmentPlanId::try_from(id).expect("valid plan id"),
            },
        }
    }
}

impl<'a> TryFrom<crate::InstallmentsBR<'a>> for InstallmentsBR {
    type Error = Error;

    fn try_from(input: crate::InstallmentsBR<'a>) -> Result<Self, Self::Error> {
        match input {
            crate::InstallmentsBR::TotalPayment => Ok(Self::TotalPayment),
            crate::InstallmentsBR::FixedPlan { count, fee } => {
                Self::FixedPlan { count, fee }.validate()
            }
            crate::InstallmentsBR::StoredPlan { id } => Ok(Self::StoredPlan { id: id.try_into()? }),
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
            let result = InstallmentsBR::from(NoInstallments);
            assert!(matches!(result, InstallmentsBR::TotalPayment));
        }
    }

    mod from_base_installments_input {
        use super::*;

        #[test]
        fn converts_total_payment() {
            let result = InstallmentsBR::from(crate::Installments::TotalPayment);
            assert!(matches!(result, InstallmentsBR::TotalPayment));
        }

        #[test]
        fn converts_fixed_plan() {
            let result = InstallmentsBR::from(crate::Installments::FixedPlan { count: 6 });
            assert!(matches!(
                result,
                InstallmentsBR::FixedPlan {
                    count: 6,
                    fee: None
                }
            ));
        }

        #[test]
        fn converts_stored_plan() {
            let result = InstallmentsBR::from(crate::Installments::StoredPlan { id: "INS54434" });
            assert!(matches!(result, InstallmentsBR::StoredPlan { .. }));
        }
    }

    mod from_br_installments_input {
        use super::*;

        #[test]
        fn constructs_total_payment() {
            let result = InstallmentsBR::try_from(crate::InstallmentsBR::TotalPayment).unwrap();
            assert!(matches!(result, InstallmentsBR::TotalPayment));
        }

        #[test]
        fn constructs_fixed_plan_without_fee() {
            let result = InstallmentsBR::try_from(crate::InstallmentsBR::FixedPlan {
                count: 6,
                fee: None,
            })
            .unwrap();
            assert!(matches!(
                result,
                InstallmentsBR::FixedPlan {
                    count: 6,
                    fee: None
                }
            ));
        }

        #[test]
        fn constructs_fixed_plan_with_zero_fee() {
            let result = InstallmentsBR::try_from(crate::InstallmentsBR::FixedPlan {
                count: 6,
                fee: Some(Decimal::ZERO),
            })
            .unwrap();
            assert!(matches!(
                result,
                InstallmentsBR::FixedPlan {
                    count: 6,
                    fee: Some(_)
                }
            ));
        }

        #[test]
        fn constructs_fixed_plan_with_positive_fee() {
            let result = InstallmentsBR::try_from(crate::InstallmentsBR::FixedPlan {
                count: 12,
                fee: Some(Decimal::new(500, 2)),
            })
            .unwrap();
            assert!(matches!(
                result,
                InstallmentsBR::FixedPlan {
                    count: 12,
                    fee: Some(_)
                }
            ));
        }

        #[test]
        fn constructs_stored_plan() {
            let result =
                InstallmentsBR::try_from(crate::InstallmentsBR::StoredPlan { id: "INS54434" })
                    .unwrap();
            assert!(matches!(result, InstallmentsBR::StoredPlan { .. }));
        }

        #[test]
        fn rejects_count_zero() {
            let result = InstallmentsBR::try_from(crate::InstallmentsBR::FixedPlan {
                count: 0,
                fee: None,
            });
            assert!(matches!(result, Err(Error::InvalidInput(_))));
        }

        #[test]
        fn rejects_count_one() {
            let result = InstallmentsBR::try_from(crate::InstallmentsBR::FixedPlan {
                count: 1,
                fee: None,
            });
            assert!(matches!(result, Err(Error::InvalidInput(_))));
        }

        #[test]
        fn rejects_negative_fee() {
            let result = InstallmentsBR::try_from(crate::InstallmentsBR::FixedPlan {
                count: 6,
                fee: Some(Decimal::new(-100, 2)),
            });
            assert!(matches!(result, Err(Error::InvalidInput(_))));
        }

        #[test]
        fn rejects_empty_plan_id() {
            let result = InstallmentsBR::try_from(crate::InstallmentsBR::StoredPlan { id: "" });
            assert!(matches!(result, Err(Error::InvalidInput(_))));
        }
    }
}
