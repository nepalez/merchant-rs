use std::convert::{AsRef, TryFrom};

use crate::Error;
use crate::internal::{Validated, sanitized::*};

/// Administrative text explaining the reason for a payment reversal.
///
/// # Sanitization
/// * trims leading and trailing whitespace
/// * removes all ASCII control characters like newlines, tabs, etc.
///
/// # Validation
/// * length: 1-255 characters
///
/// # Data Protection
/// Reversal reasons are merchant-provided administrative text (e.g., "Duplicate payment",
/// "Wrong amount") and do not contain customer PII. Consequently, both `Debug` and `AsRef`
/// are implemented without masking.
#[derive(Clone, Debug)]
pub struct ReasonText(String);

impl<'a> TryFrom<&'a str> for ReasonText {
    type Error = Error;

    #[inline]
    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        Self::sanitize(input).validate()
    }
}

impl AsRef<str> for ReasonText {
    #[inline]
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

// --- Sealed traits (not parts of the public API) ---

impl Sanitized for ReasonText {
    #[inline]
    fn sanitize(input: &str) -> Self {
        let mut output = Self(String::with_capacity(input.len()));
        trim_whitespaces(&mut output.0, input);
        output
    }
}

impl Validated for ReasonText {
    #[inline]
    fn validate(self) -> Result<Self, Error> {
        self._validate_length(&self.0, 1, 255)?;
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_REASON: &str = "Duplicate payment";

    mod construction {
        use super::*;

        #[test]
        fn accepts_valid_reasons() {
            for input in [VALID_REASON, "Wrong amount", "A", &"a".repeat(255)] {
                let result = ReasonText::try_from(input);
                assert!(result.is_ok(), "{input:?} failed validation");
            }
        }

        #[test]
        fn removes_control_characters() {
            let input = " Duplicate payment \n\t\r ";
            let reason = ReasonText::try_from(input).unwrap();
            let result = reason.as_ref();
            assert_eq!(result, VALID_REASON);
        }

        #[test]
        fn rejects_empty_reason() {
            let input = "";
            let result = ReasonText::try_from(input);

            assert!(matches!(result, Err(Error::InvalidInput(_))));
        }

        #[test]
        fn rejects_too_long_reason() {
            let input = "a".repeat(256);
            let result = ReasonText::try_from(input.as_str());

            assert!(matches!(result, Err(Error::InvalidInput(_))));
        }
    }

    mod safety {
        use super::*;

        #[test]
        fn exposes_debug() {
            let reason = ReasonText::try_from(VALID_REASON).unwrap();
            let debug_output = format!("{:?}", reason);
            // ReasonText is administrative data, not PII, so it's not masked
            assert!(debug_output.contains(VALID_REASON));
        }

        #[test]
        fn as_ref_is_safe() {
            let input = " Duplicate payment \n\t";
            let reason = ReasonText::try_from(input).unwrap();
            let exposed = reason.as_ref();
            assert_eq!(exposed, VALID_REASON);
        }
    }
}
