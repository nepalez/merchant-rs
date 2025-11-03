use iso_currency::Currency;
use rust_decimal::Decimal;

/// Monetary value with currency
///
/// # Data Protection
/// Money values are not considered sensitive data
/// because they represent transaction amounts which can be shared.
///
/// As such, they are not masked in logs (via `Debug` implementation).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Money {
    pub amount: Decimal,
    pub currency: Currency,
}
