use async_trait::async_trait;

use crate::error::Error;
use crate::flows::TwoStepPayments;

/// Optional trait for payment gateways that support adjusting a payment
/// after authorization (either incrementing or decrementing its amount)
/// of the two-step flow.
#[async_trait]
pub trait AdjustPayments: TwoStepPayments {
    async fn adjust_payment(&self, request: Request) -> Result<Response, Error>;
}

#[derive(Debug, Clone)]
pub struct Request {
    pub transaction_id: String,
    pub amount_to_adjust: i64,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub transaction_id: String,
    pub status: String,
    pub error: Option<Error>,
}
