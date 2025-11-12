use crate::Error;
use crate::internal::{Validated, sanitized::*};

/// Recipient account identifier for split payments
///
/// Identifies a connected account or sub-merchant in the payment gateway's system
/// that will receive funds in a split payment transaction.
///
/// # Sanitization
/// * trims whitespaces,
/// * removes all ASCII control characters like newlines, tabs, etc.
///
/// # Validation
/// * length: 1-255 characters
///
/// # Data Protection
/// Recipient IDs are public identifiers used for routing funds. They do not
/// provide access to operations or sensitive data, similar to routing numbers.
///
/// As such, they are:
/// * not masked in logs (via `Debug` implementation)
/// * exposed as regular public data via `AsRef<str>`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RecipientId(String);

impl<'a> TryFrom<&'a str> for RecipientId {
    type Error = Error;

    #[inline]
    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        Self::validate(Self::sanitize(input))
    }
}

impl AsRef<str> for RecipientId {
    #[inline]
    fn as_ref(&self) -> &str {
        &self.0
    }
}

// --- Sealed traits (not parts of the public API) ---

impl Sanitized for RecipientId {
    #[inline]
    fn sanitize(input: &str) -> Self {
        let mut output = Self(String::with_capacity(input.len()));
        trim_whitespaces(&mut output.0, input);
        output
    }
}

impl Validated for RecipientId {
    #[inline]
    fn validate(self) -> Result<Self, Error> {
        self._validate_length(&self.0, 1, 255)?;
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_ID_SHORT: &str = "acct_123";
    const VALID_ID_LONG: &str = "recipient_12345678901234567890";

    mod construction {
        use super::*;

        #[test]
        fn accepts_valid_ids() {
            for input in [VALID_ID_SHORT, VALID_ID_LONG, "1", "a".repeat(255).as_str()] {
                let result = RecipientId::try_from(input);
                assert!(result.is_ok(), "{input:?} failed validation");
            }
        }

        #[test]
        fn sanitises() {
            let input = " acct_123 \n\t\r ";
            let id = RecipientId::try_from(input).unwrap();
            let result = id.as_ref();
            assert_eq!(result, VALID_ID_SHORT);
        }

        #[test]
        fn rejects_empty_id() {
            let input = "";
            let result = RecipientId::try_from(input);
            assert!(matches!(result, Err(Error::InvalidInput(_))));
        }

        #[test]
        fn rejects_too_long_id() {
            let input = "a".repeat(256);
            let result = RecipientId::try_from(input.as_str());
            assert!(matches!(result, Err(Error::InvalidInput(_))));
        }
    }
}
