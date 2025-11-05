use crate::types::{MerchantInitiatedType, Money, StoredCredentialUsage};

/// Insecure structure representing a payment.
pub struct Payment<'a, Method: 'a> {
    /// The method of the payment to charge funds from
    pub method: Method,
    /// The amount to charge
    pub amount: Money,
    /// The idempotency key
    pub idempotence_key: &'a str,
    /// The scope of the payment initiated by the merchant
    /// (use `None` if the payment was initiated by a customer).
    pub merchant_initiated_type: Option<MerchantInitiatedType>,
    /// Indicates whether this is the first or subsequent use of stored credentials.
    /// Use `None` for one-time payments where credentials are not stored.
    /// Required for Credential-on-File (COF) compliance with card networks.
    pub stored_credential_usage: Option<StoredCredentialUsage>,
}
