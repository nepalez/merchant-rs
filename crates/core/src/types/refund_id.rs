use std::convert::TryFrom;

use crate::error::*;
use crate::internal::*;

/// External refund identifier from payment gateway.
///
/// # Input Constraints
/// Max length 64: External ID. Same format as TransactionId, following payment gateway
/// standards (Stripe, PayPal, Braintree refund identifiers).
///
/// Sanitization: Minimal. Relies on validation to enforce the alphanumeric rule.
#[derive(Clone, Debug)]
pub struct RefundId(String);

// SAFETY: The trait is safely implemented because the type is not considered sensitive.
unsafe impl SafeWrapper for RefundId {
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

impl Sanitized for RefundId {
    const TRIM: bool = true;
}

impl Validated for RefundId {
    const TYPE_NAME: &'static str = "RefundId";
    const MAX_LENGTH: usize = 64;
    const EXTRA_CHARS: Option<&'static str> = Some("-_");
}

impl TryFrom<String> for RefundId {
    type Error = Error;

    #[inline]
    fn try_from(input: String) -> Result<Self> {
        Self::try_from_string(input)
    }
}
