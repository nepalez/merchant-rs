use std::convert::TryFrom;

use crate::Error;
use crate::inputs::Transaction as Input;
use crate::types::{
    MerchantInitiatedType, Money, TransactionId, TransactionIdempotenceKey, TransactionStatus,
};

/// Transaction result returned by payment gateway operations.
///
/// Represents the outcome of a payment operation (charge, authorize, capture, refund, void).
/// Contains the gateway-assigned transaction ID, idempotence key for duplicate detection,
/// current transaction status, processed amount, and merchant-initiated transaction type if applicable.
///
/// # Fields
///
/// * `transaction_id` - Unique identifier assigned by the payment gateway
/// * `idempotence_key` - Key used to prevent duplicate transaction processing
/// * `status` - Current transaction status (authorized, captured, failed, etc.)
/// * `amount` - Transaction amount and currency
/// * `merchant_initiated_type` - Type of merchant-initiated transaction (for recurring/unscheduled)
#[derive(Debug, Clone)]
pub struct Transaction {
    /// The unique transaction ID returned by the payment gateway.
    pub transaction_id: TransactionId,
    /// The idempotency key.
    pub idempotence_key: TransactionIdempotenceKey,
    /// The canonical status of the transaction.
    pub status: TransactionStatus,
    /// The amount of the transaction.
    pub amount: Money,
    /// The MIT (merchant initiated type of the transaction)
    pub merchant_initiated_type: Option<MerchantInitiatedType>,
}

impl Transaction {
    /// The unique transaction ID returned by the payment gateway.
    pub fn transaction_id(&self) -> &TransactionId {
        &self.transaction_id
    }

    /// The idempotency key.
    pub fn idempotence_key(&self) -> &TransactionIdempotenceKey {
        &self.idempotence_key
    }

    /// The canonical status of the transaction.
    pub fn status(&self) -> TransactionStatus {
        self.status
    }

    /// The amount of the transaction.
    pub fn amount(&self) -> Money {
        self.amount
    }

    /// The MIT (merchant initiated type of the transaction)
    pub fn merchant_initiated_type(&self) -> Option<MerchantInitiatedType> {
        self.merchant_initiated_type
    }
}

impl<'a> TryFrom<Input<'a>> for Transaction {
    type Error = Error;

    fn try_from(input: Input<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            transaction_id: input.transaction_id.try_into()?,
            idempotence_key: input.idempotence_key.try_into()?,
            status: input.status,
            amount: input.amount,
            merchant_initiated_type: input.merchant_initiated_type,
        })
    }
}
