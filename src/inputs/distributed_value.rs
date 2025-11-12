use rust_decimal::Decimal;

/// Insecure representation of a value distributed to a recipient.
///
/// ```skip
/// let fixed = DistributedValue::Amount(dec!(50.00));
/// let percent = DistributedValue::Percent(dec!(10.0));
/// ```
#[derive(Debug, Clone, Copy)]
pub enum DistributedValue {
    /// Fixed amount in payment currency.
    Amount(Decimal),

    /// Percentage of the total payment amount.
    Percent(Decimal),
}
