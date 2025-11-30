use uuid::Uuid;

use crate::enums::{ECI, ThreeDSVersion};

/// Input for 3D Secure authentication result.
///
/// Used by gateway adapters to construct `SecuredPayment`.
/// All fields are optional to accommodate different gateway response patterns:
/// - Token-based (Stripe, Braintree, Adyen): token is enough, other fields are optional
/// - Field-based (Worldpay, NMI, Checkout.com): all fields are required
pub struct SecuredPayment<'a> {
    /// 3D Secure token from the gateway.
    pub token: Option<&'a str>,
    /// Cardholder Authentication Verification Value.
    pub cavv: Option<&'a str>,
    /// Electronic Commerce Indicator.
    pub eci: Option<ECI>,
    /// Directory Server Transaction ID (UUID per EMVCo spec).
    pub ds_transaction_id: Option<Uuid>,
    /// 3D Secure protocol version.
    pub version: Option<ThreeDSVersion>,
}
