use std::convert::TryFrom;
use std::fmt;
use zeroize_derive::ZeroizeOnDrop;

use crate::Error;
use crate::internal::{AsUnsafeRef, Masked, Validated, sanitized::*};

/// Universal bank routing identifier
///
/// Used for ABA, Sort Code, BSB, IFSC, SWIFT/BIC, etc.
///
/// # Sanitization
/// * removes common separators: spaces, dashes, underscores and dots,
/// * removes all ASCII control characters like newlines, tabs, etc.
///
/// # Validation
/// * length: 6-11 characters,
/// * only alphanumeric characters are allowed
///
/// # Data Protection
/// While PCI DSS does NOT classify routing numbers as sensitive authentication data (SAD),
/// they identify specific bank branches and can be used
/// with account numbers for unauthorized transfers,
/// making them critical financial access data.
///
/// As such, they are:
/// * masked in logs (via `Debug` implementation) to display
///   the first and last characters (both in the upper case) only,
/// * not exposed publicly except for a part of a request or response
///   via **unsafe** method `with_exposed_secret`.
#[derive(Clone, ZeroizeOnDrop)]
pub struct RoutingNumber(String);

impl<'a> TryFrom<&'a str> for RoutingNumber {
    type Error = Error;

    #[inline]
    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        let original = Self::sanitize(input).validate()?;
        Ok(Self(original.0.to_uppercase()))
    }
}

impl fmt::Debug for RoutingNumber {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.masked_debug(f)
    }
}

impl AsUnsafeRef<str> for RoutingNumber {
    #[inline]
    unsafe fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

// --- Sealed traits (not parts of the public API) ---

impl Sanitized for RoutingNumber {
    #[inline]
    fn sanitize(input: &str) -> Self {
        let mut output = Self(String::with_capacity(input.len()));
        filter_characters(&mut output.0, input, "-_.");
        output
    }
}

impl Validated for RoutingNumber {
    #[inline]
    fn validate(self) -> Result<Self, Error> {
        self._validate_length(&self.0, 6, 11)?;
        self._validate_alphanumeric(&self.0, "")?;
        Ok(self)
    }
}

// SAFETY: The trait is safely implemented because exposing the first 1 and last 1 character:
// 1. Neither causes out-of-bounds access to potentially INVALID (empty) data,
//    due to fallbacks to the empty strings,
// 2. Nor leaks the essential part of the sensitive VALID data which has at least 6 chars.
unsafe impl Masked for RoutingNumber {
    const TYPE_WRAPPER: &'static str = "RoutingNumber";

    #[inline]
    fn first_chars(&self) -> String {
        self.0.get(0..1).unwrap_or_default().to_uppercase()
    }

    #[inline]
    fn last_chars(&self) -> String {
        let len = self.0.len();
        self.0.get(len - 1..len).unwrap_or_default().to_uppercase()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_ROUTING: &str = "123456789";
    const VALID_ROUTING_UPPER: &str = "123456789";

    mod construction {
        use super::*;

        #[test]
        fn accepts_valid_routing_numbers() {
            for input in [VALID_ROUTING, "ABCDEF", "123456", "12345678901"] {
                let result = RoutingNumber::try_from(input);
                assert!(result.is_ok(), "{input:?} failed validation");
            }
        }

        #[test]
        fn removes_separators_and_converts_to_uppercase() {
            let input = " 123-456.789 \n\t\r ";
            let routing = RoutingNumber::try_from(input).unwrap();
            let result = unsafe { routing.as_ref() };
            assert_eq!(result, VALID_ROUTING_UPPER);
        }

        #[test]
        fn rejects_too_short_routing() {
            let input = "12345"; // 5 characters
            let result = RoutingNumber::try_from(input);

            assert!(matches!(result, Err(Error::InvalidInput(_))));
        }

        #[test]
        fn rejects_too_long_routing() {
            let input = "123456789012"; // 12 characters
            let result = RoutingNumber::try_from(input);

            assert!(matches!(result, Err(Error::InvalidInput(_))));
        }

        #[test]
        fn rejects_non_alphanumeric_characters() {
            let input = "1234567@9";
            let result = RoutingNumber::try_from(input);

            assert!(matches!(result, Err(Error::InvalidInput(_))));
        }
    }

    mod safety {
        use super::*;

        #[test]
        fn masks_debug() {
            let routing = RoutingNumber::try_from(VALID_ROUTING).unwrap();
            let debug_output = format!("{:?}", routing);
            assert!(debug_output.contains(r#"RoutingNumber("1***9")"#));
        }

        #[test]
        fn as_ref_is_unsafe() {
            static_assertions::assert_not_impl_all!(RoutingNumber: AsRef<str>);

            let input = "123-456-789";
            let routing = RoutingNumber::try_from(input).unwrap();
            let exposed = unsafe { <RoutingNumber as AsUnsafeRef<str>>::as_ref(&routing) };
            assert_eq!(exposed, VALID_ROUTING);
        }

        #[test]
        fn memory_is_not_leaked_after_drop() {
            let ptr: *const u8;
            let len: usize;
            unsafe {
                let routing = RoutingNumber::try_from(VALID_ROUTING).unwrap();
                let s = routing.as_ref();
                ptr = s.as_ptr();
                len = s.len();
            }

            unsafe {
                let slice = std::slice::from_raw_parts(ptr, len);
                let original_bytes = VALID_ROUTING.as_bytes();
                assert_ne!(
                    slice, original_bytes,
                    "Original routing number should not remain in memory after drop"
                );
            }
        }
    }
}
