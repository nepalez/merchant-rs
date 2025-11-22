//! India-specific installment payment type.

use crate::Error;
use crate::internal::Validated;
use crate::types::{InstallmentPlanId, OfferId, installments::NoInstallments};

/// Installment payment options for India.
///
/// India (EMI - Equated Monthly Installments) supports installment payments
/// with an optional `offer_id` parameter for No Cost EMI.
///
/// The `offer_id` parameter represents a pre-configured No Cost EMI offer where
/// the merchant absorbs the interest charges. When specified, it's used for:
/// - Gateway communication: the gateway applies the offer terms.
/// - No Cost EMI: the customer pays the base amount, the merchant pays interest to the bank.
///
/// When `offer_id` is None, standard EMI applies where the customer pays interest.
#[derive(Clone, Debug, Default)]
pub enum InstallmentsIN {
    /// Single payment (no installments).
    #[default]
    TotalPayment,
    /// A fixed number of installments (2-99) with an optional No Cost EMI offer.
    ///
    /// If Some(id), the merchant absorbs interest charges. If None, the customer
    /// pays standard EMI interest.
    FixedPlan {
        count: u8,
        offer_id: Option<OfferId>,
    },
    /// Gateway-specific stored installment plan with an optional No Cost EMI offer.
    ///
    /// `offer_id` can be specified even for stored plans to apply promotional terms.
    StoredPlan {
        id: InstallmentPlanId,
        offer_id: Option<OfferId>,
    },
}

impl Validated for InstallmentsIN {
    fn validate(self) -> Result<Self, Error> {
        match self {
            Self::FixedPlan { count, .. } if count < 2 => Err(Error::InvalidInput(
                "Installment count must be at least 2".to_string(),
            )),
            _ => Ok(self),
        }
    }
}

impl From<NoInstallments> for InstallmentsIN {
    fn from(_: NoInstallments) -> Self {
        Self::default()
    }
}

impl<'a> From<crate::Installments<'a>> for InstallmentsIN {
    fn from(input: crate::Installments<'a>) -> Self {
        match input {
            crate::Installments::TotalPayment => Self::TotalPayment,
            crate::Installments::FixedPlan { count } => Self::FixedPlan {
                count,
                offer_id: None,
            },
            crate::Installments::StoredPlan { id } => Self::StoredPlan {
                id: InstallmentPlanId::try_from(id).expect("valid plan id"),
                offer_id: None,
            },
        }
    }
}

impl<'a> TryFrom<crate::InstallmentsIN<'a>> for InstallmentsIN {
    type Error = Error;

    fn try_from(input: crate::InstallmentsIN<'a>) -> Result<Self, Self::Error> {
        match input {
            crate::InstallmentsIN::TotalPayment => Ok(Self::TotalPayment),
            crate::InstallmentsIN::FixedPlan { count, offer_id } => Self::FixedPlan {
                count,
                offer_id: offer_id.map(TryInto::try_into).transpose()?,
            }
            .validate(),
            crate::InstallmentsIN::StoredPlan { id, offer_id } => Ok(Self::StoredPlan {
                id: id.try_into()?,
                offer_id: offer_id.map(TryInto::try_into).transpose()?,
            }),
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
            let result = InstallmentsIN::from(NoInstallments);
            assert!(matches!(result, InstallmentsIN::TotalPayment));
        }
    }

    mod from_base_installments_input {
        use super::*;

        #[test]
        fn converts_total_payment() {
            let result = InstallmentsIN::from(crate::Installments::TotalPayment);
            assert!(matches!(result, InstallmentsIN::TotalPayment));
        }

        #[test]
        fn converts_fixed_plan() {
            let result = InstallmentsIN::from(crate::Installments::FixedPlan { count: 6 });
            assert!(matches!(
                result,
                InstallmentsIN::FixedPlan {
                    count: 6,
                    offer_id: None
                }
            ));
        }

        #[test]
        fn converts_stored_plan() {
            let result = InstallmentsIN::from(crate::Installments::StoredPlan { id: "INS54434" });
            assert!(matches!(
                result,
                InstallmentsIN::StoredPlan { offer_id: None, .. }
            ));
        }
    }

    mod from_in_installments_input {
        use super::*;

        #[test]
        fn constructs_total_payment() {
            let result = InstallmentsIN::try_from(crate::InstallmentsIN::TotalPayment).unwrap();
            assert!(matches!(result, InstallmentsIN::TotalPayment));
        }

        #[test]
        fn constructs_fixed_plan_without_offer() {
            let result = InstallmentsIN::try_from(crate::InstallmentsIN::FixedPlan {
                count: 6,
                offer_id: None,
            })
            .unwrap();
            assert!(matches!(
                result,
                InstallmentsIN::FixedPlan {
                    count: 6,
                    offer_id: None
                }
            ));
        }

        #[test]
        fn constructs_fixed_plan_with_offer() {
            let result = InstallmentsIN::try_from(crate::InstallmentsIN::FixedPlan {
                count: 12,
                offer_id: Some("OFFER123"),
            })
            .unwrap();
            assert!(matches!(
                result,
                InstallmentsIN::FixedPlan {
                    count: 12,
                    offer_id: Some(_)
                }
            ));
        }

        #[test]
        fn constructs_stored_plan_without_offer() {
            let result = InstallmentsIN::try_from(crate::InstallmentsIN::StoredPlan {
                id: "INS54434",
                offer_id: None,
            })
            .unwrap();
            assert!(matches!(
                result,
                InstallmentsIN::StoredPlan { offer_id: None, .. }
            ));
        }

        #[test]
        fn constructs_stored_plan_with_offer() {
            let result = InstallmentsIN::try_from(crate::InstallmentsIN::StoredPlan {
                id: "INS54434",
                offer_id: Some("OFFER456"),
            })
            .unwrap();
            assert!(matches!(
                result,
                InstallmentsIN::StoredPlan {
                    offer_id: Some(_),
                    ..
                }
            ));
        }

        #[test]
        fn rejects_count_zero() {
            let result = InstallmentsIN::try_from(crate::InstallmentsIN::FixedPlan {
                count: 0,
                offer_id: None,
            });
            assert!(matches!(result, Err(Error::InvalidInput(_))));
        }

        #[test]
        fn rejects_count_one() {
            let result = InstallmentsIN::try_from(crate::InstallmentsIN::FixedPlan {
                count: 1,
                offer_id: None,
            });
            assert!(matches!(result, Err(Error::InvalidInput(_))));
        }

        #[test]
        fn rejects_empty_plan_id() {
            let result = InstallmentsIN::try_from(crate::InstallmentsIN::StoredPlan {
                id: "",
                offer_id: None,
            });
            assert!(matches!(result, Err(Error::InvalidInput(_))));
        }

        #[test]
        fn rejects_empty_offer_id() {
            let result = InstallmentsIN::try_from(crate::InstallmentsIN::FixedPlan {
                count: 6,
                offer_id: Some(""),
            });
            assert!(matches!(result, Err(Error::InvalidInput(_))));
        }
    }
}
