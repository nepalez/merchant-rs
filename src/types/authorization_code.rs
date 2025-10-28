use std::fmt;
use zeroize_derive::ZeroizeOnDrop;

use crate::Error;
use crate::internal::{Masked, Validated, sanitized::*};

/// Authorization code from a card issuer
///
/// Supports both ISO 8583 standard (6 numeric digits)
/// and extended formats used by legacy/regional processors (e.g., older European
/// acquirers, some APAC processors use up to 8-10 characters).
///
/// # Sanitization
/// * removes common separators: spaces and dashes,
/// * removes all ASCII control characters like newlines, tabs, etc.
///
/// # Validation
/// * length: 6-10 characters,
/// * only alphanumeric characters are allowed
///
/// Gateway-specific validators should enforce stricter rules if necessary.
///
/// # Data Protection
/// While authorization codes are not Sensitive Authentication Data per PCI DSS,
/// they represent operational sensitive data. Defense-in-depth approach prevents
/// potential replay attacks in legacy systems and accidental exposure in logs.
///
/// As such, they are:
/// * masked in logs (via `Debug` implementation) to display
///   1 first and 1 last characters (both uppercased) only,
/// * not exposed publicly except for a part of a request or response
///   via **unsafe** method `with_exposed_secret`.
#[derive(Clone, ZeroizeOnDrop)]
pub struct AuthorizationCode(String);

impl<'a> TryFrom<&'a str> for AuthorizationCode {
    type Error = Error;

    #[inline]
    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        Self::sanitize(input).validate()
    }
}

impl fmt::Debug for AuthorizationCode {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.masked_debug(f)
    }
}

impl AsRef<str> for AuthorizationCode {
    #[inline]
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

// --- Sealed traits (not parts of the public API) ---

impl Sanitized for AuthorizationCode {
    #[inline]
    fn sanitize(input: &str) -> Self {
        let mut output = Self(String::with_capacity(input.len()));
        trim_whitespaces(&mut output.0, input);
        output
    }
}

impl Validated for AuthorizationCode {
    fn validate(self) -> Result<Self, Error> {
        self._validate_length(&self.0, 6, 10)?;
        self._validate_alphanumeric(&self.0, "")?;
        Ok(self)
    }
}

// SAFETY: The trait is safely implemented because exposing the first 1 and last 1 character:
// 1. Neither causes out-of-bounds access to potentially INVALID (empty) data,
//    due to fallbacks to the empty strings,
// 2. Nor leaks the essential part of the sensitive VALID data
//    due to hiding the real length of the name.
unsafe impl Masked for AuthorizationCode {
    const TYPE_WRAPPER: &'static str = "AuthorizationCode";

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
