use rust_decimal::Decimal;
use std::convert::TryFrom;

use crate::Error;
use crate::internal::Validated;
use crate::types::Recipients;

/// Payment redistribution for capture/refund operations.
///
/// Allows changing the payment amount and/or recipients during capture or refund.
/// Both fields are optional, supporting various redistribution scenarios:
///
/// # Redistribution Scenarios
///
/// * `{ total: None, recipients: None }` - No change, use original distribution
/// * `{ total: Some(amount), recipients: None }` - Change amount only (partial capture/refund)
/// * `{ total: None, recipients: Some(r) }` - Change recipients only (same total, different split)
/// * `{ total: Some(amount), recipients: Some(r) }` - Change both amount and recipients
///
/// The portion of `total` not allocated to recipients goes to the platform.
///
/// # Validation
///
/// * If total is specified, it must be positive (> 0)
/// * If both total and recipients are specified, recipients total must not exceed payment total
/// * Individual recipients are validated according to their rules
///
/// # Examples
///
/// ```skip
/// use merchant_rs::types::RedistributedAmount;
/// use rust_decimal_macros::dec;
///
/// // Partial refund - 50.00 from original 100.00
/// let partial = RedistributedAmount::from(dec!(50.00));
///
/// // Change recipients only
/// let recipients_change = RedistributedAmount::from(recipients);
///
/// // No change
/// let no_change = RedistributedAmount::from(());
/// ```
#[derive(Debug, Clone, Default)]
pub struct RedistributedAmount {
    total: Option<Decimal>,
    recipients: Option<Recipients>,
}

impl RedistributedAmount {
    /// Returns the new total amount if specified
    #[inline]
    pub fn total(&self) -> Option<Decimal> {
        self.total
    }

    /// Returns a reference to the new recipients if specified
    #[inline]
    pub fn recipients(&self) -> Option<&Recipients> {
        self.recipients.as_ref()
    }
}

impl From<Option<Decimal>> for RedistributedAmount {
    fn from(total: Option<Decimal>) -> Self {
        Self {
            total,
            recipients: None,
        }
    }
}

impl From<Decimal> for RedistributedAmount {
    fn from(total: Decimal) -> Self {
        Some(total).into()
    }
}

impl From<Option<Recipients>> for RedistributedAmount {
    fn from(recipients: Option<Recipients>) -> Self {
        Self {
            total: None,
            recipients,
        }
    }
}

impl From<Recipients> for RedistributedAmount {
    fn from(recipients: Recipients) -> Self {
        Some(recipients).into()
    }
}

impl From<()> for RedistributedAmount {
    fn from(_: ()) -> Self {
        Self::default()
    }
}

impl<'a> TryFrom<crate::RedistributedAmount<'a>> for RedistributedAmount {
    type Error = Error;

    fn try_from(input: crate::RedistributedAmount<'a>) -> Result<Self, Self::Error> {
        Self {
            total: input.total,
            recipients: input.recipients.map(TryFrom::try_from).transpose()?,
        }
        .validate()
    }
}

impl Validated for RedistributedAmount {
    fn validate(self) -> Result<Self, Error> {
        if let Some(total) = self.total {
            if total <= Decimal::ZERO {
                return Err(Error::InvalidInput(
                    "Total amount must be positive".to_string(),
                ));
            }

            if let Some(ref recipients) = self.recipients {
                let recipients_total = recipients.calculate_total(total)?;
                if recipients_total > total {
                    return Err(Error::InvalidInput(format!(
                        "Recipients total ({}) exceeds payment total ({})",
                        recipients_total, total
                    )));
                }
            }
        }

        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;
    use std::collections::HashMap;

    #[test]
    fn default_is_empty() {
        let amount = RedistributedAmount::default();
        assert!(amount.total().is_none());
        assert!(amount.recipients().is_none());
    }

    #[test]
    fn converts_from_unit() {
        let amount = RedistributedAmount::from(());
        assert!(amount.total().is_none());
        assert!(amount.recipients().is_none());
    }

    #[test]
    fn converts_from_decimal() {
        let amount = RedistributedAmount::from(dec!(50.00));
        assert_eq!(amount.total(), Some(dec!(50.00)));
        assert!(amount.recipients().is_none());
    }

    #[test]
    fn converts_from_option_decimal() {
        let amount = RedistributedAmount::from(Some(dec!(50.00)));
        assert_eq!(amount.total(), Some(dec!(50.00)));
        assert!(amount.recipients().is_none());

        let amount = RedistributedAmount::from(None::<Decimal>);
        assert!(amount.total().is_none());
        assert!(amount.recipients().is_none());
    }

    #[test]
    fn converts_from_recipients() {
        let mut recipients_input = HashMap::new();
        recipients_input.insert(
            "seller_1",
            crate::inputs::DistributedValue::Amount(dec!(30.00)),
        );
        let recipients = crate::types::Recipients::try_from(recipients_input).unwrap();

        let amount = RedistributedAmount::from(recipients);
        assert!(amount.total().is_none());
        assert!(amount.recipients().is_some());
    }

    #[test]
    fn converts_from_option_recipients() {
        let mut recipients_input = HashMap::new();
        recipients_input.insert(
            "seller_1",
            crate::inputs::DistributedValue::Amount(dec!(30.00)),
        );
        let recipients = crate::types::Recipients::try_from(recipients_input).unwrap();

        let amount = RedistributedAmount::from(Some(recipients));
        assert!(amount.total().is_none());
        assert!(amount.recipients().is_some());

        let amount = RedistributedAmount::from(None::<crate::types::Recipients>);
        assert!(amount.total().is_none());
        assert!(amount.recipients().is_none());
    }

    #[test]
    fn validates_positive_total() {
        let input = crate::inputs::RedistributedAmount {
            total: Some(dec!(50.00)),
            recipients: None,
        };

        let result = RedistributedAmount::try_from(input);
        assert!(result.is_ok());
    }

    #[test]
    fn rejects_zero_total() {
        let input = crate::inputs::RedistributedAmount {
            total: Some(dec!(0.00)),
            recipients: None,
        };

        let result = RedistributedAmount::try_from(input);
        assert!(matches!(result, Err(Error::InvalidInput(_))));
    }

    #[test]
    fn rejects_negative_total() {
        let input = crate::inputs::RedistributedAmount {
            total: Some(dec!(-10.00)),
            recipients: None,
        };

        let result = RedistributedAmount::try_from(input);
        assert!(matches!(result, Err(Error::InvalidInput(_))));
    }

    #[test]
    fn accepts_none_total() {
        let input = crate::inputs::RedistributedAmount {
            total: None,
            recipients: None,
        };

        let result = RedistributedAmount::try_from(input);
        assert!(result.is_ok());
    }

    #[test]
    fn accepts_valid_total_and_recipients() {
        let mut recipients = HashMap::new();
        recipients.insert(
            "seller_1",
            crate::inputs::DistributedValue::Amount(dec!(30.00)),
        );

        let input = crate::inputs::RedistributedAmount {
            total: Some(dec!(100.00)),
            recipients: Some(recipients),
        };

        let result = RedistributedAmount::try_from(input);
        assert!(result.is_ok());
    }

    #[test]
    fn rejects_recipients_exceeding_total() {
        let mut recipients = HashMap::new();
        recipients.insert(
            "seller_1",
            crate::inputs::DistributedValue::Amount(dec!(150.00)),
        );

        let input = crate::inputs::RedistributedAmount {
            total: Some(dec!(100.00)),
            recipients: Some(recipients),
        };

        let result = RedistributedAmount::try_from(input);
        assert!(matches!(result, Err(Error::InvalidInput(_))));
    }

    #[test]
    fn accepts_recipients_without_total() {
        let mut recipients = HashMap::new();
        recipients.insert(
            "seller_1",
            crate::inputs::DistributedValue::Amount(dec!(30.00)),
        );

        let input = crate::inputs::RedistributedAmount {
            total: None,
            recipients: Some(recipients),
        };

        let result = RedistributedAmount::try_from(input);
        assert!(result.is_ok());
    }
}
