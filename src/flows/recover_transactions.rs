use async_trait::async_trait;

use crate::Error;
use crate::Gateway;
use crate::types::{Transaction, TransactionIdempotenceKey};

/// Payment gateway trait for disaster recovery via transaction search.
///
/// Supports searching for transactions by idempotence key when the transaction ID was lost
/// due to system failure, database rollback, or network interruption before persistence.
///
/// # When to Use
///
/// * **System crash**: Application crashed after transaction but before saving ID
/// * **Database rollback**: Transaction committed at gateway but database transaction rolled back
/// * **Network failure**: Response received but network dropped before processing
/// * **Duplicate detection**: Find previous transactions with same idempotence key
///
/// # Search Behavior
///
/// Returns async iterator because idempotence key may match multiple transactions:
/// * Network retries may create duplicate transactions
/// * Same idempotence key might be reused (though not recommended)
/// * Results may span multiple pages requiring pagination
///
/// Client must deduplicate results as gateway cannot guarantee uniqueness within a page.
///
/// # Gateway Support
///
/// Not all gateways support transaction search:
/// * **Support**: Stripe, Braintree, PayPal
/// * **No support**: Adyen, crypto processors, voucher systems
#[async_trait]
pub trait RecoverTransactions: Gateway {
    /// Async iterator type for paginated transaction results
    type Iterator: TransactionIterator;

    /// Search for transactions by idempotence key.
    ///
    /// Returns an async iterator for paginated results. Client must iterate through
    /// all results and deduplicate to find the desired transaction.
    ///
    /// # Parameters
    ///
    /// * `idempotence_key` - Key used to prevent duplicate transaction processing
    ///
    /// # Returns
    ///
    /// Async iterator yielding matching transactions
    async fn transactions(&self, idempotence_key: TransactionIdempotenceKey) -> Self::Iterator;
}

/// Async iterator for paginated transaction search results.
#[async_trait]
pub trait TransactionIterator: Sized {
    /// Fetch the next transaction from search results.
    ///
    /// Returns `None` when no more results are available.
    async fn next(&mut self) -> Option<Result<Transaction, Error>>;
}
