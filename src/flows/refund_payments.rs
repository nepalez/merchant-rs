use async_trait::async_trait;

use crate::Error;
use crate::types::{MerchantReferenceId, Transaction, TransactionId};

/// Trait for payment gateways that support the return of funds to a customer.
#[async_trait]
pub trait RefundPayments {
    async fn refund(
        &self,
        transaction_id: TransactionId,
        merchant_reference_id: MerchantReferenceId,
    ) -> Result<Transaction, Error>;
}
