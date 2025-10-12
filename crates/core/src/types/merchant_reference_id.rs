// merchant_reference_id.rs
use std::convert::TryFrom;

use crate::error::{Error, Result};
use crate::types::{SafeWrapper, Sanitized, Validated};

const MAX_LENGTH: usize = 64;
const TYPE_NAME: &str = "Merchant reference ID";

/// Merchant's internal reference identifier for the transaction.
///
/// Max length 64: High limit for global PAGs (AliPay, Worldpay, Adyen) and complex
/// internal references. Follows ISO 20022 pain.001 merchant reference practices.
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
    fn sanitize(input: String) -> Result<String> {
        let trimmed = input.trim();
        if trimmed.len() == input.len() {
            Ok(input)
        } else {
            Ok(trimmed.to_string())
        }
    }
}

impl Validated for MerchantReferenceId {
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
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.'))
        {
            Err(Error::validation_failed(format!(
                "{TYPE_NAME} must contain only alphanumeric characters, hyphens, underscores, and dots"
            )))
        } else {
            Ok(())
        }
    }
}

impl SafeWrapper for MerchantReferenceId {
    type Inner = String;

    fn wrap(inner: String) -> Self {
        Self(inner)
    }
}
