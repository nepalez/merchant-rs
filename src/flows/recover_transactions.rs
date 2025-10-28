use async_trait::async_trait;

use crate::Error;
use crate::inputs::Transaction;
use crate::types::MerchantReferenceId;

/// Optional trait for getting a paginated list of transactions by an idempotency key
/// with some additional filters.
/// The results should be deduplicated on the client's side
/// (they cannot be deduplicated within the particular page only).
#[async_trait]
pub trait RecoverTransactions {
    /// Confirms and debits the previously authorized funds.
    async fn recover(
        &self,
        merchant_reference_id: MerchantReferenceId,
    ) -> Result<Page<Transaction>, Error>;

    // TODO: implement async iterator
}

// TODO: move to types

pub struct Page<T: Sized> {
    items: Vec<T>,
    has_next: bool,
}

impl<T> Page<T> {
    pub fn items(&self) -> &[T] {
        &self.items
    }

    pub fn has_next(&self) -> bool {
        self.has_next
    }
}
