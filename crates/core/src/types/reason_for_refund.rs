use std::convert::TryFrom;
use std::fmt;

use crate::error::*;
use crate::internal::*;

/// Optional administrative text explaining the reason for a refund.
///
/// # Input Constraints
/// Max length 255: Standard limit for optional administrative text fields across
/// payment gateways (Stripe, PayPal, Adyen).
/// Cannot be empty - if no reason is provided, use `None` at the API level rather
/// than constructing an empty `ReasonForRefund`.
///
/// Sanitization: None needed, as all characters are technically allowed within
/// the length limit.
///
/// # Security
/// Debug implementation shows only the length of the content. Free-text fields may
/// contain arbitrary PII (names, emails, phone numbers) if merchants are poorly trained.
/// Showing length only prevents accidental PII exposure while maintaining debugging
/// utility.
#[derive(Clone)]
pub struct ReasonForRefund(String);

impl TryFrom<String> for ReasonForRefund {
    type Error = Error;

    #[inline]
    fn try_from(input: String) -> Result<Self> {
        Self::try_from_string(input)
    }
}

// Length-only display prevents exposure of arbitrary customer data that may be
// included in refund reasons.
impl fmt::Debug for ReasonForRefund {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let masked = format!("[{} chars]", self.0.len());
        f.debug_tuple("ReasonForRefund").field(&masked).finish()
    }
}

// Sealed traits implementations

impl Sanitized for ReasonForRefund {
    // Uses default implementation (no sanitization)
}

impl Validated for ReasonForRefund {
    const TYPE_NAME: &'static str = "Reason for refund";
    const MAX_LENGTH: usize = 255;
    // Skip chars validation
    const EXTRA_CHARS: Option<&'static str> = None;
}

impl SafeWrapper for ReasonForRefund {
    type Inner = String;

    fn wrap(inner: String) -> Self {
        Self(inner)
    }
}
