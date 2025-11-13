use rust_decimal::Decimal;
use std::convert::TryFrom;

use crate::Error;
use crate::inputs::DistributedAmount as Input;
use crate::internal::Validated;
use crate::types::Recipients;

/// Payment amount with optional distribution to recipients.
///
/// Represents a payment amount that can be split among multiple recipients.
/// The total amount is always required, while recipients list can be empty
/// (indicating all funds go to the platform).
///
/// # Distribution Model
///
/// The portion of `total` not allocated to recipients goes to the platform.
/// * Empty recipients: entire amount goes to the platform
/// * Partial allocation: platform receives `total - sum(recipients)`
/// * Full allocation: platform receives nothing (all distributed to recipients)
///
/// # Validation
///
/// * Total amount must be positive (> 0)
/// * Sum of recipient allocations must not exceed total amount
/// * Individual recipients are validated according to their rules
///
/// # Examples
///
/// ```skip
/// use merchant_rs::types::DistributedAmount;
/// use rust_decimal_macros::dec;
///
/// // Simple payment - entire 100.00 goes to platform
/// let amount = DistributedAmount::from(dec!(100.00));
///
/// // Payment with recipients - platform receives 70.00, seller receives 30.00
/// let mut recipients = HashMap::new();
/// recipients.insert("seller_1", DistributedValue::Amount(dec!(30.00)));
/// let input = inputs::DistributedAmount {
///     total: dec!(100.00),
///     recipients: Some(recipients),
/// };
/// let amount = DistributedAmount::try_from(input)?;
/// ```
#[derive(Debug, Clone)]
pub struct DistributedAmount {
    total: Decimal,
    recipients: Recipients,
}

impl DistributedAmount {
    /// Returns the total payment amount
    #[inline]
    pub fn total(&self) -> Decimal {
        self.total
    }

    /// Returns a reference to the recipients
    #[inline]
    pub fn recipients(&self) -> &Recipients {
        &self.recipients
    }
}

impl From<Decimal> for DistributedAmount {
    fn from(total: Decimal) -> Self {
        Self {
            total,
            recipients: Recipients::default(),
        }
    }
}

impl<'a> TryFrom<Input<'a>> for DistributedAmount {
    type Error = Error;

    fn try_from(input: Input<'a>) -> Result<Self, Self::Error> {
        Self {
            total: input.total,
            recipients: Recipients::try_from(input.recipients)?,
        }
        .validate()
    }
}

impl Validated for DistributedAmount {
    fn validate(self) -> Result<Self, Error> {
        if self.total <= Decimal::ZERO {
            return Err(Error::InvalidInput(
                "Total amount must be positive".to_string(),
            ));
        }

        let recipients_total = self.recipients.calculate_total(self.total)?;
        if recipients_total > self.total {
            return Err(Error::InvalidInput(format!(
                "Recipients total ({}) exceeds payment total ({})",
                recipients_total, self.total
            )));
        }

        Ok(self)
    }
}
