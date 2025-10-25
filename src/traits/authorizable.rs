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
    /// Reserves funds (Auth) or immediately debits funds (Sale/Purchase).
    async fn authorize(&self, request: Request) -> Result<Response>;
}

/// Request body for authorizing a payment.
#[derive(Debug, Clone)]
pub struct Request {
    /// The monetary amount to be authorized.
    pub amount: Money,
    /// The source of the payment (must be a token or bank account details).
    pub source: PaymentSource,
    /// Unique ID provided by the merchant for tracing the transaction.
    pub merchant_reference_id: MerchantReferenceId,
    /// Optional identifier for the customer.
    pub customer_id: CustomerId,
    /// Opaque byte array representing arbitrary metadata. The format (e.g., JSON, XML)
    /// is interpreted by the Adapter, making the core format-agnostic.
    pub metadata: Option<Vec<u8>>,
}

/// Response body after an authorization attempt.
#[derive(Debug, Clone)]
pub struct Response {
    /// The unique transaction ID returned by the payment gateway.
    pub transaction_id: TransactionId,
    /// The canonical status of the transaction.
    pub status: TransactionStatus,
    /// Optional authorization code returned by the bank.
    pub authorization_code: Option<AuthorizationCode>,
    /// Details of any error that occurred, even if the status is Declined.
    pub error: Option<Error>,
}
