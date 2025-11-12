use rust_decimal::Decimal;
use std::convert::TryFrom;

use crate::Error;
use crate::inputs::DistributedValue as Input;
use crate::internal::Validated;

/// Value distributed to a recipient in split payments
///
/// Represents how much of the total payment amount should be allocated
/// to a specific recipient. Can be specified as either a fixed amount
/// or a percentage of the total.
///
/// # Variants
///
/// * `Amount(Decimal)`: Fixed amount in the payment currency
///   - Example: `Amount(50.00)` = exactly 50.00 in currency units
///   - Must be positive (> 0)
///
/// * `Percent(Decimal)`: Percentage of the total payment amount
///   - Example: `Percent(10.0)` = 10% of total amount
///   - Must be in range (0, 100), exclusive on both ends
///   - Calculated dynamically based on total amount
///
/// # Validation
///
/// * Amount: must be positive (> 0)
/// * Percent: must be between 0 and 100, exclusive (0 < percent < 100)
///
/// # Examples
///
/// ```skip
/// use merchant_rs::inputs::DistributedValue;
/// use rust_decimal_macros::dec;
///
/// // Fixed amount: recipient gets exactly 50.00
/// let fixed = DistributedValue::Amount(dec!(50.00));
///
/// // Percentage: recipient gets 10% of total
/// let percent = DistributedValue::Percent(dec!(10.0));
/// ```
#[derive(Debug, Clone, Copy)]
pub enum DistributedValue {
    /// Fixed amount in payment currency (guaranteed positive)
    Amount(Decimal),

    /// Percentage of total payment amount (guaranteed 0 < percent < 100)
    Percent(Decimal),
}

impl TryFrom<Input> for DistributedValue {
    type Error = Error;

    fn try_from(input: Input) -> Result<Self, Self::Error> {
        let value = match input {
            Input::Amount(amount) => DistributedValue::Amount(amount),
            Input::Percent(percent) => DistributedValue::Percent(percent),
        };
        value.validate()
    }
}

// --- Sealed traits (not parts of the public API) ---

impl Validated for DistributedValue {
    fn validate(self) -> Result<Self, Error> {
        match self {
            DistributedValue::Amount(amount) => {
                if amount <= Decimal::ZERO {
                    return Err(Error::InvalidInput(format!(
                        "Recipient amount must be positive, got {amount}"
                    )));
                }
            }
            DistributedValue::Percent(percent) => {
                if percent <= Decimal::ZERO || percent >= Decimal::from(100) {
                    return Err(Error::InvalidInput(format!(
                        "Recipient percent must be between 0 and 100 (exclusive), got {percent}"
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

    mod construction {
        use super::*;

        #[test]
        fn accepts_valid_amounts() {
            for amount in [dec!(0.01), dec!(1.00), dec!(100.00), dec!(9999.99)] {
                let input = Input::Amount(amount);
                let result = DistributedValue::try_from(input);
                assert!(result.is_ok(), "Amount {amount} failed validation");
            }
        }

        #[test]
        fn accepts_valid_percents() {
            for percent in [dec!(0.01), dec!(1.00), dec!(50.00), dec!(99.99)] {
                let input = Input::Percent(percent);
                let result = DistributedValue::try_from(input);
                assert!(result.is_ok(), "Percent {percent} failed validation");
            }
        }

        #[test]
        fn rejects_zero_amount() {
            let input = Input::Amount(dec!(0.00));
            let result = DistributedValue::try_from(input);
            assert!(matches!(result, Err(Error::InvalidInput(_))));
        }

        #[test]
        fn rejects_negative_amount() {
            let input = Input::Amount(dec!(-10.00));
            let result = DistributedValue::try_from(input);
            assert!(matches!(result, Err(Error::InvalidInput(_))));
        }

        #[test]
        fn rejects_zero_percent() {
            let input = Input::Percent(dec!(0.00));
            let result = DistributedValue::try_from(input);
            assert!(matches!(result, Err(Error::InvalidInput(_))));
        }

        #[test]
        fn rejects_negative_percent() {
            let input = Input::Percent(dec!(-5.00));
            let result = DistributedValue::try_from(input);
            assert!(matches!(result, Err(Error::InvalidInput(_))));
        }

        #[test]
        fn rejects_hundred_percent() {
            let input = Input::Percent(dec!(100.00));
            let result = DistributedValue::try_from(input);
            assert!(matches!(result, Err(Error::InvalidInput(_))));
        }

        #[test]
        fn rejects_over_hundred_percent() {
            let input = Input::Percent(dec!(150.00));
            let result = DistributedValue::try_from(input);
            assert!(matches!(result, Err(Error::InvalidInput(_))));
        }
    }
}
