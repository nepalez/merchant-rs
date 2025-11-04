use std::convert::TryFrom;
use std::fmt;
use zeroize_derive::ZeroizeOnDrop;

use crate::Error;
use crate::internal::{AsUnsafeRef, Masked, Validated, sanitized::*};

/// Optional administrative text explaining the reason for a refund
///
/// # Sanitization
/// * trims leading and trailing whitespace
/// * removes all ASCII control characters like newlines, tabs, etc.
///
/// # Validation
/// * length: 1-255 characters
///
/// # Data Protection
/// Free-text fields may contain arbitrary PII (names, emails, phone numbers)
/// if merchants are poorly trained. Showing length only prevents accidental PII
/// exposure while maintaining the debugging utility.
///
/// As such, it is:
/// * masked in logs (via `Debug` implementation) to display only the length of the content,
/// * not exposed publicly except for a part of a request or response
///   via **unsafe** method `with_exposed_secret`.
#[derive(Clone, ZeroizeOnDrop)]
pub struct ReasonForRefund(String);

impl<'a> TryFrom<&'a str> for ReasonForRefund {
    type Error = Error;

    #[inline]
    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        Self(input.to_string()).validate()
    }
}

impl fmt::Debug for ReasonForRefund {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.masked_debug(f)
    }
}

impl AsUnsafeRef<str> for ReasonForRefund {
    #[inline]
    unsafe fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

// --- Sealed traits (not parts of the public API) ---

impl Sanitized for ReasonForRefund {
    fn sanitize(input: &str) -> Self {
        let mut output = Self(String::with_capacity(input.len()));
        trim_whitespaces(&mut output.0, input);
        output
    }
}

impl Validated for ReasonForRefund {
    fn validate(self) -> Result<Self, Error> {
        self._validate_length(&self.0, 1, 255)?;
        Ok(self)
    }
}

// SAFETY: The trait is safely implemented as it does NOT expose any part of the internal value.
unsafe impl Masked for ReasonForRefund {
    const TYPE_WRAPPER: &'static str = "ReasonForRefund";

    fn masked_debug(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let masked = format!("[{} chars]", self.0.chars().count());
        f.debug_tuple(Self::TYPE_WRAPPER).field(&masked).finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_REASON: &str = "Customer requested refund";

    mod construction {
        use super::*;

        #[test]
        fn accepts_valid_reasons() {
            for input in [VALID_REASON, "A", "a".repeat(255).as_str()] {
                let result = ReasonForRefund::try_from(input);
                assert!(result.is_ok(), "{input:?} failed validation");
            }
        }

        #[test]
        fn preserves_input() {
            // ReasonForRefund does NOT sanitize in try_from, only validates
            let input = "Customer requested refund";
            let reason = ReasonForRefund::try_from(input).unwrap();
            let result = unsafe { reason.as_ref() };
            assert_eq!(result, VALID_REASON);
        }

        #[test]
        fn rejects_empty_reason() {
            let input = "";
            let result = ReasonForRefund::try_from(input);

            assert!(matches!(result, Err(Error::InvalidInput(_))));
        }

        #[test]
        fn rejects_too_long_reason() {
            let input = "a".repeat(256);
            let result = ReasonForRefund::try_from(input.as_str());

            assert!(matches!(result, Err(Error::InvalidInput(_))));
        }
    }

    mod safety {
        use super::*;

        #[test]
        fn masks_debug() {
            let reason = ReasonForRefund::try_from(VALID_REASON).unwrap();
            let debug_output = format!("{:?}", reason);
            // Should show character count, not content. VALID_REASON = "Customer requested refund" has 25 chars
            assert!(debug_output.contains("ReasonForRefund"));
            assert!(debug_output.contains("25 chars"));
            assert!(!debug_output.contains("Customer"));
        }

        #[test]
        fn as_ref_returns_original_value() {
            // ReasonForRefund does NOT sanitize, so it returns original value
            let input = "Customer requested refund";
            let reason = ReasonForRefund::try_from(input).unwrap();
            let exposed = unsafe { reason.as_ref() };
            assert_eq!(exposed, VALID_REASON);
        }

        #[test]
        fn memory_is_not_leaked_after_drop() {
            let ptr: *const u8;
            let len: usize;
            unsafe {
                let reason = ReasonForRefund::try_from(VALID_REASON).unwrap();
                let s = reason.as_ref();
                ptr = s.as_ptr();
                len = s.len();
            }

            // SAFETY: This test verifies memory was zeroed after a drop.
            // Reading potentially freed memory is unsafe and only valid in tests
            // immediately after a drop, before any reallocation.
            unsafe {
                let slice = std::slice::from_raw_parts(ptr, len);
                let original_bytes = VALID_REASON.as_bytes();
                assert_ne!(
                    slice, original_bytes,
                    "Original reason should not remain in memory after drop"
                );
            }
        }
    }
}
