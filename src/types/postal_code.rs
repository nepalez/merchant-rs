use std::convert::TryFrom;
use std::fmt;
use zeroize_derive::ZeroizeOnDrop;

use crate::Error;
use crate::internal::{AsUnsafeRef, Masked, Validated, sanitized::*};

/// Postal code used in addresses
///
/// # Sanitization
/// * trims whitespaces,
/// * removes all ASCII control characters like newlines, tabs, etc.
///
/// # Validation
/// * length: 3-10 characters,
/// * only alphanumeric characters, spaces and dashes are allowed
///
/// # Data Protection
/// Postal codes can identify specific geographic areas
/// and when combined with other data, enable person identification,
/// making them PII (Personal Identifiable Information).
///
/// As such, they are:
/// * masked in logs (via `Debug` implementation) to display
///   up to the first 2 characters but no more than 1/3 of the code length,
/// * not exposed publicly except for a part of a request or response
///   via **unsafe** method `with_exposed_secret`.
#[derive(Clone, ZeroizeOnDrop)]
pub struct PostalCode(String);

impl<'a> TryFrom<&'a str> for PostalCode {
    type Error = Error;

    #[inline]
    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        Self::sanitize(input).validate()
    }
}

impl fmt::Debug for PostalCode {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.masked_debug(f)
    }
}

impl AsUnsafeRef<str> for PostalCode {
    #[inline]
    unsafe fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

// --- Sealed traits (not parts of the public API) ---

impl Sanitized for PostalCode {
    #[inline]
    fn sanitize(input: &str) -> Self {
        let mut output = Self(String::with_capacity(input.len()));
        trim_whitespaces(&mut output.0, input);
        output
    }
}

impl Validated for PostalCode {
    // We don't care about zeroization of the temporary data, that is not PII.
    #[inline]
    fn validate(self) -> Result<Self, Error> {
        self._validate_length(&self.0, 3, 10)?;
        self._validate_alphanumeric(&self.0, "- ")?;
        Ok(self)
    }
}

// SAFETY: The trait is safely implemented because exposing
// up to the first 2 characters, but no more than 1/3 of the code length:
// 1. Neither causes out-of-bounds access to potentially INVALID (empty) data,
//    due to fallbacks to the empty strings,
// 2. Nor leaks the sensitive VALID data because the first part of the code
//    points out to a broad geographical area.
unsafe impl Masked for PostalCode {
    const TYPE_WRAPPER: &'static str = "PostalCode";

    #[inline]
    fn first_chars(&self) -> String {
        let len = (self.0.len() / 3).min(2);
        self.0.get(0..len).unwrap_or_default().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_CODE_US: &str = "12345";
    const VALID_CODE_UK: &str = "SW1A 1AA";
    const VALID_CODE_CA: &str = "K1A-0B1";

    mod construction {
        use super::*;

        #[test]
        fn accepts_valid_codes() {
            for input in [VALID_CODE_US, VALID_CODE_UK, VALID_CODE_CA, "123", "1234567890"] {
                let result = PostalCode::try_from(input);
                assert!(result.is_ok(), "{input:?} failed validation");
            }
        }

        #[test]
        fn removes_control_characters() {
            let input = " 12345 \n\t\r ";
            let code = PostalCode::try_from(input).unwrap();
            let result = unsafe { code.as_ref() };
            assert_eq!(result, VALID_CODE_US);
        }

        #[test]
        fn rejects_too_short_code() {
            let input = "12"; // 2 characters
            let result = PostalCode::try_from(input);

            assert!(matches!(result, Err(Error::InvalidInput(_))));
        }

        #[test]
        fn rejects_too_long_code() {
            let input = "12345678901"; // 11 characters
            let result = PostalCode::try_from(input);

            assert!(matches!(result, Err(Error::InvalidInput(_))));
        }

        #[test]
        fn rejects_invalid_characters() {
            let input = "123@45";
            let result = PostalCode::try_from(input);

            assert!(matches!(result, Err(Error::InvalidInput(_))));
        }
    }

    mod safety {
        use super::*;

        #[test]
        fn masks_debug_short() {
            let code = PostalCode::try_from(VALID_CODE_US).unwrap();
            let debug_output = format!("{:?}", code);
            // For 5 chars: 5/3 = 1, min(1, 2) = 1
            assert!(debug_output.contains(r#"PostalCode("1***")"#));
        }

        #[test]
        fn masks_debug_long() {
            let code = PostalCode::try_from("1234567890").unwrap();
            let debug_output = format!("{:?}", code);
            // For 10 chars: 10/3 = 3, min(3, 2) = 2
            assert!(debug_output.contains(r#"PostalCode("12***")"#));
        }

        #[test]
        fn as_ref_is_unsafe() {
            static_assertions::assert_not_impl_all!(PostalCode: AsRef<str>);

            let input = " 12345 \n\t";
            let code = PostalCode::try_from(input).unwrap();
            let exposed = unsafe { <PostalCode as AsUnsafeRef<str>>::as_ref(&code) };
            assert_eq!(exposed, VALID_CODE_US);
        }

        #[test]
        fn memory_is_not_leaked_after_drop() {
            let ptr: *const u8;
            let len: usize;
            unsafe {
                let code = PostalCode::try_from(VALID_CODE_US).unwrap();
                let s = code.as_ref();
                ptr = s.as_ptr();
                len = s.len();
            }

            // SAFETY: This test verifies memory was zeroed after a drop.
            // Reading potentially freed memory is unsafe and only valid in tests
            // immediately after a drop, before any reallocation.
            unsafe {
                let slice = std::slice::from_raw_parts(ptr, len);
                let original_bytes = VALID_CODE_US.as_bytes();
                assert_ne!(
                    slice, original_bytes,
                    "Original postal code should not remain in memory after drop"
                );
            }
        }
    }
}
