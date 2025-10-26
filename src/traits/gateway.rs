use crate::error::Error;
use crate::types::{
    TransactionStatus,
    secure::{AuthorizationCode, TransactionId},
};
use async_trait::async_trait;

/// The base trait defining a payment gateway adapter's core identity and capabilities.
/// This trait is the minimal requirement for any adapter.
///
/// Every trait must support only one operation,
/// namely provide the status of transactions by their IDs (primary key).
/// All the other capabilities are optional and can vary from one adapter to another.
#[allow(private_bounds)]
#[async_trait]
pub trait Gateway {
    /// Returns a unique identifier for the gateway adapter
    /// (e.g., "stripe", "adyen").
    fn id(&self) -> &str;

    /// Get the status of a transaction.
    async fn status(&self, request: Request) -> Result<Response, Error>;
}

#[derive(Debug, Clone)]
pub struct Request {
    transaction_id: TransactionId,
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
