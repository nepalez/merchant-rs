use async_trait::async_trait;

use crate::Error;
use crate::types::{Transaction, TransactionId};

/// The trait to support voiding (cancelling) a pending authorization.
///
/// Adapters should implement this trait only when they support cancellation
/// of previously authorized payments.
#[async_trait]
pub trait CancelPayments {
    /// Cancel a pending authorization, releasing the reserved funds,
    /// or reverse a recently processed one-step transaction (Sale/Purchase) before the settlement.
    ///
    /// The 'void' operation is mandatory here because it represents the immediate
    /// ability to retract the action initiated by 'authorize' before the funds
    /// are permanently settled by the payment network
    /// (which is actual for 1-step flows as well).
    async fn void(&self, transaction_id: TransactionId) -> Result<Transaction, Error>;
}
