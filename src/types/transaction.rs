use std::convert::TryFrom;

use iso_currency::Currency;

use crate::Error;
use crate::inputs::Transaction as Input;
use crate::types::{
    Destinations, MerchantInitiatedType, TransactionId, TransactionIdempotenceKey,
    TransactionStatus,
};

/// Transaction result returned by payment gateway operations.
///
/// Represents the outcome of a payment operation (charge, authorize, capture, refund, void).
/// Contains the gateway-assigned transaction ID, idempotence key for duplicate detection,
/// current transaction status, currency, payment destinations, and merchant-initiated transaction type if applicable.
#[derive(Debug, Clone)]
pub struct Transaction {
    pub(crate) transaction_id: TransactionId,
    pub(crate) idempotence_key: TransactionIdempotenceKey,
    pub(crate) status: TransactionStatus,
    pub(crate) currency: Currency,
    pub(crate) destinations: Destinations,
    pub(crate) merchant_initiated_type: Option<MerchantInitiatedType>,
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
    pub fn status(&self) -> &TransactionStatus {
        &self.status
    }

    /// The currency of the transaction.
    pub fn currency(&self) -> Currency {
        self.currency
    }

    /// The payment destinations (platform or split between recipients).
    pub fn destinations(&self) -> &Destinations {
        &self.destinations
    }

    /// The MIT (merchant initiated type of the transaction)
    pub fn merchant_initiated_type(&self) -> &Option<MerchantInitiatedType> {
        &self.merchant_initiated_type
    }
}

impl<'a> TryFrom<Input<'a>> for Transaction {
    type Error = Error;

    fn try_from(input: Input<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            transaction_id: input.transaction_id.try_into()?,
            idempotence_key: input.idempotence_key.try_into()?,
            status: input.status,
            currency: input.currency,
            destinations: input.destinations.try_into()?,
            merchant_initiated_type: input.merchant_initiated_type,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::AsUnsafeRef;
    use crate::inputs;
    use rust_decimal_macros::dec;

    fn valid_input() -> Input<'static> {
        Input {
            transaction_id: " txn_12345678 \n\t",
            idempotence_key: " idempotence-key-123 \n\t",
            status: TransactionStatus::Captured,
            currency: Currency::USD,
            destinations: inputs::Destinations::Platform(dec!(100.00)),
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
            assert_eq!(transaction.currency, Currency::USD);
            assert_eq!(transaction.destinations.total_amount(), dec!(100.00));
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
