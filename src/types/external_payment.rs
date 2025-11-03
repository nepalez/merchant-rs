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
    /// The transaction to complete.
    pub transaction: Transaction,
    /// The data for payment completion.
    pub payment_data: ExternalPaymentData,
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
