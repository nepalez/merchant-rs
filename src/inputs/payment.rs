use iso_currency::Currency;
use rust_decimal::Decimal;

/// Payment information.
pub struct Payment<'a, M> {
    /// The payment method.
    pub payment_method: M,
    /// The currency of the payment.
    pub currency: Currency,
    /// The total payment amount.
    pub total_amount: Decimal,
    /// The amount going to the platform.
    pub base_amount: Decimal,
    /// The idempotency key.
    pub idempotence_key: &'a str,
}
