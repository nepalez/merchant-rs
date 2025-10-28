use async_trait::async_trait;

use crate::Error;
use crate::types::{Transaction, TransactionId};

/// The base trait defining a payment gateway adapter's core identity and capabilities.
/// This trait is the minimal requirement for any adapter.
///
/// Every trait must support only one operation,
/// namely provide the status of transactions by their IDs (primary key).
/// All the other capabilities are optional and can vary from one adapter to another.
#[allow(private_bounds)]
#[async_trait]
pub trait CheckTransaction {
    /// Get the status of a transaction.
    async fn status(&self, transaction_id: TransactionId) -> Result<Transaction, Error>;
}
