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

impl TryFrom<String> for CustomerId {
    type Error = Error;

    #[inline]
    fn try_from(input: String) -> Result<Self> {
        Self::try_from_string(input)
    }
}

// The first+last approach prevents length disclosure through mask format.
// Multiple IDs collapse to the same mask (e.g., "c", "customer_id_abc"
// all become "C****C" variations), preventing ID reconstruction.
impl fmt::Debug for CustomerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let first_char = self.0.chars().next().unwrap();
        let last_char = self.0.chars().next_back().unwrap();
        let masked = format!(
            "{}{DEBUG_MASK}{}",
            first_char.to_uppercase(),
            last_char.to_uppercase(),
        );
        f.debug_tuple("CustomerId").field(&masked).finish()
    }
}

// Sealed traits implementations

impl Sanitized for CustomerId {
    const TRIM: bool = true;
}

impl Validated for CustomerId {
    const TYPE_NAME: &'static str = "Customer ID";
    const MAX_LENGTH: usize = 50;
    const EXTRA_CHARS: Option<&'static str> = Some("-_.");
}

impl SafeWrapper for CustomerId {
    type Inner = String;

    fn wrap(inner: String) -> Self {
        Self(inner)
    }
}
