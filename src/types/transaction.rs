use iso_currency::Currency;
use std::convert::TryFrom;

use crate::types::{Recipients, TransactionId, TransactionIdempotenceKey};
use crate::{Error, MerchantInitiatedType, TransactionStatus};

/// Transaction result returned by payment gateway operations.
///
/// Represents the outcome of a payment operation (charge, authorize, capture, refund, void).
/// Contains the gateway-assigned transaction ID, idempotence key for duplicate detection,
/// current transaction status, currency, payment recipients, and merchant-initiated transaction type if applicable.
#[derive(Debug, Clone)]
pub struct Transaction {
    pub(crate) transaction_id: TransactionId,
    pub(crate) idempotence_key: TransactionIdempotenceKey,
    pub(crate) status: TransactionStatus,
    pub(crate) currency: Currency,
    pub(crate) recipients: Option<Recipients>,
    pub(crate) merchant_initiated_type: Option<MerchantInitiatedType>,
}

impl Transaction {
    /// The unique transaction ID returned by the payment gateway.
    #[inline]
    pub fn transaction_id(&self) -> &TransactionId {
        &self.transaction_id
    }

    /// The idempotency key.
    #[inline]
    pub fn idempotence_key(&self) -> &TransactionIdempotenceKey {
        &self.idempotence_key
    }

    /// The canonical status of the transaction.
    #[inline]
    pub fn status(&self) -> &TransactionStatus {
        &self.status
    }

    /// The currency of the transaction.
    #[inline]
    pub fn currency(&self) -> Currency {
        self.currency
    }

    /// The payment recipients (None = platform receives all).
    #[inline]
    pub fn recipients(&self) -> Option<&Recipients> {
        self.recipients.as_ref()
    }

    /// The MIT (merchant initiated type of the transaction)
    #[inline]
    pub fn merchant_initiated_type(&self) -> Option<&MerchantInitiatedType> {
        self.merchant_initiated_type.as_ref()
    }
}

impl<'a> TryFrom<crate::Transaction<'a>> for Transaction {
    type Error = Error;

    fn try_from(input: crate::Transaction<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            transaction_id: input.transaction_id.try_into()?,
            idempotence_key: input.idempotence_key.try_into()?,
            status: input.status,
            currency: input.currency,
            recipients: input.recipients.map(TryFrom::try_from).transpose()?,
            merchant_initiated_type: input.merchant_initiated_type,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::AsUnsafeRef;

    fn valid_input() -> crate::Transaction<'static> {
        crate::Transaction {
            transaction_id: " txn_12345678 \n\t",
            idempotence_key: " idempotence-key-123 \n\t",
            status: TransactionStatus::Captured,
            currency: Currency::USD,
            recipients: None,
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
            assert!(transaction.recipients.is_none());
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
