use std::convert::TryFrom;

use crate::error::{Error, Result};
use crate::types::{SafeWrapper, Sanitized, Validated};

const MAX_LENGTH: usize = 64;
const TYPE_NAME: &str = "Refund ID";

/// External refund identifier from payment gateway.
///
/// # Input Constraints
/// Max length 64: External ID. Same format as TransactionId, following payment gateway
/// standards (Stripe, PayPal, Braintree refund identifiers).
///
/// Sanitization: Minimal. Relies on validation to enforce the alphanumeric rule.
#[derive(Clone, Debug)]
pub struct RefundId(String);

impl TryFrom<String> for RefundId {
    type Error = Error;

    #[inline]
    fn try_from(input: String) -> Result<Self> {
        Self::try_from_string(input)
    }
}

// Sealed traits implementations

impl Sanitized for RefundId {
    fn sanitize(input: String) -> Result<String> {
        let trimmed = input.trim();
        if trimmed.len() == input.len() {
            Ok(input)
        } else {
            Ok(trimmed.to_string())
        }
    }
}

impl Validated for RefundId {
    const TYPE_NAME: &'static str = "Refund ID";
    const MAX_LENGTH: usize = 64;
    const EXTRA_CHARS: Option<&'static str> = Some("-_");
}

impl SafeWrapper for RefundId {
    type Inner = String;

    fn wrap(inner: String) -> Self {
        Self(inner)
    }
}
