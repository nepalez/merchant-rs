use std::convert::TryFrom;

use crate::error::{Error, Result};
use crate::types::{SafeWrapper, Sanitized, Validated};

const MAX_LENGTH: usize = 64;
const TYPE_NAME: &str = "Authorization ID";

/// External authorization identifier from payment gateway.
///
/// Max length 64: External ID. Same format as TransactionId, following payment gateway
/// authorization tracking standards (Stripe, Authorize.Net, Braintree).
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
    fn validate(input: &str) -> Result<()> {
        let len = input.len();

        if len == 0 {
            Err(Error::validation_failed(format!(
                "{TYPE_NAME} cannot be empty"
            )))
        } else if len > MAX_LENGTH {
            Err(Error::validation_failed(format!(
                "{TYPE_NAME} length ({len}) exceeds maximum ({MAX_LENGTH})"
            )))
        } else if !input
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_'))
        {
            Err(Error::validation_failed(format!(
                "{TYPE_NAME} must contain only alphanumeric characters, hyphens, and underscores"
            )))
        } else {
            Ok(())
        }
    }
}

impl SafeWrapper for AuthorizationId {
    type Inner = String;

    fn wrap(inner: String) -> Self {
        Self(inner)
    }
}
