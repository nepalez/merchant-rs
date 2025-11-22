//! Offer identifier for promotional installment plans.

use crate::Error;
use crate::internal::{Validated, sanitized::*};

/// Offer identifier for promotional installment plans
///
/// Identifies a specific promotional offer for installment payments,
/// typically used for No Cost EMI programs where the merchant absorbs
/// the interest cost.
///
/// Common in India (Razorpay, PayU) where offers are created in the
/// merchant dashboard and referenced during payment authorization.
///
/// # Sanitization
/// * trims whitespaces,
/// * removes all ASCII control characters like newlines, tabs, etc.
///
/// # Validation
/// * length: 1-255 characters
///
/// # Data Protection
/// Offer IDs are public identifiers used for selecting promotional options.
/// They do not provide access to operations or sensitive data.
///
/// As such, they are:
/// * not masked in logs (via `Debug` implementation)
/// * exposed as regular public data via `AsRef<str>`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OfferId(String);

impl<'a> TryFrom<&'a str> for OfferId {
    type Error = Error;

    #[inline]
    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        Self::validate(Self::sanitize(input))
    }
}

impl AsRef<str> for OfferId {
    #[inline]
    fn as_ref(&self) -> &str {
        &self.0
    }
}

// --- Sealed traits (not parts of the public API) ---

impl Sanitized for OfferId {
    #[inline]
    fn sanitize(input: &str) -> Self {
        let mut output = Self(String::with_capacity(input.len()));
        trim_whitespaces(&mut output.0, input);
        output
    }
}

impl Validated for OfferId {
    #[inline]
    fn validate(self) -> Result<Self, Error> {
        self._validate_length(&self.0, 1, 255)?;
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_ID: &str = "OFFER_EMI_6M";

    mod construction {
        use super::*;

        #[test]
        fn accepts_valid_ids() {
            for input in [VALID_ID, "1", "offer_123", "a".repeat(255).as_str()] {
                let result = OfferId::try_from(input);
                assert!(result.is_ok(), "{input:?} failed validation");
            }
        }

        #[test]
        fn sanitises() {
            let input = " OFFER_EMI_6M \n\t\r ";
            let id = OfferId::try_from(input).unwrap();
            let result = id.as_ref();
            assert_eq!(result, VALID_ID);
        }

        #[test]
        fn rejects_empty_id() {
            let input = "";
            let result = OfferId::try_from(input);
            assert!(matches!(result, Err(Error::InvalidInput(_))));
        }

        #[test]
        fn rejects_too_long_id() {
            let input = "a".repeat(256);
            let result = OfferId::try_from(input.as_str());
            assert!(matches!(result, Err(Error::InvalidInput(_))));
        }
    }
}
