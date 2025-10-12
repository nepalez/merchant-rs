use std::convert::TryFrom;
use std::fmt;

use crate::error::{Error, Result};
use crate::types::{SafeWrapper, Sanitized, Validated};

const MAX_LENGTH: usize = 50;
const TYPE_NAME: &str = "Cardholder name";
const DEBUG_MASK: &str = "***";

/// Cardholder name as it appears on the payment card.
///
/// # Input Constraints
/// Max length 50: EMV and ISO/IEC 7813 standard for embossed cardholder names.
/// Sanitization: Minimal (trim). Any non-Latin character (e.g., Cyrillic, Chinese)
/// must fail validation, not be transliterated/filtered.
///
/// # Security
/// Debug implementation masks all characters except first and last, both capitalized.
/// This is direct PII under GDPR/CCPA and must be protected in logs despite PCI DSS
/// not requiring cardholder name masking.
#[derive(Clone)]
pub struct CardHolderName(String);

impl TryFrom<String> for CardHolderName {
    type Error = Error;

    #[inline]
    fn try_from(input: String) -> Result<Self> {
        Self::try_from_string(input)
    }
}

// The first+last approach provides debugging context
// while ensuring multiple names collapse to the same mask
// (e.g., "Y", "Young Shy" and "Yury" all become "Y***Y"),
// thus preventing name reconstruction.
impl fmt::Debug for CardHolderName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let first_char = self.0.chars().next().unwrap();
        let last_char = self.0.chars().next_back().unwrap();
        let masked = format!(
            "{}{DEBUG_MASK}{}",
            first_char.to_uppercase(),
            last_char.to_uppercase(),
        );
        f.debug_tuple("CardHolderName").field(&masked).finish()
    }
}

// Sealed traits implementations

impl Sanitized for CardHolderName {
    fn sanitize(input: String) -> Result<String> {
        let trimmed = input.trim();
        if trimmed.len() == input.len() {
            Ok(input)
        } else {
            Ok(trimmed.to_string())
        }
    }
}

impl Validated for CardHolderName {
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
            .all(|c| c.is_ascii_alphabetic() || matches!(c, ' ' | '-' | '\'' | '.'))
        {
            Err(Error::validation_failed(format!(
                "{TYPE_NAME} must contain only ASCII letters, spaces, \
                and standard punctuation (-, ', .)"
            )))
        } else {
            Ok(())
        }
    }
}

impl SafeWrapper for CardHolderName {
    type Inner = String;

    fn wrap(inner: String) -> Self {
        Self(inner)
    }
}
