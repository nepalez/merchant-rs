use async_trait::async_trait;

use crate::Error;
use crate::internal::PaymentSource;
use crate::types::{BNPL, CashVoucher, ExternalPayment, ExternalPaymentData, Token, TransactionId};

/// Optional trait for payment gateways that support asynchronous payments.
///
/// Within the flow the payment is initiated but not completed. The completion
/// of the payment happens outside the current flow, e.g., via redirect or webhook.
///
/// The `initiate` method builds a transaction along with the data for payment completion
/// (like the link to redirect the customer, or some data like the Multibanco does etc.)
/// The adapter can also send the data under the hood (for example, if it requires
/// the customer's phone to send him the SMS).
///
/// The client can:
/// * show the data to the customer or redirect him to some url,
///   and then wait for the completion of the transaction;
/// * receive the `payment_data` later by the `transaction_id`
///   to repeat the redirect or display the data in the UI;
/// * check the status of the transaction via `CheckTransactions` trait implementation;
/// * handle the webhook response using the `HandleWebhooks` trait implementation.
#[async_trait]
pub trait ExternalPayments {
    #[allow(private_bounds)]
    type Source: Source;

    /// Initiate the transaction and receive it along with a `PaymentData`
    /// The payment should be made outside the gateway's flow.
    /// The client should check the status of the transaction later.
    async fn initiate(&self, source: Self::Source) -> Result<ExternalPayment, Error>;

    /// Retrieve the payment data for a previously initiated transaction by its ID.
    async fn payment_data(
        &self,
        transaction_id: TransactionId,
    ) -> Result<ExternalPaymentData, Error>;
}

/// Marker trait for sources that support external payments.
trait Source: PaymentSource {}
impl Source for BNPL {}
impl Source for CashVoucher {}
impl Source for Token {}
