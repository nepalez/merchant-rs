//! Installment plan identifier from gateway API.

use crate::Error;
use crate::internal::{Validated, sanitized::*};

/// Installment plan identifier from gateway API
///
/// Identifies a specific installment plan returned by the gateway's
/// installments API (e.g., dLocal's "INS54434", EBANX plan IDs).
///
/// # Sanitization
/// * trims whitespaces,
/// * removes all ASCII control characters like newlines, tabs, etc.
///
/// # Validation
/// * length: 1-255 characters
///
/// # Data Protection
/// Plan IDs are public identifiers used for selecting installment options.
/// They do not provide access to operations or sensitive data.
///
/// As such, they are:
/// * not masked in logs (via `Debug` implementation)
/// * exposed as regular public data via `AsRef<str>`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstallmentPlanId(String);

impl<'a> TryFrom<&'a str> for InstallmentPlanId {
    type Error = Error;

    #[inline]
    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        Self::validate(Self::sanitize(input))
    }
}

impl AsRef<str> for InstallmentPlanId {
    #[inline]
    fn as_ref(&self) -> &str {
        &self.0
    }
}

// --- Sealed traits (not parts of the public API) ---

impl Sanitized for InstallmentPlanId {
    #[inline]
    fn sanitize(input: &str) -> Self {
        let mut output = Self(String::with_capacity(input.len()));
        trim_whitespaces(&mut output.0, input);
        output
    }
}

impl Validated for InstallmentPlanId {
    #[inline]
    fn validate(self) -> Result<Self, Error> {
        self._validate_length(&self.0, 1, 255)?;
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_ID: &str = "INS54434";

    mod construction {
        use super::*;

        #[test]
        fn accepts_valid_ids() {
            for input in [VALID_ID, "1", "plan_123", "a".repeat(255).as_str()] {
                let result = InstallmentPlanId::try_from(input);
                assert!(result.is_ok(), "{input:?} failed validation");
            }
        }

        #[test]
        fn sanitises() {
            let input = " INS54434 \n\t\r ";
            let id = InstallmentPlanId::try_from(input).unwrap();
            let result = id.as_ref();
            assert_eq!(result, VALID_ID);
        }

        #[test]
        fn rejects_empty_id() {
            let input = "";
            let result = InstallmentPlanId::try_from(input);
            assert!(matches!(result, Err(Error::InvalidInput(_))));
        }

        #[test]
        fn rejects_too_long_id() {
            let input = "a".repeat(256);
            let result = InstallmentPlanId::try_from(input.as_str());
            assert!(matches!(result, Err(Error::InvalidInput(_))));
        }
    }
}
