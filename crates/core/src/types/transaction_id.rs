use std::convert::TryFrom;

use crate::error::{Error, Result};
use crate::types::{SafeWrapper, Sanitized, Validated};

const MAX_LENGTH: usize = 64;
const TYPE_NAME: &str = "Transaction ID";

/// External transaction identifier from payment gateway.
///
/// # Input Constraints
/// Max length 64: Covers crypto hashes and long PAG IDs (UPI, Stripe, PayPal).
/// Follows common payment gateway identifier formats (alphanumeric with separators).
///
/// Sanitization: Only trims. Any symbol (e.g., '.') must fail validation to maintain
/// strict API format integrity.
#[derive(Clone, Debug)]
pub struct TransactionId(String);

impl TryFrom<String> for TransactionId {
    type Error = Error;

    #[inline]
    fn try_from(input: String) -> Result<Self> {
        Self::try_from_string(input)
    }
}

// Sealed traits implementations

impl Sanitized for TransactionId {
    fn sanitize(input: String) -> Result<String> {
        let trimmed = input.trim();
        if trimmed.len() == input.len() {
            Ok(input)
        } else {
            Ok(trimmed.to_string())
        }
    }
}

impl Validated for TransactionId {
    const TYPE_NAME: &'static str = "Transaction ID";
    const MAX_LENGTH: usize = 64;
    const EXTRA_CHARS: Option<&'static str> = Some("-_");
}

impl SafeWrapper for TransactionId {
    type Inner = String;

    fn wrap(inner: String) -> Self {
        Self(inner)
    }
}
