use async_trait::async_trait;

use crate::error::{Error, Result};
use crate::traits::Gateway;
use crate::types::{
    Money, TransactionStatus,
    secure::{AuthorizationCode, CustomerId, MerchantReferenceId, PaymentSource, TransactionId},
};

/// The trait for initiating a payment transaction (Authorize or Sale).
///
/// Any gateway is expected to implement this trait.
#[async_trait]
pub trait Authorizable: Gateway {
    /// Reserve funds (Auth) or immediately debits funds (Sale/Purchase).
    async fn authorize(&self, request: Request) -> Result<Response>;
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

#[derive(Debug, Clone)]
pub struct StatusRequest {
    pub transaction_id: TransactionId,
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
