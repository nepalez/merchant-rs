use std::convert::TryFrom;
use std::fmt;
use zeroize_derive::ZeroizeOnDrop;

use crate::internal::{Masked, Validated, sanitized::*};
use crate::{AsUnsafeRef, Error};

/// External subscription identifier from a payment gateway
///
/// # Sanitization
/// * trims whitespaces,
/// * removes all ASCII control characters like newlines, tabs, etc.
///
/// # Validation
/// * length: 4-255 characters,
/// * only alphanumeric characters, dashes and underscores are allowed
///
/// # Coverage
/// Supports 100+ payment systems worldwide:
/// * Global: Stripe `sub_1MowQVLkdIwHu7ixeRlqHVzs`, PayPal `I-8XRLDA4MNEW3`, Square `8151fc89-da15-4eb9-a685-1a70883cebfc`
/// * Europe: Mollie `sub_rVKGtNd6s3`, GoCardless `SB00003GKMHFFY`
/// * Asia: Razorpay `sub_00000000000001`, Omise `schd_test_no1t4tnemucod0e51mo`
/// * Latin America: MercadoPago `2c938084726fca480172750000000000`
/// * Africa: Paystack `SUB_6phdx225bavuwtb`, Flutterwave `4147`
///
/// # Data Protection
/// Subscription identifiers can be used to manage subscriptions (cancel, update, pause)
/// and access subscription details, requiring access control at the highest level.
///
/// As such, they are:
/// * masked in logs (via `Debug` implementation) to display
///   the first and last characters (both in the upper case) only,
/// * not exposed publicly except for a part of a request or response
///   via **unsafe** method `as_ref`.
#[derive(Clone, PartialEq, Eq, ZeroizeOnDrop)]
pub struct SubscriptionId(String);

impl<'a> TryFrom<&'a str> for SubscriptionId {
    type Error = Error;

    #[inline]
    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        Self::sanitize(input).validate()
    }
}

impl fmt::Debug for SubscriptionId {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.masked_debug(f)
    }
}

impl AsUnsafeRef<str> for SubscriptionId {
    #[inline]
    unsafe fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

// --- Sealed traits (not parts of the public API) ---

impl Sanitized for SubscriptionId {
    #[inline]
    fn sanitize(input: &str) -> Self {
        let mut output = Self(String::with_capacity(input.len()));
        trim_whitespaces(&mut output.0, input);
        output
    }
}

impl Validated for SubscriptionId {
    #[inline]
    fn validate(self) -> Result<Self, Error> {
        self._validate_length(&self.0, 4, 255)?;
        self._validate_alphanumeric(&self.0, "-_")?;
        Ok(self)
    }
}

// SAFETY: The trait is safely implemented because exposing the first 1 and last 1 character:
// 1. Neither causes out-of-bounds access to potentially INVALID (empty) data,
//    due to fallbacks to the empty strings,
// 2. Nor leaks the essential part of the sensitive VALID data which has at least 4 chars,
//    while also hiding the real length and case of the subscription ID.
unsafe impl Masked for SubscriptionId {
    const TYPE_WRAPPER: &'static str = "SubscriptionId";

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

    mod construction {
        use super::*;

        #[test]
        fn accepts_valid_formats() {
            let formats = [
                ("sub_1MowQVLkdIwHu7ixeRlqHVzs", "Stripe"),
                ("I-8XRLDA4MNEW3", "PayPal"),
                ("8151fc89-da15-4eb9-a685-1a70883cebfc", "Square UUID"),
                ("sub_rVKGtNd6s3", "Mollie"),
                ("SB00003GKMHFFY", "GoCardless"),
                ("sub_00000000000001", "Razorpay"),
                ("schd_test_no1t4tnemucod0e51mo", "Omise"),
                ("2c938084726fca480172750000000000", "MercadoPago"),
                ("SUB_6phdx225bavuwtb", "Paystack"),
                ("4147", "Flutterwave"),
                ("1234", "min length"),
                (&"a".repeat(255), "max length"),
            ];

            for (input, name) in formats {
                let result = SubscriptionId::try_from(input);
                assert!(result.is_ok(), "{name} format failed: {input}");
            }
        }

        #[test]
        fn removes_control_characters() {
            let input = " sub_12345678 \n\t\r ";
            let id = SubscriptionId::try_from(input).unwrap();
            let result = unsafe { id.as_ref() };
            assert_eq!(result, "sub_12345678");
        }

        #[test]
        fn rejects_too_short_id() {
            let result = SubscriptionId::try_from("123");
            assert!(matches!(result, Err(Error::InvalidInput(_))));
        }

        #[test]
        fn rejects_too_long_id() {
            let input = "a".repeat(256);
            let result = SubscriptionId::try_from(input.as_str());
            assert!(matches!(result, Err(Error::InvalidInput(_))));
        }
    }

    mod safety {
        use super::*;

        #[test]
        fn masks_debug() {
            let id = SubscriptionId::try_from("sub_12345678").unwrap();
            let debug_output = format!("{:?}", id);
            assert!(debug_output.contains(r#"SubscriptionId("S***8")"#));
        }

        #[test]
        fn memory_is_not_leaked_after_drop() {
            let ptr: *const u8;
            let len: usize;
            unsafe {
                let id = SubscriptionId::try_from("sub_12345678").unwrap();
                let s = id.as_ref();
                ptr = s.as_ptr();
                len = s.len();
            }

            unsafe {
                let slice = std::slice::from_raw_parts(ptr, len);
                let original_bytes = "sub_12345678".as_bytes();
                assert_ne!(
                    slice, original_bytes,
                    "Original subscription ID should not remain in memory after drop"
                );
            }
        }
    }
}
