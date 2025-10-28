use async_trait::async_trait;

use crate::Error;
use crate::types::{MerchantReferenceId, Payment, Transaction, TransactionId};

/// Optional trait for payment gateways that support completing a two-step flow,
/// where the first step is an authorization and the second is a capture.
#[async_trait]
pub trait TwoStepPayments {
    async fn authorize(&self, payment: Payment) -> Result<Transaction, Error>;

    /// Confirms and debits the previously authorized funds.
    async fn capture(
        &self,
        transaction_id: TransactionId,
        merchant_reference_id: MerchantReferenceId,
    ) -> Result<Transaction, Error>;
}
