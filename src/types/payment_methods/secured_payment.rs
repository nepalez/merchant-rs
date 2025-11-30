use uuid::Uuid;

use crate::Error;
use crate::enums::{ECI, ThreeDSVersion};
use crate::types::{CAVV, ThreeDSecureToken};

/// Result of 3D Secure authentication.
///
/// This type is constructed by gateway adapters after successful 3DS authentication
/// and returned to the caller. The caller never creates this type directly.
///
/// All fields are optional to accommodate different gateway response patterns:
/// - **Token-based** (Stripe, Braintree, Adyen): token is enough for payment,
///   other fields are optional metadata
/// - **Field-based** (Worldpay, NMI, Checkout.com): all fields except token are required
///   and must be passed explicitly in authorize/capture
///
/// This unified design allows callers to handle 3DS results uniformly regardless
/// of the underlying gateway implementation.
#[derive(Clone, Debug)]
pub struct SecuredPayment {
    pub(crate) token: Option<ThreeDSecureToken>,
    pub(crate) cavv: Option<CAVV>,
    pub(crate) eci: Option<ECI>,
    pub(crate) ds_transaction_id: Option<Uuid>,
    pub(crate) version: Option<ThreeDSVersion>,
}

impl SecuredPayment {
    /// 3D Secure token from the gateway.
    #[inline]
    pub fn token(&self) -> Option<&ThreeDSecureToken> {
        self.token.as_ref()
    }

    /// Cardholder Authentication Verification Value.
    #[inline]
    pub fn cavv(&self) -> Option<&CAVV> {
        self.cavv.as_ref()
    }

    /// Electronic Commerce Indicator.
    #[inline]
    pub fn eci(&self) -> Option<ECI> {
        self.eci
    }

    /// Directory Server Transaction ID (UUID per EMVCo spec).
    #[inline]
    pub fn ds_transaction_id(&self) -> Option<Uuid> {
        self.ds_transaction_id
    }

    /// 3D Secure protocol version.
    #[inline]
    pub fn version(&self) -> Option<ThreeDSVersion> {
        self.version
    }
}

impl TryFrom<crate::inputs::SecuredPayment<'_>> for SecuredPayment {
    type Error = Error;

    fn try_from(input: crate::inputs::SecuredPayment<'_>) -> Result<Self, Self::Error> {
        Ok(Self {
            token: input.token.map(ThreeDSecureToken::try_from).transpose()?,
            cavv: input.cavv.map(CAVV::try_from).transpose()?,
            eci: input.eci,
            ds_transaction_id: input.ds_transaction_id,
            version: input.version,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::inputs;

    fn valid_input() -> inputs::SecuredPayment<'static> {
        inputs::SecuredPayment {
            token: Some("pi_1234567890abcdef"),
            cavv: Some("AAABBBCCCdddeeefff111222333"),
            eci: Some(ECI::FullyAuthenticated),
            ds_transaction_id: Some(
                Uuid::parse_str("64d76f6d-e512-4aba-ae29-f7af0dc7db09").unwrap(),
            ),
            version: Some(ThreeDSVersion::V2_2_0),
        }
    }

    #[test]
    fn constructed_from_valid_input() {
        let input = valid_input();
        let result = SecuredPayment::try_from(input).unwrap();

        assert!(result.token().is_some());
        assert!(result.cavv().is_some());
        assert_eq!(result.eci(), Some(ECI::FullyAuthenticated));
        assert!(result.ds_transaction_id().is_some());
        assert_eq!(result.version(), Some(ThreeDSVersion::V2_2_0));
    }

    #[test]
    fn constructed_with_all_fields_none() {
        let input = inputs::SecuredPayment {
            token: None,
            cavv: None,
            eci: None,
            ds_transaction_id: None,
            version: None,
        };
        let result = SecuredPayment::try_from(input).unwrap();

        assert!(result.token().is_none());
        assert!(result.cavv().is_none());
        assert!(result.eci().is_none());
        assert!(result.ds_transaction_id().is_none());
        assert!(result.version().is_none());
    }

    #[test]
    fn rejects_invalid_token() {
        let mut input = valid_input();
        input.token = Some("ab");

        let result = SecuredPayment::try_from(input);
        assert!(matches!(result, Err(Error::InvalidInput(_))));
    }

    #[test]
    fn rejects_invalid_cavv() {
        let mut input = valid_input();
        input.cavv = Some("ab");

        let result = SecuredPayment::try_from(input);
        assert!(matches!(result, Err(Error::InvalidInput(_))));
    }
}
