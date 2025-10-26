use async_trait::async_trait;

use crate::error::Result;
use crate::types::{
    Money, TransactionStatus,
    secure::{MerchantReferenceId, TransactionId},
};

/// Optional trait for getting a paginated list of transactions by an idempotency key
/// with some additional filters.
/// The results should be deduplicated on the client's side
/// (they cannot be deduplicated within the particular page only).
#[async_trait]
pub trait Recoverable {
    /// Confirms and debits the previously authorized funds.
    async fn recover(&self, request: Request) -> Result<Page<Transaction>>;
}

/// Request body for capturing a previously authorized payment.
#[derive(Debug, Clone)]
pub struct Request {
    /// Unique ID provided by the merchant for tracing the transaction.
    pub merchant_reference_id: MerchantReferenceId,
    // TODO: add filters
    pub page: Option<u32>,
}

/// Response body after a successful or failed capture.
#[derive(Debug, Clone)]
pub struct Transaction {
    /// The new transaction ID for the capture operation.
    pub transaction_id: TransactionId,
    /// The canonical status (Should be Captured or Failed).
    pub status: TransactionStatus,
    /// The final amount successfully captured.
    pub amount: Money,
}

pub struct Page<T> {
    pub items: Vec<T>,
    pub next_page: Option<u32>,
}
