use std::convert::TryFrom;
use std::fmt;
use zeroize_derive::ZeroizeOnDrop;

use crate::Error;
use crate::internal::{AsUnsafeRef, Masked, Validated, sanitized::*};

/// External transaction identifier from a payment gateway
///
/// # Sanitization
/// * trims whitespaces,
/// * removes all ASCII control characters like newlines, tabs, etc.
///
/// # Validation
/// * length: 8-255 characters,
/// * only alphanumeric characters, dashes and underscores are allowed
///
/// # Data Protection
/// While neither PII nor classified as sensitive by PCI DSS, transaction identifiers
/// can be used to initiate operations (void, capture, refund) and access transaction details,
/// requiring access control at the highest level.
///
/// As such, they are:
/// * masked in logs (via `Debug` implementation) to display
///   the first and last characters (both in the upper case) only,
/// * not exposed publicly except for a part of a request or response
///   via **unsafe** method `with_exposed_secret`.
#[derive(Clone, ZeroizeOnDrop)]
pub struct TransactionId(String);

impl<'a> TryFrom<&'a str> for TransactionId {
    type Error = Error;

    #[inline]
    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        Self::sanitize(input).validate()
    }
}

impl fmt::Debug for TransactionId {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.masked_debug(f)
    }
}

impl AsUnsafeRef<str> for TransactionId {
    #[inline]
    unsafe fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

// --- Sealed traits (not parts of the public API) ---

impl Sanitized for TransactionId {
    #[inline]
    fn sanitize(input: &str) -> Self {
        let mut output = Self(String::with_capacity(input.len()));
        trim_whitespaces(&mut output.0, input);
        output
    }
}

impl Validated for TransactionId {
    #[inline]
    fn validate(self) -> Result<Self, Error> {
        self._validate_length(&self.0, 8, 255)?;
        self._validate_alphanumeric(&self.0, "-_")?;
        Ok(self)
    }
}

// SAFETY: The trait is safely implemented because exposing the first 1 and last 1 character:
// 1. Neither causes out-of-bounds access to potentially INVALID (empty) data,
//    due to fallbacks to the empty strings,
// 2. Nor leaks the essential part of the sensitive VALID data which has at least 8 chars,
//    while also hiding the real length and case of the authorization ID.
unsafe impl Masked for TransactionId {
    const TYPE_WRAPPER: &'static str = "TransactionId";

    #[inline]
    fn first_chars(&self) -> String {
        self.0.get(0..1).unwrap_or_default().to_uppercase()
    }

    #[inline]
    fn last_chars(&self) -> String {
        self.0
            .get(self.0.len() - 1..)
            .unwrap_or_default()
            .to_uppercase()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_ID: &str = "txn_12345678";

    mod construction {
        use super::*;

        #[test]
        fn accepts_valid_transaction_ids() {
            for input in [VALID_ID, "12345678", "a".repeat(255).as_str()] {
                let result = TransactionId::try_from(input);
                assert!(result.is_ok(), "{input:?} failed validation");
            }
        }

        #[test]
        fn removes_control_characters() {
            let input = " txn_12345678 \n\t\r ";
            let id = TransactionId::try_from(input).unwrap();
            let result = unsafe { id.as_ref() };
            assert_eq!(result, VALID_ID);
        }

        #[test]
        fn rejects_too_short_id() {
            let result = TransactionId::try_from("1234567");
            assert!(matches!(result, Err(Error::InvalidInput(_))));
        }

        #[test]
        fn rejects_too_long_id() {
            let input = "a".repeat(256);
            let result = TransactionId::try_from(input.as_str());
            assert!(matches!(result, Err(Error::InvalidInput(_))));
        }
    }

    mod safety {
        use super::*;

        #[test]
        fn masks_debug() {
            let id = TransactionId::try_from(VALID_ID).unwrap();
            let debug_output = format!("{:?}", id);
            assert!(debug_output.contains(r#"TransactionId("T***8")"#));
        }

        #[test]
        fn memory_is_not_leaked_after_drop() {
            let ptr: *const u8;
            let len: usize;
            unsafe {
                let id = TransactionId::try_from(VALID_ID).unwrap();
                let s = id.as_ref();
                ptr = s.as_ptr();
                len = s.len();
            }

            unsafe {
                let slice = std::slice::from_raw_parts(ptr, len);
                let original_bytes = VALID_ID.as_bytes();
                assert_ne!(
                    slice, original_bytes,
                    "Original transaction ID should not remain in memory after drop"
                );
            }
        }
    }
}
