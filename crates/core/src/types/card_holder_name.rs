use std::convert::TryFrom;
use std::fmt;

use crate::error::*;
use crate::internal::*;

const DEBUG_MASK: &str = "***";

/// Cardholder name as it appears on the payment card.
///
/// # Input Constraints
/// Max length 50: EMV and ISO/IEC 7813 standard for embossed cardholder names.
///
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

// SAFETY:
//
// The trait is safely implemented because:
// 1. String is used as the inner type because cardholder name is not considered
//    "sensitive" under PCI DSS. However, it is direct PII under GDPR/CCPA,
//    so we still implement masking in Debug.
// 2. Exposes 1 first and 1 last char, both capitalized, in Debug implementation
//    which with a help of mask in between doesn't reveal the name, but
//    mixes it with other names having the same first and last letters.
// 3. Validation ensures that the name has at least 1 character,
//    which prevents out-of-bounds error in Debug implementation.
unsafe impl SafeWrapper for CardHolderName {
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

impl Sanitized for CardHolderName {
    const TRIM: bool = true;
}

impl Validated for CardHolderName {
    const TYPE_NAME: &'static str = "CardHolderName";
    const MAX_LENGTH: usize = 50;
    // Custom use for this type: see implementation below!
    const EXTRA_CHARS: Option<&'static str> = Some(" -'.");

    #[inline]
    fn validate(input: &str) -> Result<()> {
        Self::validate_length(input)?;

        // Safe unwrap as per definition above
        let extra = Self::EXTRA_CHARS.unwrap();
        if !input
            .chars()
            .all(|c| c.is_ascii_alphabetic() || extra.contains(c))
        {
            return Err(Error::validation_failed(format!(
                "{} must contain only ASCII letters and '{}'",
                Self::TYPE_NAME,
                extra
            )));
        }

        Ok(())
    }
}

impl fmt::Debug for CardHolderName {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.masked_debug(f)
    }
}
