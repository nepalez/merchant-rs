// merchant_reference_id.rs
use std::convert::TryFrom;

use crate::error::*;
use crate::internal::*;

/// Merchant's internal reference identifier for the transaction.
///
/// # Input Constraints
/// Max length 64: High limit for global PAGs (AliPay, Worldpay, Adyen) and complex
/// internal references. Follows ISO 20022 pain.001 merchant reference practices.
///
/// Sanitization: Only trims. Other symbols (e.g., "$") must cause validation failure.
#[derive(Clone, Debug)]
pub struct MerchantReferenceId(String);

impl TryFrom<String> for MerchantReferenceId {
    type Error = Error;

    #[inline]
    fn try_from(input: String) -> Result<Self> {
        Self::try_from_string(input)
    }
}

// Sealed traits implementations

impl Sanitized for MerchantReferenceId {
    const TRIM: bool = true;
}

impl Validated for MerchantReferenceId {
    const TYPE_NAME: &'static str = "Merchant reference ID";
    const MAX_LENGTH: usize = 64;
    const EXTRA_CHARS: Option<&'static str> = Some("-_.");
}

impl SafeWrapper for MerchantReferenceId {
    type Inner = String;

    fn wrap(inner: String) -> Self {
        Self(inner)
    }
}
