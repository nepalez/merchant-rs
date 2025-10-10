use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};
use crate::traits::{Authorizable, Gateway, RefundsCapability};
use crate::types::{Money, TransactionStatus};

/// TODO: to be guarded by feature flag 'standard-transactions'.
/// Trait for payment gateways that support the return of funds to a customer.
#[async_trait]
pub trait Refundable
where
    Self: Authorizable,
    Self: Gateway<RefundsCapability = RefundsSupported>,
{
    async fn refund(&self, request: RefundRequest) -> Result<RefundResponse>;
}

/// Indicates that the adapter does not support refunding transactions.
/// Applies to gateways that do not allow refunds (e.g., some crypto payment processors).
pub struct RefundsSupported;
impl RefundsCapability for RefundsSupported {}

/// TODO: Should be guarded by a feature flag (e.g., "standard-transactions").
/// Request body for initiating a refund.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundRequest {
    /// ID of the transaction to be refunded (usually a Capture ID).
    pub transaction_id: String,
    /// The exact amount to refund.
    pub amount_to_refund: Money,
    /// Optional reason for the refund, often required by the gateway.
    pub reason: Option<String>,
    /// Unique ID provided by the merchant for tracing the refund operation.
    pub merchant_reference_id: String,
}

/// TODO: Should be guarded by a feature flag (e.g., "standard-transactions").
/// Response body after a successful or failed refund.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundResponse {
    /// Indicates if the operation was successful.
    pub is_success: bool,
    /// The unique ID returned by the gateway for the refund record.
    pub refund_id: String,
    /// The canonical status (Should be Refunded or Failed).
    pub status: TransactionStatus,
    /// The final amount successfully refunded.
    pub refunded_amount: Money,
    /// Details of any error that occurred.
    pub error: Option<Error>,
}
