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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::AsUnsafeRef;
    use iso_currency::Currency;
    use rust_decimal_macros::dec;

    fn valid_input() -> Input<'static> {
        Input {
            transaction_id: " txn_12345678 \n\t",
            idempotence_key: " idempotence-key-123 \n\t",
            status: TransactionStatus::Captured,
            amount: Money {
                amount: dec!(100.00),
                currency: Currency::USD,
            },
            merchant_initiated_type: Some(MerchantInitiatedType::Recurring),
        }
    }

    #[test]
    fn constructed_from_valid_input() {
        let input = valid_input();
        let transaction = Transaction::try_from(input).unwrap();

        unsafe {
            assert_eq!(transaction.transaction_id.as_ref(), "txn_12345678");
            assert_eq!(transaction.idempotence_key.as_ref(), "idempotence-key-123");
            assert_eq!(transaction.status, TransactionStatus::Captured);
            assert_eq!(transaction.amount.amount, dec!(100.00));
            assert_eq!(transaction.amount.currency, Currency::USD);
            assert_eq!(
                transaction.merchant_initiated_type,
                Some(MerchantInitiatedType::Recurring)
            );
        }
    }

    #[test]
    fn rejects_invalid_transaction_id() {
        let mut input = valid_input();
        input.transaction_id = "short";

        let result = Transaction::try_from(input);
        assert!(matches!(result, Err(Error::InvalidInput(_))));
    }

    #[test]
    fn rejects_invalid_idempotence_key() {
        let mut input = valid_input();
        input.idempotence_key = "";

        let result = Transaction::try_from(input);
        assert!(matches!(result, Err(Error::InvalidInput(_))));
    }
}
