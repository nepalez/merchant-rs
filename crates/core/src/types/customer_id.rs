use std::convert::TryFrom;
use std::fmt;

use crate::error::*;
use crate::internal::*;

const DEBUG_MASK: &str = "***";

/// Customer identifier from external vault or payment system.
///
/// # Input Constraints
/// Max length 50: Standard for vault customer identifiers (Stripe, Braintree, PayPal).
/// Follows common payment gateway practices for customer reference IDs.
/// Character set includes alphanumeric and common separators (-, _, .) used across
/// major payment platforms.
///
/// Sanitization: Only trims whitespace (common user/system error).
/// Any other non-compliant char will fail validation.
///
/// # Security
/// Debug implementation masks all characters except first and last, both uppercased.
/// While not classified as direct PII, customer IDs enable transaction correlation
/// and customer profiling. Defense-in-depth approach prevents accidental exposure
/// in logs and complies with strict interpretations of GDPR Article 4(1)
/// regarding indirect identifiers.
#[derive(Clone)]
pub struct CustomerId(String);

// SAFETY:
//
// The trait is safely implemented because:
// 1. String is used as the inner type because customer id is not considered
//    "sensitive" under PCI DSS. However, it can be used for indirect identification
//    of a customer, so we still implement masking in Debug.
// 2. Exposes 1 first and 1 last char, both capitalized, in Debug implementation
//    which with a help of mask in between doesn't reveal the name, but
//    mixes it with other names having the same first and last letters.
// 3. Validation ensures that the name has at least 1 character,
//    which prevents out-of-bounds error in Debug implementation.
unsafe impl SafeWrapper for CustomerId {
    type Inner = String;

    const FIRST_CHARS: usize = 1;
    const LAST_CHARS: usize = 1;

    #[inline]
    fn wrap(inner: Self::Inner) -> Self {
        Self(inner)
    }

    #[inline]
    unsafe fn inner(&self) -> &Self::Inner {
        &self.0
    }
}

impl Sanitized for CustomerId {
    const TRIM: bool = true;
}

impl Validated for CustomerId {
    const TYPE_NAME: &'static str = "CustomerId";
    const MAX_LENGTH: usize = 50;
    const EXTRA_CHARS: Option<&'static str> = Some("-_.");
}

impl TryFrom<String> for CustomerId {
    type Error = Error;

    #[inline]
    fn try_from(input: String) -> Result<Self> {
        Self::try_from_string(input)
    }
}

impl fmt::Debug for CustomerId {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.masked_debug(f)
    }
}
