use std::convert::TryFrom;
use std::fmt;
use zeroize_derive::ZeroizeOnDrop;

use crate::internal::{Masked, Validated, sanitized::*};
use crate::{AsUnsafeRef, Error};

/// Cardholder name as it appears on a payment card
///
/// # Sanitization
/// * trims whitespaces,
/// * removes all ASCII control characters like newlines, tabs, etc.
///
/// # Validation
/// * length: 3-26 characters (EMV and ISO/IEC 7813 standard),
/// * only ASCII alphabetic characters, spaces, dashes, apostrophes and dots are allowed,
/// * any non-Latin character (e.g., Cyrillic, Chinese) fails validation
///
/// # Data Protection
/// While PCI DSS does NOT classify cardholder names as sensitive authentication data (SAD),
/// they are critical PII and financial access data that can be associated with their owners.
///
/// As such, they are:
/// * masked in logs (via `Debug` implementation) to display
///   the first and last characters (both in the upper case) only,
///   which prevents leaking short names,
/// * not exposed publicly except for a part of a request or response
///   via **unsafe** method `with_exposed_secret`.
#[derive(Clone, ZeroizeOnDrop)]
pub struct CardHolderName(String);

impl<'a> TryFrom<&'a str> for CardHolderName {
    type Error = Error;

    #[inline]
    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        let original = Self::sanitize(input).validate()?;
        Ok(Self(original.0.to_uppercase()))
    }
}

impl fmt::Debug for CardHolderName {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <Self as Masked>::masked_debug(self, f)
    }
}

impl AsUnsafeRef<str> for CardHolderName {
    #[inline]
    unsafe fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

// --- Sealed traits (not parts of the public API) ---

impl Sanitized for CardHolderName {
    #[inline]
    fn sanitize(input: &str) -> Self {
        let mut output = Self(String::with_capacity(input.len()));
        trim_whitespaces(&mut output.0, input);
        output
    }
}

impl Validated for CardHolderName {
    #[inline]
    fn validate(self) -> Result<Self, Error> {
        self._validate_length(&self.0, 3, 26)?;
        self._validate_alphabetic(&self.0, " -'.")?;
        Ok(self)
    }
}

/// # Safety
/// The trait is safely implemented because exposing the first 1 and last 1 character:
/// 1. Neither causes out-of-bounds access to potentially INVALID (empty) data,
///    due to fallbacks to the empty strings,
/// 2. Nor leaks the essential part of the sensitive VALID data
///    due to hiding the real length of the name.
unsafe impl Masked for CardHolderName {
    const TYPE_WRAPPER: &'static str = "CardHolderName";

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

    mod construction {
        use super::*;

        #[test]
        fn accepts_valid_names() {
            for input in [VALID_NAME, "Ann", "Mary-Jane Smith", "O'Brien", "Dr. House"] {
                let result = CardHolderName::try_from(input);
                assert!(result.is_ok(), "{input:?} failed validation");
            }
        }

        #[test]
        fn removes_control_characters_and_converts_to_uppercase() {
            let input = " john doe \n\t\r ";
            let name = CardHolderName::try_from(input).unwrap();
            let result = unsafe { name.as_ref() };
            assert_eq!(result, VALID_NAME_UPPER);
        }

        #[test]
        fn rejects_too_short_name() {
            let input = "ab"; // 2 characters (lowercase will be converted to uppercase AB)
            let result = CardHolderName::try_from(input);

            if let Err(Error::InvalidInput(msg)) = result {
                // The message will contain uppercase version A***B because sanitization happens before validation
                assert!(msg.contains("***"));
            } else {
                panic!("Expected InvalidInput error, got {result:?}");
            }
        }

        #[test]
        fn rejects_too_long_name() {
            let input = "Abcdefghijklmnopqrstuvwxyz"; // 26 characters
            let result = CardHolderName::try_from(input).unwrap(); // Valid length
            assert!(unsafe { result.as_ref() }.len() == 26);

            let input = "abcdefghijklmnopqrstuvwxyzz"; // 27 characters (lowercase)
            let result = CardHolderName::try_from(input);

            if let Err(Error::InvalidInput(msg)) = result {
                // Will contain uppercase version
                assert!(msg.contains("***"));
            } else {
                panic!("Expected InvalidInput error, got {result:?}");
            }
        }

        #[test]
        fn rejects_non_alphabetic_characters() {
            let input = "John Doe123";
            let result = CardHolderName::try_from(input);

            if let Err(Error::InvalidInput(msg)) = result {
                assert!(msg.contains("J***3"));
            } else {
                panic!("Expected InvalidInput error, got {result:?}");
            }
        }

        #[test]
        fn rejects_non_latin_characters() {
            let input = "Иван Иванов";
            let result = CardHolderName::try_from(input);

            assert!(matches!(result, Err(Error::InvalidInput(_))));
        }
    }

    mod safety {
        use super::*;

        #[test]
        fn masks_debug() {
            let name = CardHolderName::try_from(VALID_NAME).unwrap();
            let debug_output = format!("{:?}", name);
            assert!(debug_output.contains(r#"CardHolderName("J***E")"#));
        }

        #[test]
        fn as_ref_is_unsafe() {
            static_assertions::assert_not_impl_all!(CardHolderName: AsRef<str>);

            let input = " john doe \n\t";
            let name = CardHolderName::try_from(input).unwrap();
            let exposed = unsafe { <CardHolderName as AsUnsafeRef<str>>::as_ref(&name) };
            assert_eq!(exposed, VALID_NAME_UPPER);
        }

        #[test]
        fn memory_is_not_leaked_after_drop() {
            let ptr: *const u8;
            let len: usize;
            unsafe {
                let name = CardHolderName::try_from(VALID_NAME).unwrap();
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
                    "Original cardholder name should not remain in memory after drop"
                );
            }
        }
    }
}
