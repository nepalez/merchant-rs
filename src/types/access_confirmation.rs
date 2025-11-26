use std::convert::TryFrom;
use std::fmt;
use zeroize_derive::ZeroizeOnDrop;

use crate::internal::{Masked, Validated, sanitized::*};
use crate::{AsUnsafeRef, Error};

/// Proof value from authorization callback (redirect flow, webhook, etc.)
///
/// # Sanitization
/// * trims whitespaces and control characters
///
/// # Validation
/// * length: 1-1024 characters
/// * alphanumeric and base64 characters (`-_+=/`) allowed
///
/// # Data Protection
/// Confirmation values can be used to complete authorization flows.
/// As such, they are:
/// * masked in logs (via `Debug` implementation)
/// * not exposed publicly except via **unsafe** `as_ref`
#[derive(Clone, ZeroizeOnDrop)]
pub struct AccessConfirmation(String);

impl<'a> TryFrom<&'a str> for AccessConfirmation {
    type Error = Error;

    #[inline]
    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        Self::sanitize(input).validate()
    }
}

impl fmt::Debug for AccessConfirmation {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.masked_debug(f)
    }
}

impl AsUnsafeRef<str> for AccessConfirmation {
    #[inline]
    unsafe fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

// --- Sealed traits (not parts of the public API) ---

impl Sanitized for AccessConfirmation {
    #[inline]
    fn sanitize(input: &str) -> Self {
        let mut output = Self(String::with_capacity(input.len()));
        trim_whitespaces(&mut output.0, input);
        output
    }
}

impl Validated for AccessConfirmation {
    fn validate(self) -> Result<Self, Error> {
        self._validate_length(&self.0, 1, 1024)?;
        self._validate_alphanumeric(&self.0, "-_+=/")?;
        Ok(self)
    }
}

// SAFETY: The trait is safely implemented because exposing the first 1 and last 1 character:
// 1. Neither causes out-of-bounds access to potentially INVALID (empty) data,
//    due to fallbacks to the empty strings,
// 2. Nor leaks the essential part of the sensitive VALID data,
//    while also hiding the real length and case.
unsafe impl Masked for AccessConfirmation {
    const TYPE_WRAPPER: &'static str = "AccessConfirmation";

    #[inline]
    fn first_chars(&self) -> String {
        self.0.get(0..1).unwrap_or_default().to_uppercase()
    }

    #[inline]
    fn last_chars(&self) -> String {
        self.0
            .get(self.0.len().saturating_sub(1)..)
            .unwrap_or_default()
            .to_uppercase()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_CONFIRMATION: &str = "seti_1234567890";

    mod construction {
        use super::*;

        #[test]
        fn accepts_valid_confirmations() {
            for input in [
                "a",
                "seti_1234567890",
                "RE123456789",
                "YWJjZGVm",        // base64
                "YWJjZGVm+/==",    // base64 with padding
                "abc-def_ghi",     // URL-safe
                &"a".repeat(1024), // max length
            ] {
                let result = AccessConfirmation::try_from(input);
                assert!(result.is_ok(), "{input:?} failed validation");
            }
        }

        #[test]
        fn removes_control_characters() {
            let input = " seti_1234567890 \n\t\r ";
            let confirmation = AccessConfirmation::try_from(input).unwrap();
            let result = unsafe { confirmation.as_ref() };
            assert_eq!(result, VALID_CONFIRMATION);
        }

        #[test]
        fn rejects_empty_string() {
            let result = AccessConfirmation::try_from("");
            assert!(matches!(result, Err(Error::InvalidInput(_))));
        }

        #[test]
        fn rejects_too_long() {
            let input = "a".repeat(1025);
            let result = AccessConfirmation::try_from(input.as_str());
            assert!(matches!(result, Err(Error::InvalidInput(_))));
        }

        #[test]
        fn rejects_invalid_characters() {
            for input in ["abc@def", "abc def", "abc<script>"] {
                let result = AccessConfirmation::try_from(input);
                assert!(
                    matches!(result, Err(Error::InvalidInput(_))),
                    "{input:?} should be rejected"
                );
            }
        }
    }

    mod safety {
        use super::*;

        #[test]
        fn masks_debug() {
            let confirmation = AccessConfirmation::try_from(VALID_CONFIRMATION).unwrap();
            let debug_output = format!("{:?}", confirmation);
            assert!(debug_output.contains(r#"AccessConfirmation("S***0")"#));
        }

        #[test]
        fn as_ref_is_unsafe() {
            static_assertions::assert_not_impl_all!(AccessConfirmation: AsRef<str>);

            let input = " seti_1234567890 \n\t";
            let confirmation = AccessConfirmation::try_from(input).unwrap();
            let exposed =
                unsafe { <AccessConfirmation as AsUnsafeRef<str>>::as_ref(&confirmation) };
            assert_eq!(exposed, VALID_CONFIRMATION);
        }

        #[test]
        fn memory_is_not_leaked_after_drop() {
            let ptr: *const u8;
            let len: usize;
            unsafe {
                let confirmation = AccessConfirmation::try_from(VALID_CONFIRMATION).unwrap();
                let s = confirmation.as_ref();
                ptr = s.as_ptr();
                len = s.len();
            }

            // SAFETY: This test verifies memory was zeroed after a drop.
            // Reading potentially freed memory is unsafe and only valid in tests
            // immediately after a drop, before any reallocation.
            unsafe {
                let slice = std::slice::from_raw_parts(ptr, len);
                let original_bytes = VALID_CONFIRMATION.as_bytes();
                assert_ne!(
                    slice, original_bytes,
                    "Original confirmation should not remain in memory after drop"
                );
            }
        }
    }
}
