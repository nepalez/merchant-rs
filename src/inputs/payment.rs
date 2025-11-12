use iso_currency::Currency;
use rust_decimal::Decimal;

use crate::inputs::Recipients;
use crate::types::{MerchantInitiatedType, StoredCredentialUsage};

/// Insecure structure representing a payment.
pub struct Payment<'a, Method: 'a> {
    /// The method of the payment to charge funds from
    pub method: Method,
    /// The currency for this payment
    pub currency: Currency,
    /// Total payment amount
    pub total_amount: Decimal,
    /// Optional payment recipients (None = the platform receives all)
    pub recipients: Option<Recipients<'a>>,
    /// The idempotency key
    pub idempotence_key: &'a str,
    /// The scope of the payment initiated by the merchant
    /// (use `None` if the payment was initiated by a customer).
    pub merchant_initiated_type: Option<MerchantInitiatedType>,
    /// Indicates whether this is the first or later use of stored credentials.
    /// Use `None` for one-time payments where credentials are not stored.
    /// Required for Credential-on-File (COF) compliance with card networks.
    pub stored_credential_usage: Option<StoredCredentialUsage>,
}
