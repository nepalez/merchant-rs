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

impl TryFrom<String> for AuthorizationId {
    type Error = Error;

    #[inline]
    fn try_from(input: String) -> Result<Self> {
        Self::try_from_string(input)
    }
}

// Sealed traits implementations

impl Sanitized for AuthorizationId {
    fn sanitize(input: String) -> Result<String> {
        let trimmed = input.trim();
        if trimmed.len() == input.len() {
            Ok(input)
        } else {
            Ok(trimmed.to_string())
        }
    }
}

impl Validated for AuthorizationId {
    const TYPE_NAME: &'static str = "Authorization ID";
    const MAX_LENGTH: usize = 64;
    const EXTRA_CHARS: Option<&'static str> = Some("-_");
}

impl SafeWrapper for AuthorizationId {
    type Inner = String;

    fn wrap(inner: String) -> Self {
        Self(inner)
    }
}
