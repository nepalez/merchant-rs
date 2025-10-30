use crate::Error;
use crate::inputs::ExternalPaymentData as Input;

/// Payment completion data for external payment flows.
///
/// Contains information needed to complete an external payment, such as
/// * Redirect URL for customer authentication (BNPL, online banking)
/// * Voucher codes or reference numbers (cash voucher systems)
/// * QR code data (mobile payments)
/// * Bank transfer instructions (account number, reference code)
///
/// The structure is currently empty as the specific fields depend on the payment method
/// and gateway implementation. Gateway adapters should extend this with method-specific data.
#[derive(Clone, Debug)]
pub struct ExternalPaymentData {}

impl<'a> TryFrom<Input<'a>> for ExternalPaymentData {
    type Error = Error;

    fn try_from(_input: Input<'a>) -> Result<Self, Self::Error> {
        Ok(Self {})
    }
}
