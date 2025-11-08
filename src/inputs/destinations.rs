use std::collections::HashMap;

use rust_decimal::Decimal;

/// Payment destinations input for split payments
///
/// Input structure for defining payment distribution before validation.
/// Accepts either a single platform amount or a HashMap of destinations
/// with unvalidated string keys.
///
/// ```skip
/// // Platform payment
/// let destinations = inputs::Destinations::Platform(dec!(100.00));
///
/// // Split payment
/// let mut splits = HashMap::new();
/// splits.insert("platform", dec!(5.00));
/// splits.insert("seller_123", dec!(95.00));
/// let destinations = inputs::Destinations::Split(splits);
/// ```
#[derive(Debug, Clone)]
pub enum Destinations {
    /// Platform receives the specified amount
    Platform(Decimal),

    /// Split payment between multiple destinations
    ///
    /// Keys are unvalidated recipient ID strings that will be validated
    /// during conversion to types::Destinations.
    Split(HashMap<&'static str, Decimal>),
}
