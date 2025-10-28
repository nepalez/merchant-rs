use async_trait::async_trait;

use crate::Error;
use crate::types::{Transaction, TransactionIdempotenceKey};

/// Optional trait for getting a paginated list of transactions by an idempotency key
/// with some additional filters.
/// The results should be deduplicated on the client's side
/// (they cannot be deduplicated within the particular page only).
pub trait RecoverTransactions {
    type Iterator: TransactionIterator;

    /// Provides the async iterator by transactions having a provided idempotence key
    fn transactions(&self, idempotence_key: TransactionIdempotenceKey) -> Self::Iterator;
}

#[async_trait]
pub trait TransactionIterator: Sized {
    async fn next(&mut self) -> Option<Result<Transaction, Error>>;
}
