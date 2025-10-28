use crate::types::{MerchantInitiatedType, Money, TransactionStatus};

/// Information to build a transaction in Gateway adapters implementations.
pub struct Transaction<'a> {
    /// The unique transaction ID returned by the payment gateway.
    pub transaction_id: &'a str,
    /// The idempotency key.
    pub idempotence_key: &'a str,
    /// The canonical status of the transaction.
    pub status: TransactionStatus,
    /// The amount of the transaction.
    pub amount: Money,
    /// The MIT (merchant initiated type of the transaction)
    pub merchant_initiated_type: Option<MerchantInitiatedType>,
}
