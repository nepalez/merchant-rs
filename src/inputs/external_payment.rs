use crate::inputs::{ExternalPaymentData, Transaction};

/// The data for completing an external payment along with the transaction.
pub struct ExternalPayment<'a> {
    /// The transaction to complete.
    pub transaction: Transaction<'a>,
    /// The data for payment completion.
    pub payment_data: ExternalPaymentData<'a>,
}
