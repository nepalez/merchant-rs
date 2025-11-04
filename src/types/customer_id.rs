use std::fmt;
use zeroize_derive::ZeroizeOnDrop;

use crate::Error;
use crate::internal::{AsUnsafeRef, Masked, Validated, sanitized::*};

/// User identifier from an external vault or payment system
///
/// # Sanitization
/// * trims whitespaces,
/// * removes all ASCII control characters like newlines, tabs, etc.
///
/// # Validation
/// * length: 1-255 characters
///
/// # Data Protection
/// User IDs enable transaction correlation and user profiling,
/// and are considered PII (Personal Identifiable Information).
///
/// As such, they are:
/// * masked in logs (via `Debug` implementation) to display
///   the first 4 and the last 4 characters but not leaving less than 8 characters masked.
/// * not exposed publicly except for a part of a request or response
///   via **unsafe** method `with_exposed_secret`.
#[derive(Clone, ZeroizeOnDrop)]
pub struct CustomerId(String);

impl<'a> TryFrom<&'a str> for CustomerId {
    type Error = Error;

    #[inline]
    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        Self::sanitize(input).validate()
    }
}

impl fmt::Debug for CustomerId {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.masked_debug(f)
    }
}

impl AsUnsafeRef<str> for CustomerId {
    #[inline]
    unsafe fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

// --- Sealed traits (not parts of the public API) ---

impl Sanitized for CustomerId {
    #[inline]
    fn sanitize(input: &str) -> Self {
        let mut output = Self(String::with_capacity(input.len()));
        trim_whitespaces(&mut output.0, input);
        output
    }
}

impl Validated for CustomerId {
    #[inline]
    fn validate(self) -> Result<Self, Error> {
        self._validate_length(&self.0, 1, 255)?;
        Ok(self)
    }
}

// SAFETY: The trait is safely implemented because exposing the first 4 and last 4 characters
// (but not leaving less than 8 characters masked):
// 1. Neither causes out-of-bounds access to potentially INVALID (empty) data by itself,
// 2. Nor leaks the essential part of the sensitive VALID data.
unsafe impl Masked for CustomerId {
    const TYPE_WRAPPER: &'static str = "CustomerId";

    #[inline]
    fn first_chars(&self) -> String {
        let len = self.0.len().saturating_sub(8).saturating_div(2).min(4);
        self.0.get(0..len).unwrap_or_default().to_string()
    }

    #[inline]
    fn last_chars(&self) -> String {
        let len = self.0.len().saturating_sub(8).saturating_div(2).min(4);
        self.0.get(0..len).unwrap_or_default().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_ID_SHORT: &str = "abc123";
    const VALID_ID_LONG: &str = "customer_12345678901234567890";

    mod construction {
        use super::*;

        #[test]
        fn accepts_valid_ids() {
            for input in [VALID_ID_SHORT, VALID_ID_LONG, "1", "a".repeat(255).as_str()] {
                let result = CustomerId::try_from(input);
                assert!(result.is_ok(), "{input:?} failed validation");
            }
        }

        #[test]
        fn removes_control_characters() {
            let input = " abc123 \n\t\r ";
            let id = CustomerId::try_from(input).unwrap();
            let result = unsafe { id.as_ref() };
            assert_eq!(result, VALID_ID_SHORT);
        }

        #[test]
        fn rejects_empty_id() {
            let input = "";
            let result = CustomerId::try_from(input);

            if let Err(Error::InvalidInput(msg)) = result {
                assert!(msg.contains(r#"CustomerId("***")"#));
            } else {
                panic!("Expected InvalidInput error, got {result:?}");
            }
        }

        #[test]
        fn rejects_too_long_id() {
            let input = "a".repeat(256);
            let result = CustomerId::try_from(input.as_str());

            assert!(matches!(result, Err(Error::InvalidInput(_))));
        }
    }

    mod safety {
        use super::*;

        #[test]
        fn masks_debug_short() {
            let id = CustomerId::try_from(VALID_ID_SHORT).unwrap();
            let debug_output = format!("{:?}", id);
            // For short IDs, nothing should be exposed
            assert!(debug_output.contains(r#"CustomerId("***")"#));
        }

        #[test]
        fn masks_debug_long() {
            let id = CustomerId::try_from(VALID_ID_LONG).unwrap();
            let debug_output = format!("{:?}", id);
            // For long IDs, up to 4 chars from each end should be exposed
            assert!(debug_output.contains("cust"));
        }

        #[test]
        fn as_ref_is_unsafe() {
            static_assertions::assert_not_impl_all!(CustomerId: AsRef<str>);

            let input = " abc123 \n\t";
            let id = CustomerId::try_from(input).unwrap();
            let exposed = unsafe { <CustomerId as AsUnsafeRef<str>>::as_ref(&id) };
            assert_eq!(exposed, VALID_ID_SHORT);
        }

        #[test]
        fn memory_is_not_leaked_after_drop() {
            let ptr: *const u8;
            let len: usize;
            unsafe {
                let id = CustomerId::try_from(VALID_ID_SHORT).unwrap();
                let s = id.as_ref();
                ptr = s.as_ptr();
                len = s.len();
            }

            // SAFETY: This test verifies memory was zeroed after a drop.
            // Reading potentially freed memory is unsafe and only valid in tests
            // immediately after a drop, before any reallocation.
            unsafe {
                let slice = std::slice::from_raw_parts(ptr, len);
                let original_bytes = VALID_ID_SHORT.as_bytes();
                assert_ne!(
                    slice, original_bytes,
                    "Original customer ID should not remain in memory after drop"
                );
            }
        }
    }
}
