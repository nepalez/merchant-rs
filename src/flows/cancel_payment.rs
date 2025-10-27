use async_trait::async_trait;

use crate::error::Error;
use crate::types::{
    TransactionStatus,
    secure::{MerchantReferenceId, TransactionId},
};

/// The trait to support voiding (cancelling) a pending authorization.
///
/// Adapters should implement this trait only when they support cancellation
/// of previously authorized payments.
#[async_trait]
pub trait CancelPayment {
    /// Cancels a pending authorization, releasing the reserved funds, or reverses a
    /// recently processed one-step transaction (Sale/Purchase) before settlement.
    ///
    /// The 'void' operation is mandatory here because it represents the immediate
    /// ability to retract the action initiated by 'authorize' before the funds
    /// are permanently settled by the payment network (which is actual
    /// for 1-step flows as well).
    async fn void(&self, request: Request) -> crate::Result<Response>;
}

/// Request body for voiding (canceling) a pending authorization.
#[derive(Debug, Clone)]
pub struct Request {
    /// ID of the original transaction to void.
    pub transaction_id: TransactionId,
    /// Unique ID provided by the merchant for tracing the void operation.
    pub merchant_reference_id: MerchantReferenceId,
}

/// Response body after a successful or failed void operation.
#[derive(Debug, Clone)]
pub struct Response {
    /// The transaction ID associated with the void operation.
    pub transaction_id: TransactionId,
    /// The canonical status (Should be Voided or Failed).
    pub status: TransactionStatus,
    /// Details of any error that occurred.
    pub error: Option<Error>,
}
