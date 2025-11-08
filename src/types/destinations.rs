use rust_decimal::Decimal;
use std::collections::HashMap;
use std::convert::TryFrom;

use crate::Error;
use crate::inputs;
use crate::types::RecipientId;

/// Payment destinations for split payments
///
/// Defines how payment funds should be distributed - either to a single platform account
/// or split between multiple destinations in marketplace scenarios.
///
/// All amounts are in the payment currency (currency is specified separately in Payment/Transaction).
/// The sum of split amounts must equal the total payment amount.
///
/// # Split Ordering
/// Split payments use HashMap for recipient uniqueness. Payment systems do not guarantee
/// processing order for splits, so ordering is not preserved.
#[derive(Debug, Clone)]
pub enum Destinations {
    /// Platform receives the specified amount
    ///
    /// Used for standard payments where all funds go to the platform's account.
    /// The amount is in the payment's currency.
    Platform(Decimal),

    /// Split payment between multiple destinations
    ///
    /// Used for marketplace payments where funds are distributed between
    /// platform, sellers, service providers, etc.
    ///
    /// # Constraints
    /// * Each recipient (RecipientId) must be unique (enforced by HashMap)
    /// * Sum of all destination amounts must equal the total payment amount
    /// * All amounts are in the payment's currency
    /// * At least one destination is required
    Split(HashMap<RecipientId, Decimal>),
}

impl Destinations {
    /// Calculate total amount from destinations
    ///
    /// Returns the total payment amount regardless of whether it's a platform
    /// payment or split payment.
    pub fn total_amount(&self) -> Decimal {
        match self {
            Destinations::Platform(amount) => *amount,
            Destinations::Split(destinations) => destinations.values().copied().sum(),
        }
    }
}

impl TryFrom<inputs::Destinations> for Destinations {
    type Error = Error;

    fn try_from(input: inputs::Destinations) -> Result<Self, Self::Error> {
        match input {
            inputs::Destinations::Platform(amount) => Ok(Self::Platform(amount)),
            inputs::Destinations::Split(destinations) => {
                let map = destinations
                    .into_iter()
                    .map(|(k, v)| RecipientId::try_from(k).map(|id| (id, v)))
                    .collect::<Result<HashMap<_, _>, _>>()?;
                Ok(Self::Split(map))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn platform_total_amount() {
        let destinations = Destinations::Platform(dec!(100.00));
        assert_eq!(destinations.total_amount(), dec!(100.00));
    }

    #[test]
    fn split_total_amount() {
        let mut splits = HashMap::new();
        splits.insert(RecipientId::try_from("platform").unwrap(), dec!(5.00));
        splits.insert(RecipientId::try_from("seller_123").unwrap(), dec!(95.00));

        let destinations = Destinations::Split(splits);

        assert_eq!(destinations.total_amount(), dec!(100.00));
    }

    #[test]
    fn empty_split() {
        let destinations = Destinations::Split(HashMap::new());
        assert_eq!(destinations.total_amount(), dec!(0));
    }
}
