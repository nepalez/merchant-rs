use std::convert::TryFrom;
use std::fmt;
use zeroize_derive::ZeroizeOnDrop;

use crate::internal::{Masked, Validated, sanitized::*};
use crate::{AsUnsafeRef, Error};

/// Personal phone number
///
/// # Sanitization
/// * removes all non-digit characters
/// * adds the leading `+` sign
///
/// # Validation
/// * length: 6-16 characters (including the `+` sign)
///
/// # Data Protection
/// Phone numbers enable contact with their holders
/// and SMS-based authentication, making them sensitive PII
/// (Personal Identifiable Information).
///
/// As such, they are:
/// * masked in logs (via `Debug` implementation) to display
///   the `+` sign and the last 2 digits only.
/// * not exposed publicly except for a part of a request or response
///   via **unsafe** method `with_exposed_secret`.
#[derive(Clone, ZeroizeOnDrop)]
pub struct PhoneNumber(String);

impl<'a> TryFrom<&'a str> for PhoneNumber {
    type Error = Error;

    #[inline]
    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        Self::sanitize(input).validate()
    }
}

impl fmt::Debug for PhoneNumber {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <Self as Masked>::masked_debug(self, f)
    }
}

impl AsUnsafeRef<str> for PhoneNumber {
    #[inline]
    unsafe fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

// --- Sealed traits (not parts of the public API) ---

impl Sanitized for PhoneNumber {
    #[inline]
    fn sanitize(input: &str) -> Self {
        let mut output = Self(String::with_capacity(input.len() + 1));
        output.0.push('+');
        for c in input.chars() {
            if c.is_ascii_digit() {
                output.0.push(c);
            }
        }
        output
    }
}

impl Validated for PhoneNumber {
    #[inline]
    fn validate(self) -> Result<Self, Error> {
        self._validate_length(&self.0, 6, 16)?;
        Ok(self)
    }
}

/// # Safety
/// The trait is safely implemented because exposing the `+` sign
/// along with the last 2 characters:
/// 1. Neither causes out-of-bounds access to potentially INVALID (empty) data,
///    due to fallbacks to the empty strings,
/// 2. Nor leaks the sensitive VALID data (which has at least 5 digits) in total
///    and does not expose its actual length (this could be done by the first digits
///    which aren't exposed anyway).
unsafe impl Masked for PhoneNumber {
    const TYPE_WRAPPER: &'static str = "PhoneNumber";

    #[inline]
    fn first_chars(&self) -> String {
        self.0.get(0..1).unwrap_or_default().to_string()
    }

    #[inline]
    fn last_chars(&self) -> String {
        self.0
            .get(self.0.len().saturating_sub(2)..)
            .unwrap_or_default()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_PHONE: &str = "+1234567890";

    mod construction {
        use super::*;

        #[test]
        fn accepts_valid_phones() {
            for input in ["1234567890", "+1234567890", "12345", "123456789012345"] {
                let result = PhoneNumber::try_from(input);
                assert!(result.is_ok(), "{input:?} failed validation");
            }
        }

        #[test]
        fn removes_non_digits_and_adds_plus() {
            let input = " (123) 456-7890 \n\t\r ";
            let phone = PhoneNumber::try_from(input).unwrap();
            let result = unsafe { phone.as_ref() };
            assert_eq!(result, VALID_PHONE);
        }

        #[test]
        fn rejects_too_short_phone() {
            let input = "1234"; // 4 digits -> +1234 (5 characters)
            let result = PhoneNumber::try_from(input);

            if let Err(Error::InvalidInput(msg)) = result {
                assert!(msg.contains(r#"PhoneNumber("+***34")"#));
            } else {
                panic!("Expected InvalidInput error, got {result:?}");
            }
        }

        #[test]
        fn rejects_too_long_phone() {
            let input = "12345678901234567"; // 17 digits
            let result = PhoneNumber::try_from(input);

            assert!(matches!(result, Err(Error::InvalidInput(_))));
        }

        #[test]
        fn rejects_empty_phone() {
            let input = "";
            let result = PhoneNumber::try_from(input);

            // Empty input becomes "+" which has length 1, less than 6
            if let Err(Error::InvalidInput(msg)) = result {
                // PhoneNumber will mask as "+***"
                // since last_chars returns an empty string for length 1
                assert!(msg.contains("PhoneNumber"));
            } else {
                panic!("Expected InvalidInput error, got {result:?}");
            }
        }
    }

    mod safety {
        use super::*;

        #[test]
        fn masks_debug() {
            let phone = PhoneNumber::try_from(VALID_PHONE.trim_start_matches('+')).unwrap();
            let debug_output = format!("{:?}", phone);
            assert!(debug_output.contains(r#"PhoneNumber("+***90")"#));
        }

        #[test]
        fn as_ref_is_unsafe() {
            static_assertions::assert_not_impl_all!(PhoneNumber: AsRef<str>);

            let input = "(123) 456-7890";
            let phone = PhoneNumber::try_from(input).unwrap();
            let exposed = unsafe { <PhoneNumber as AsUnsafeRef<str>>::as_ref(&phone) };
            assert_eq!(exposed, VALID_PHONE);
        }

        #[test]
        fn memory_is_not_leaked_after_drop() {
            let ptr: *const u8;
            let len: usize;
            unsafe {
                let phone = PhoneNumber::try_from(VALID_PHONE.trim_start_matches('+')).unwrap();
                let s = phone.as_ref();
                ptr = s.as_ptr();
                len = s.len();
            }

            // SAFETY: This test verifies memory was zeroed after a drop.
            // Reading potentially freed memory is unsafe and only valid in tests
            // immediately after a drop, before any reallocation.
            unsafe {
                let slice = std::slice::from_raw_parts(ptr, len);
                let original_bytes = VALID_PHONE.as_bytes();
                assert_ne!(
                    slice, original_bytes,
                    "Original phone number should not remain in memory after drop"
                );
            }
        }
    }
}
