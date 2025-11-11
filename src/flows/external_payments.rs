use async_trait::async_trait;

use crate::Error;
use crate::Gateway;
use crate::types::{ExternalPayment, ExternalPaymentData, ExternalPaymentMethod, TransactionId};

/// Payment gateway trait for asynchronous external payment flows.
///
/// Supports payment methods where the transaction is initiated but not immediately completed.
/// The payment completion happens outside the direct flow through customer redirect, webhook, or other async mechanism.
///
/// # Flow
///
/// 1. **Initiate**: Create transaction and receive payment completion data
/// 2. **External Completion**: Customer completes payment through redirect/voucher/QR code
/// 3. **Status Check**: Poll transaction status via `CheckTransaction` trait
/// 4. **Webhook**: Receive async notification of completion (if supported by gateway)
///
/// # Payment Completion Methods
///
/// * **Redirect**: Customer redirected to payment provider (BNPL, online banking)
/// * **Voucher**: Customer receives code to pay at physical location (cash vouchers)
/// * **QR Code**: Customer scans code with mobile banking app
/// * **Bank Transfer**: Customer makes manual transfer with reference number
///
/// # Usage
///
/// The client application should:
/// * Display `payment_data` to guide customer through completion
/// * Poll transaction status using `CheckTransaction::status()`
/// * Handle webhook notifications for async status updates
/// * Retrieve payment data again via `payment_data()` if needed for retry
///
/// # Type Parameter
///
/// * `Method` - Payment method type constrained to external methods (vouchers, BNPL, etc.)
#[async_trait]
pub trait ExternalPayments: Gateway {
    #[allow(private_bounds)]
    type Method: ExternalPaymentMethod;

    /// Initiate the transaction and receive it along with a `PaymentData`
    /// The payment should be made outside the gateway's flow.
    /// The client should check the status of the transaction later.
    async fn initiate(&self, method: Self::Method) -> Result<ExternalPayment, Error>;

    /// Retrieve the payment data for a previously initiated transaction by its ID.
    async fn payment_data(
        &self,
        transaction_id: TransactionId,
    ) -> Result<ExternalPaymentData, Error>;
}
