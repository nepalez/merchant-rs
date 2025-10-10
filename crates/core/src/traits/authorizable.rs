use async_trait::async_trait;

use crate::error::{Error, Result};
use crate::traits::Gateway;
use crate::types::{
    AccountNumber, AuthorizationCode, AuthorizationId, BankName, CustomerId, MerchantReferenceId,
    Money, PaymentSource, PaymentToken, RoutingNumber, TransactionId, TransactionStatus,
};

/// Core trait for initiating a payment transaction (Authorize or Sale) and subsequent void.
/// Every standard payment gateway adapter MUST implement this trait.
#[async_trait]
pub trait Authorizable: Gateway {
    /// Reserves funds (Auth) or immediately debits funds (Sale/Purchase).
    async fn authorize(&self, request: AuthorizationRequest) -> Result<AuthorizationResponse>;

    /// Cancels a pending authorization, releasing the reserved funds, or reverses a
    /// recently processed one-step transaction (Sale/Purchase) before settlement.
    ///
    /// The 'void' operation is mandatory here because it represents the immediate
    /// ability to retract the action initiated by 'authorize' before the funds
    /// are permanently settled by the payment network (which is actual
    /// for 1-step flows as well).
    async fn void(&self, request: VoidRequest) -> Result<VoidResponse>;
}

/// Request body for authorizing a payment.
#[derive(Debug, Clone)]
pub struct AuthorizationRequest {
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
pub struct AuthorizationResponse {
    /// Indicates if the transaction was successful (e.g., `Authorized` or `Captured`).
    pub is_success: bool,
    /// The unique transaction ID returned by the payment gateway.
    pub transaction_id: TransactionId,
    /// The canonical status of the transaction.
    pub status: TransactionStatus,
    /// Optional authorization code returned by the bank.
    pub authorization_code: Option<AuthorizationCode>,
    /// Details of any error that occurred, even if the status is Declined.
    pub error: Option<Error>,
}

/// Request body for voiding (canceling) a pending authorization.
#[derive(Debug, Clone)]
pub struct VoidRequest {
    /// ID of the authorization to void.
    pub authorization_id: AuthorizationId,
    /// Unique ID provided by the merchant for tracing the void operation.
    pub merchant_reference_id: MerchantReferenceId,
}

/// Response body after a successful or failed void operation.
#[derive(Debug, Clone)]
pub struct VoidResponse {
    /// Indicates if the operation was successful.
    pub is_success: bool,
    /// The transaction ID associated with the void operation.
    pub transaction_id: TransactionId,
    /// The canonical status (Should be Voided or Failed).
    pub status: TransactionStatus,
    /// Details of any error that occurred.
    pub error: Option<Error>,
}
