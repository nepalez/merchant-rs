use async_trait::async_trait;

use crate::error::{Error, Result};
use crate::traits::{Authorizable, Gateway, TransactionFlow};
use crate::types::*;

/// Optional trait for payment gateways that support completing a two-step flow.
///
/// An adapter must implement this ONLY if it supports the two-step Auth -> Capture model.
/// Gateways that only support Sale/Purchase should NOT implement this trait.
#[async_trait]
pub trait Capturable
where
    Self: Authorizable,
    Self: Gateway<TransactionFlow = TwoStepFlow>,
{
    /// Confirms and debits the previously authorized funds.
    async fn capture(&self, request: CaptureRequest) -> Result<CaptureResponse>;
}

/// Transaction Style: Two-step flow (Authorize + subsequent Capture).
/// Applies to gateways that support delayed capture and full lifecycle management (e.g., Stripe).
pub struct TwoStepFlow;
impl TransactionFlow for TwoStepFlow {}

/// Request body for capturing a previously authorized payment.
#[derive(Debug, Clone)]
pub struct CaptureRequest {
    /// ID of the original authorization transaction returned by the gateway.
    pub transaction_id: TransactionId,
    /// The exact amount to capture. Must be less than or equal to the authorized amount.
    pub amount_to_capture: Money,
    /// Unique ID provided by the merchant for tracing the capture operation.
    pub merchant_reference_id: MerchantReferenceId,
}

/// Response body after a successful or failed capture.
#[derive(Debug, Clone)]
pub struct CaptureResponse {
    /// The new transaction ID for the capture operation.
    pub transaction_id: TransactionId,
    /// The canonical status (Should be Captured or Failed).
    pub status: TransactionStatus,
    /// The final amount successfully captured.
    pub captured_amount: Money,
    /// Details of any error that occurred.
    pub error: Option<Error>,
}
