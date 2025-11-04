use std::convert::TryFrom;
use std::fmt;
use zeroize_derive::ZeroizeOnDrop;

use crate::internal::{Masked, Validated, sanitized::*};
use crate::{AsUnsafeRef, Error};

/// National identification number of the user
///
/// # Sanitization
/// * removes common separators: spaces, dashes, dots, underscores, and apostrophes,
/// * removes all ASCII control characters like newlines, tabs, etc.
///
/// # Validation
/// * length: 7-18 characters,
/// * only alphanumeric characters are allowed
///
/// # Data Protection
/// National IDs can precisely identify individuals and enable identity theft or fraud,
/// making them highly sensitive PII (Personal Identifiable Information).
///
/// As such, they are:
/// * masked in logs (via `Debug` implementation) to display
///   the first and last characters only,
/// * not exposed publicly except for a part of a request or response
///   via **unsafe** method `with_exposed_secret`.
#[derive(Clone, ZeroizeOnDrop)]
pub struct NationalId(String);

impl<'a> TryFrom<&'a str> for NationalId {
    type Error = Error;

    #[inline]
    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        Self::sanitize(input).validate()
    }
}

impl fmt::Debug for NationalId {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <Self as Masked>::masked_debug(self, f)
    }
}

impl AsUnsafeRef<str> for NationalId {
    #[inline]
    unsafe fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

// --- Sealed traits (not parts of the public API) ---

impl Sanitized for NationalId {
    #[inline]
    fn sanitize(input: &str) -> Self {
        let mut output = Self(String::with_capacity(input.len()));
        filter_characters(&mut output.0, input, "'.-_");
        output
    }
}

impl Validated for NationalId {
    #[inline]
    fn validate(self) -> Result<Self, Error> {
        self._validate_length(&self.0, 7, 18)?;
        self._validate_alphanumeric(&self.0, "")?;
        Ok(self)
    }
}

/// # Safety
/// The trait is safely implemented because exposing the first 1 and last 1 character:
/// 1. Neither causes out-of-bounds access to potentially INVALID (empty) data,
///    due to fallbacks to the empty strings,
/// 2. Nor leaks the essential part of the sensitive VALID data which has at least 7 chars.
unsafe impl Masked for NationalId {
    const TYPE_WRAPPER: &'static str = "NationalId";

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

    const VALID_ID: &str = "1234567890";
    const VALID_ID_ALPHA: &str = "ABC123XY";

    mod construction {
        use super::*;

        #[test]
        fn accepts_valid_ids() {
            for input in [VALID_ID, VALID_ID_ALPHA, "1234567", "123456789012345678"] {
                let result = NationalId::try_from(input);
                assert!(result.is_ok(), "{input:?} failed validation");
            }
        }

        #[test]
        fn removes_separators() {
            let input = " 123-456.789_0 \n\t\r ";
            let id = NationalId::try_from(input).unwrap();
            let result = unsafe { id.as_ref() };
            assert_eq!(result, VALID_ID);
        }

        #[test]
        fn rejects_too_short_id() {
            let input = "123456"; // 6 characters
            let result = NationalId::try_from(input);

            if let Err(Error::InvalidInput(msg)) = result {
                assert!(msg.contains("1***6"));
            } else {
                panic!("Expected InvalidInput error, got {result:?}");
            }
        }

        #[test]
        fn rejects_too_long_id() {
            let input = "1234567890123456789"; // 19 characters
            let result = NationalId::try_from(input);

            if let Err(Error::InvalidInput(msg)) = result {
                assert!(msg.contains("1***9"));
            } else {
                panic!("Expected InvalidInput error, got {result:?}");
            }
        }

        #[test]
        fn rejects_non_alphanumeric_characters() {
            let input = "1234567@";
            let result = NationalId::try_from(input);

            if let Err(Error::InvalidInput(msg)) = result {
                assert!(msg.contains("1***@"));
            } else {
                panic!("Expected InvalidInput error, got {result:?}");
            }
        }
    }

    mod safety {
        use super::*;

        #[test]
        fn masks_debug() {
            let id = NationalId::try_from(VALID_ID).unwrap();
            let debug_output = format!("{:?}", id);
            assert!(debug_output.contains(r#"NationalId("1***0")"#));
        }

        #[test]
        fn as_ref_is_unsafe() {
            static_assertions::assert_not_impl_all!(NationalId: AsRef<str>);

            let input = "123-456-7890";
            let id = NationalId::try_from(input).unwrap();
            let exposed = unsafe { <NationalId as AsUnsafeRef<str>>::as_ref(&id) };
            assert_eq!(exposed, VALID_ID);
        }

        #[test]
        fn memory_is_not_leaked_after_drop() {
            let ptr: *const u8;
            let len: usize;
            unsafe {
                let id = NationalId::try_from(VALID_ID).unwrap();
                let s = id.as_ref();
                ptr = s.as_ptr();
                len = s.len();
            }

            // SAFETY: This test verifies memory was zeroed after a drop.
            // Reading potentially freed memory is unsafe and only valid in tests
            // immediately after a drop, before any reallocation.
            unsafe {
                let slice = std::slice::from_raw_parts(ptr, len);
                let original_bytes = VALID_ID.as_bytes();
                assert_ne!(
                    slice, original_bytes,
                    "Original national ID should not remain in memory after drop"
                );
            }
        }
    }
}
