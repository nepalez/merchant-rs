use std::fmt;
use zeroize_derive::ZeroizeOnDrop;

use crate::internal::{Masked, Validated, sanitized::*};
use crate::{AsUnsafeRef, Error};

/// Full name of a payer
///
/// # Sanitization
/// * trims whitespaces,
/// * removes all ASCII control characters like newlines, tabs, etc.
///
/// # Validation
/// * length: 3-60 characters,
/// * only ASCII alphanumerics, spaces, dashes, apostrophes and dots are allowed,
/// * any non-Latin character (e.g., Cyrillic, Chinese) fails validation
///
/// # Data Protection
/// While PCI DSS does NOT classify names as sensitive authentication data (SAD),
/// they are critical PII and financial access data that can be associated with their owners.
///
/// As such, they are:
/// * masked in logs (via `Debug` implementation) to display
///   the first and last characters (both in the upper case) only,
///   which prevents leaking short names,
/// * not exposed publicly except for a part of a request or response
///   via **unsafe** method `with_exposed_secret`.
#[derive(Clone, ZeroizeOnDrop)]
pub struct FullName(String);

impl<'a> TryFrom<&'a str> for FullName {
    type Error = Error;

    #[inline]
    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        let original = Self::sanitize(input).validate()?;
        Ok(Self(original.0.to_uppercase()))
    }
}

impl fmt::Debug for FullName {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <Self as Masked>::masked_debug(self, f)
    }
}

impl AsUnsafeRef<str> for FullName {
    #[inline]
    unsafe fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

// --- Sealed traits (not parts of the public API) ---

impl Sanitized for FullName {
    #[inline]
    fn sanitize(input: &str) -> Self {
        let mut output = Self(String::with_capacity(input.len()));
        trim_whitespaces(&mut output.0, input);
        output
    }
}

impl Validated for FullName {
    #[inline]
    fn validate(self) -> Result<Self, Error> {
        self._validate_length(&self.0, 3, 60)?;
        self._validate_alphanumeric(&self.0, " -'.")?;
        Ok(self)
    }
}

/// # Safety
/// The trait is safely implemented because exposing the first 1 and last 1 character:
/// 1. Neither causes out-of-bounds access to potentially INVALID (empty) data,
///    due to fallbacks to the empty strings,
/// 2. Nor leaks the VALID data due to hiding the real length of the full name.
unsafe impl Masked for FullName {
    const TYPE_WRAPPER: &'static str = "FullName";

    #[inline]
    fn first_chars(&self) -> String {
        self.0.get(0..1).unwrap_or_default().to_string()
    }

    #[inline]
    fn last_chars(&self) -> String {
        let len = self.0.len();
        self.0.get(len - 1..len).unwrap_or_default().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_NAME: &str = "John Doe";
    const VALID_NAME_UPPER: &str = "JOHN DOE";
    const VALID_NAME_WITH_NUMBERS: &str = "John Doe 3rd";

    mod construction {
        use super::*;

        #[test]
        fn accepts_valid_names() {
            for input in [
                VALID_NAME,
                "Ann Smith",
                "Mary-Jane O'Brien",
                "Dr. House",
                VALID_NAME_WITH_NUMBERS,
            ] {
                let result = FullName::try_from(input);
                assert!(result.is_ok(), "{input:?} failed validation");
            }
        }

        #[test]
        fn removes_control_characters_and_converts_to_uppercase() {
            let input = " john doe \n\t\r ";
            let name = FullName::try_from(input).unwrap();
            let result = unsafe { name.as_ref() };
            assert_eq!(result, VALID_NAME_UPPER);
        }

        #[test]
        fn rejects_too_short_name() {
            let input = "Ab"; // 2 characters
            let result = FullName::try_from(input);

            if let Err(Error::InvalidInput(msg)) = result {
                assert!(msg.contains("***"));
            } else {
                panic!("Expected InvalidInput error, got {result:?}");
            }
        }

        #[test]
        fn rejects_too_long_name() {
            let input = "a".repeat(61);
            let result = FullName::try_from(input.as_str());

            assert!(matches!(result, Err(Error::InvalidInput(_))));
        }

        #[test]
        fn rejects_non_alphanumeric_characters() {
            let input = "John Doe @";
            let result = FullName::try_from(input);

            assert!(matches!(result, Err(Error::InvalidInput(_))));
        }
    }

    mod safety {
        use super::*;

        #[test]
        fn masks_debug() {
            let name = FullName::try_from(VALID_NAME).unwrap();
            let debug_output = format!("{:?}", name);
            assert!(debug_output.contains(r#"FullName("J***E")"#));
        }

        #[test]
        fn as_ref_is_unsafe() {
            static_assertions::assert_not_impl_all!(FullName: AsRef<str>);

            let input = " john doe \n\t";
            let name = FullName::try_from(input).unwrap();
            let exposed = unsafe { <FullName as AsUnsafeRef<str>>::as_ref(&name) };
            assert_eq!(exposed, VALID_NAME_UPPER);
        }

        #[test]
        fn memory_is_not_leaked_after_drop() {
            let ptr: *const u8;
            let len: usize;
            unsafe {
                let name = FullName::try_from(VALID_NAME).unwrap();
                let s = name.as_ref();
                ptr = s.as_ptr();
                len = s.len();
            }

            // SAFETY: This test verifies memory was zeroed after a drop.
            // Reading potentially freed memory is unsafe and only valid in tests
            // immediately after a drop, before any reallocation.
            unsafe {
                let slice = std::slice::from_raw_parts(ptr, len);
                let original_bytes = VALID_NAME_UPPER.as_bytes();
                assert_ne!(
                    slice, original_bytes,
                    "Original full name should not remain in memory after drop"
                );
            }
        }
    }
}
