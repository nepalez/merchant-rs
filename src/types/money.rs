use iso_currency::Currency;
use rust_decimal::Decimal;

/// Monetary value with currency
///
/// # Data Protection
/// Money values are not considered sensitive data
/// because they represent transaction amounts which can be shared.
///
/// As such, they are:
/// * not masked in logs (via `Debug` implementation),
/// * exposed via safe public methods `amount()` and `currency()`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Money {
    amount: Decimal,
    currency: Currency,
}

impl Money {
    #[inline]
    pub fn new(amount: Decimal, currency: Currency) -> Self {
        Self { amount, currency }
    }

    #[inline]
    pub fn amount(&self) -> Decimal {
        self.amount
    }

    #[inline]
    pub fn currency(&self) -> Currency {
        self.currency
    }
}
