use rust_decimal::Decimal;
use std::collections::HashMap;
use std::convert::TryFrom;

use crate::Error;
use crate::internal::Validated;
use crate::types::{DistributedValue, RecipientId};

/// Payment recipients mapping
///
/// Maps recipient identifiers to their allocated portions of the payment amount.
/// Each recipient receives either a fixed amount or a percentage of the total.
///
/// Used in payments, subscriptions, and transactions to distribute funds
/// across multiple recipients (e.g., platform, merchants, partners).
///
/// # Structure
///
/// Internally stores a `HashMap<RecipientId, DistributedValue>` where:
/// * Key: Validated recipient identifier
/// * Value: Validated distributed value (amount or percentage)
///
/// # Validation
///
/// All recipients are validated during construction via `TryFrom`:
/// * Recipient IDs are sanitized and validated (1-255 characters)
/// * Distributed values are validated (amounts > 0, percentages in (0, 100))
/// * Amount totals: Can be checked against payment total
///
/// # Examples
///
/// ```skip
/// use merchant_rs::inputs;
/// use merchant_rs::types::Recipients;
/// use rust_decimal_macros::dec;
/// use std::collections::HashMap;
///
/// let mut input = HashMap::new();
/// input.insert("merchant_a", inputs::DistributedValue::Amount(dec!(50.00)));
/// input.insert("merchant_b", inputs::DistributedValue::Percent(dec!(10.0)));
///
/// let recipients = Recipients::try_from(input).unwrap();
///
/// // Calculate total allocated for a 200.00 payment
/// let total = recipients.calculate_total(dec!(200.00)).unwrap();
/// // Total: 50.00 + (200.00 * 10%) = 50.00 + 20.00 = 70.00
/// ```
#[derive(Debug, Clone, Default)]
pub struct Recipients(HashMap<RecipientId, DistributedValue>);

impl Recipients {
    /// Calculate the total amount allocated to recipients
    ///
    /// Given the total payment amount, calculates the sum of all recipient allocations.
    /// Percentage-based allocations are converted to amounts using the total.
    ///
    /// # Arguments
    ///
    /// * `total_amount` - The total payment amount to calculate percentages against
    ///
    /// # Returns
    ///
    /// Sum of all recipient allocations in the payment currency
    pub fn calculate_total(&self, total_amount: Decimal) -> Result<Decimal, Error> {
        self.0
            .values()
            .try_fold(Decimal::ZERO, |acc, part| match part {
                DistributedValue::Amount(amount) => Ok(acc + amount),
                DistributedValue::Percent(percent) => {
                    Ok(acc + (total_amount * percent / Decimal::from(100)))
                }
            })
    }

    /// Returns an iterator over the recipients
    ///
    /// Yields tuples of `(&RecipientId, &DistributedValue)`.
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = (&RecipientId, &DistributedValue)> {
        self.0.iter()
    }

    /// Returns the number of recipients
    #[inline]
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl<'a> TryFrom<crate::Recipients<'a>> for Recipients {
    type Error = Error;

    fn try_from(input: crate::Recipients<'a>) -> Result<Self, Self::Error> {
        let recipients = input
            .into_iter()
            .map(|(id, part)| {
                let recipient_id = RecipientId::try_from(id)?;
                let converted_part = DistributedValue::try_from(part)?;
                Ok((recipient_id, converted_part))
            })
            .collect::<Result<HashMap<_, _>, Error>>()?;

        Self(recipients).validate()
    }
}

// --- Sealed traits (not parts of the public API) ---

impl Validated for Recipients {
    fn validate(self) -> Result<Self, Error> {
        let validated_recipients = self
            .0
            .into_iter()
            .map(|(id, part)| {
                let validated_id = id.validate()?;
                let validated_part = part.validate()?;
                Ok((validated_id, validated_part))
            })
            .collect::<Result<HashMap<_, _>, Error>>()?;

        Ok(Self(validated_recipients))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn recipients_validates_positive_amount() {
        let mut input = HashMap::new();
        input.insert(
            "seller_1",
            crate::inputs::DistributedValue::Amount(dec!(100.00)),
        );

        let result = Recipients::try_from(input);
        assert!(result.is_ok());
    }

    #[test]
    fn recipients_rejects_negative_amount() {
        let mut input = HashMap::new();
        input.insert(
            "seller_1",
            crate::inputs::DistributedValue::Amount(dec!(-10.00)),
        );

        let result = Recipients::try_from(input);
        assert!(matches!(result, Err(Error::InvalidInput(_))));
    }

    #[test]
    fn recipients_validates_valid_percent() {
        let mut input = HashMap::new();
        input.insert(
            "seller_1",
            crate::inputs::DistributedValue::Percent(dec!(50.00)),
        );

        let result = Recipients::try_from(input);
        assert!(result.is_ok());
    }

    #[test]
    fn recipients_rejects_invalid_percent() {
        let mut input = HashMap::new();
        input.insert(
            "seller_1",
            crate::inputs::DistributedValue::Percent(dec!(150.00)),
        );

        let result = Recipients::try_from(input);
        assert!(matches!(result, Err(Error::InvalidInput(_))));
    }

    #[test]
    fn recipients_validates() {
        let mut input = HashMap::new();
        input.insert(
            "seller_1",
            crate::inputs::DistributedValue::Amount(dec!(50.00)),
        );
        input.insert(
            "seller_2",
            crate::inputs::DistributedValue::Amount(dec!(30.00)),
        );

        let result = Recipients::try_from(input);
        assert!(result.is_ok());
    }

    #[test]
    fn recipients_calculates_total_with_amounts() {
        let mut input = HashMap::new();
        input.insert(
            "seller_1",
            crate::inputs::DistributedValue::Amount(dec!(50.00)),
        );
        input.insert(
            "seller_2",
            crate::inputs::DistributedValue::Amount(dec!(30.00)),
        );

        let recipients = Recipients::try_from(input).unwrap();
        let total = recipients.calculate_total(dec!(100.00)).unwrap();
        assert_eq!(total, dec!(80.00));
    }

    #[test]
    fn recipients_calculates_total_with_percents() {
        let mut input = HashMap::new();
        input.insert(
            "seller_1",
            crate::inputs::DistributedValue::Percent(dec!(10.00)),
        );
        input.insert(
            "seller_2",
            crate::inputs::DistributedValue::Percent(dec!(5.00)),
        );

        let recipients = Recipients::try_from(input).unwrap();
        let total = recipients.calculate_total(dec!(100.00)).unwrap();
        assert_eq!(total, dec!(15.00));
    }

    #[test]
    fn recipients_calculates_total_with_mixed() {
        let mut input = HashMap::new();
        input.insert(
            "seller_1",
            crate::inputs::DistributedValue::Amount(dec!(50.00)),
        );
        input.insert(
            "seller_2",
            crate::inputs::DistributedValue::Percent(dec!(10.00)),
        );

        let recipients = Recipients::try_from(input).unwrap();
        let total = recipients.calculate_total(dec!(100.00)).unwrap();
        assert_eq!(total, dec!(60.00));
    }
}
