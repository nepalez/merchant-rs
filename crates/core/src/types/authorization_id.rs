use std::convert::TryFrom;

use crate::error::*;
use crate::internal::*;

/// External authorization identifier from payment gateway.
///
/// # Input Constraints
/// Max length 64: External ID. Same format as TransactionId, following payment gateway
/// authorization tracking standards (Stripe, Authorize.Net, Braintree).
///
/// Sanitization: Minimal. Relies on validation to enforce the alphanumeric rule.
#[derive(Clone, Debug)]
pub struct AuthorizationId(String);

// SAFETY: The trait is safely implemented because this type is not considered sensitive data.
unsafe impl SafeWrapper for AuthorizationId {
    type Inner = String;

    #[inline]
    fn wrap(inner: Self::Inner) -> Self {
        Self(inner)
    }

    #[inline]
    unsafe fn inner(&self) -> &Self::Inner {
        &self.0
    }
}

impl Sanitized for AuthorizationId {
    const TRIM: bool = true;
}

impl Validated for AuthorizationId {
    const TYPE_NAME: &'static str = "AuthorizationId";
    const MAX_LENGTH: usize = 64;
    const EXTRA_CHARS: Option<&'static str> = Some("-_");
}

impl TryFrom<String> for AuthorizationId {
    type Error = Error;

    #[inline]
    fn try_from(input: String) -> Result<Self> {
        Self::try_from_string(input)
    }
}
