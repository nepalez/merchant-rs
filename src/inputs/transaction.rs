use iso_currency::Currency;

use crate::{Recipients, MerchantInitiatedType, TransactionStatus};

/// Information to build a transaction in Gateway adapters implementations.
pub struct Transaction<'a> {
    /// The unique transaction ID returned by the payment gateway.
    pub transaction_id: &'a str,
    /// The idempotency key.
    pub idempotence_key: &'a str,
    /// The canonical status of the transaction.
    pub status: TransactionStatus,
    /// The currency of the transaction.
    pub currency: Currency,
    /// The payment recipients.
    pub recipients: Option<Recipients<'a>>,
    /// The MIT (merchant initiated type of the transaction)
    pub merchant_initiated_type: Option<MerchantInitiatedType>,
}
