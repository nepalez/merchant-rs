use crate::Error;
use crate::inputs::ExternalPayment as Input;
use crate::types::{ExternalPaymentData, Transaction};

/// Result from initiating an external payment flow.
///
/// Contains the transaction record along with payment data needed for external completion
/// (e.g., redirect URL, voucher code, QR code data).
///
/// External payments require completion outside the immediate flow, such as
/// * Customer redirect to payment provider (BNPL, online banking)
/// * Display of payment instructions (voucher code, bank transfer details)
/// * QR code scanning for mobile payments
///
/// The client should use `payment_data` to guide the customer through the completion process,
/// then check transaction status via the `CheckTransaction` trait or handle webhook notifications.
pub struct ExternalPayment {
    pub(crate) transaction: Transaction,
    pub(crate) payment_data: ExternalPaymentData,
}

impl ExternalPayment {
    /// The transaction to complete.
    #[inline]
    pub fn transaction(&self) -> &Transaction {
        &self.transaction
    }

    /// The data for payment completion.
    #[inline]
    pub fn payment_data(&self) -> &ExternalPaymentData {
        &self.payment_data
    }
}

impl<'a> TryFrom<Input<'a>> for ExternalPayment {
    type Error = Error;

    fn try_from(input: Input<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            transaction: input.transaction.try_into()?,
            payment_data: input.payment_data.try_into()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::AsUnsafeRef;
    use crate::inputs;
    use crate::types::{MerchantInitiatedType, TransactionStatus};
    use iso_currency::Currency;

    fn valid_input() -> Input<'static> {
        Input {
            transaction: inputs::Transaction {
                transaction_id: " txn_12345678 \n\t",
                idempotence_key: " idempotence-key-123 \n\t",
                status: TransactionStatus::Captured,
                currency: Currency::USD,
                recipients: None,
                merchant_initiated_type: Some(MerchantInitiatedType::Recurring),
            },
            payment_data: Default::default(),
        }
    }

    #[test]
    fn constructed_from_valid_input() {
        let input = valid_input();
        let external_payment = ExternalPayment::try_from(input).unwrap();

        unsafe {
            assert_eq!(
                external_payment.transaction.transaction_id.as_ref(),
                "txn_12345678"
            );
            assert_eq!(
                external_payment.transaction.idempotence_key.as_ref(),
                "idempotence-key-123"
            );
            assert_eq!(
                external_payment.transaction.status,
                TransactionStatus::Captured
            );
            assert_eq!(external_payment.transaction.currency, Currency::USD);
            assert!(external_payment.transaction.recipients.is_none());
            assert_eq!(
                external_payment.transaction.merchant_initiated_type,
                Some(MerchantInitiatedType::Recurring)
            );
        }
    }

    #[test]
    fn rejects_invalid_transaction_id() {
        let mut input = valid_input();
        input.transaction.transaction_id = "short";

        let result = ExternalPayment::try_from(input);
        assert!(matches!(result, Err(Error::InvalidInput(_))));
    }

    #[test]
    fn rejects_invalid_idempotence_key() {
        let mut input = valid_input();
        input.transaction.idempotence_key = "";

        let result = ExternalPayment::try_from(input);
        assert!(matches!(result, Err(Error::InvalidInput(_))));
    }
}
