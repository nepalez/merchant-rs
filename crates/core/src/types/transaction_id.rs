use std::convert::TryFrom;

use crate::error::*;
use crate::internal::*;

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

// SAFETY: The trait is safely implemented because the type is not considered sensitive.
unsafe impl SafeWrapper for TransactionId {
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

impl Sanitized for TransactionId {
    const TRIM: bool = true;
}

impl Validated for TransactionId {
    const TYPE_NAME: &'static str = "TransactionId";
    const MAX_LENGTH: usize = 64;
    const EXTRA_CHARS: Option<&'static str> = Some("-_");
}

impl TryFrom<String> for TransactionId {
    type Error = Error;

    #[inline]
    fn try_from(input: String) -> Result<Self> {
        Self::try_from_string(input)
    }
}
