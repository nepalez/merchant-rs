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

// SAFETY:
//
// The trait is safely implemented because:
// 1. The type by itself does not contain sensitive data, but the Debug
//    implementation prevents accidental exposure of arbitrary PII in logs.
// 2. The debug implementation is customized and does not expose any part of the inner string,
//    only its length.
unsafe impl SafeWrapper for ReasonForRefund {
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

impl Sanitized for ReasonForRefund {}

impl Validated for ReasonForRefund {
    const TYPE_NAME: &'static str = "ReasonForRefund";
    const MAX_LENGTH: usize = 255;
    // Skip chars validation
    const EXTRA_CHARS: Option<&'static str> = None;
}

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
        f.debug_tuple(<Self as Validated>::TYPE_NAME)
            .field(&masked)
            .finish()
    }
}
