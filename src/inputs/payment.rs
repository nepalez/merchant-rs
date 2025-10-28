use crate::inputs::PaymentSource;
use crate::types::{MerchantInitiatedType, Money};

/// Insecure structure representing a payment.
pub struct Payment<'a> {
    /// The source of the payment to charge funds from
    pub source: PaymentSource<'a>,
    /// The amount to charge
    pub amount: Money,
    /// The idempotency key
    pub idempotence_key: &'a str,
    /// The scope of the payment initiated by the merchant
    /// (use `None` if the payment was initiated by a customer).
    pub merchant_initiated_type: Option<MerchantInitiatedType>,
}
