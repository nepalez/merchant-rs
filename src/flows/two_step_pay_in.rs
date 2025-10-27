use async_trait::async_trait;

use crate::error::Error;
use crate::types::{
    Money,
    enums::TransactionStatus,
    secure::{MerchantReferenceId, NewPayment, TransactionId},
};

/// Optional trait for payment gateways that support completing a two-step flow,
/// where the first step is an authorization and the second is a capture.
#[async_trait]
pub trait TwoStepPayIn {
    async fn authorize(&self, payment: NewPayment) -> Result<Response, Error>;

    /// Confirms and debits the previously authorized funds.
    async fn capture(&self, request: CaptureRequest) -> Result<Response, Error>;
}

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
pub struct Response {
    /// The new transaction ID for the capture operation.
    pub transaction_id: TransactionId,
    /// The canonical status (Should be Captured or Failed).
    pub status: TransactionStatus,
    /// The final amount successfully captured.
    pub captured_amount: Money,
    /// Details of any error that occurred.
    pub error: Option<Error>,
}
