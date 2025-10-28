use async_trait::async_trait;

use crate::Error;
use crate::types::{
    AuthorizationCode, CustomerId, MerchantReferenceId, Money, Payment, PaymentSource,
    TransactionId, TransactionStatus,
};

/// Optional trait for payment gateways that support completing a one-step flow,
/// without the necessity to capture them later.
#[async_trait]
pub trait OneStepPayments {
    /// Immediately charge the payment.
    async fn charge(&self, payment: Payment) -> Result<Response, Error>;
}

/// Request body for authorizing a payment.
#[derive(Debug, Clone)]
pub struct Request {
    /// The source of the payment (must be a token or bank account details).
    pub source: PaymentSource,
    /// The monetary amount to be authorized.
    pub amount: Money,
    /// Unique ID provided by the merchant for tracing the transaction.
    pub merchant_reference_id: Option<MerchantReferenceId>,
    /// Optional identifier for the customer.
    pub customer_id: Option<CustomerId>,
}

/// Response body after an authorization attempt.
#[derive(Debug, Clone)]
pub struct Response {
    /// The unique transaction ID returned by the payment gateway.
    pub transaction_id: TransactionId,
    /// The content of the transaction
    pub content: Request,
    /// The canonical status of the transaction.
    pub status: TransactionStatus,
    /// Optional authorization code returned by the bank.
    pub authorization_code: Option<AuthorizationCode>,
    /// Details of any error that occurred, even if the status is Declined.
    pub error: Option<Error>,
}
