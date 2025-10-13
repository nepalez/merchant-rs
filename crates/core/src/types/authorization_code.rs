use std::convert::TryFrom;
use std::fmt;

use crate::error::*;
use crate::internal::*;

const TYPE_NAME: &str = "Authorization code";
const DEBUG_MASK: &str = "***";

/// Authorization code from card issuer.
///
/// # Input Constraints
/// Length 6-10, alphanumeric: Supports both ISO 8583 standard (6 numeric digits)
/// and extended formats used by legacy/regional processors (e.g., older European
/// acquirers, some APAC processors use up to 8-10 characters).
/// Gateway-specific validators MUST enforce stricter rules where required:
/// - ISO 8583 compliant systems: exactly 6 numeric digits
/// - Authorize.Net and similar: exactly 6 alphanumeric characters
/// - Regional processors: may use 7-10 characters
///
/// Sanitization: Aggressive removal of common separators (' ', '-') often used
/// in user input (e.g., "123-456"), as they are guaranteed non-data.
///
/// # Security
/// Debug implementation masks all characters except first and last, both uppercased.
/// While authorization codes are not Sensitive Authentication Data per PCI DSS,
/// they represent operational sensitive data. Defense-in-depth approach prevents
/// potential replay attacks in legacy systems and accidental exposure in logs.
#[derive(Clone)]
pub struct AuthorizationCode(String);

impl TryFrom<String> for AuthorizationCode {
    type Error = Error;

    #[inline]
    fn try_from(input: String) -> Result<Self> {
        Self::try_from_string(input)
    }
}

// The first+last approach prevents length disclosure and provides debugging context.
// Multiple codes collapse to the same mask (e.g., "ABC123" and "A12BC3" both become
// "A***3"), preventing code reconstruction.
impl fmt::Debug for AuthorizationCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let first_char = self.0.chars().next().unwrap();
        let last_char = self.0.chars().next_back().unwrap();
        let masked = format!(
            "{}{DEBUG_MASK}{}",
            first_char.to_uppercase(),
            last_char.to_uppercase(),
        );
        f.debug_tuple("AuthorizationCode").field(&masked).finish()
    }
}

// Sealed traits implementations

impl Sanitized for AuthorizationCode {
    const CHARS_TO_REMOVE: Option<&'static str> = Some("-");
}

impl Validated for AuthorizationCode {
    const TYPE_NAME: &'static str = "Authorization code";
    const MIN_LENGTH: usize = 6;
    const MAX_LENGTH: usize = 10;
    // Strictly alphanumeric (letters and digits only)
    const EXTRA_CHARS: Option<&'static str> = Some("");
}

impl SafeWrapper for AuthorizationCode {
    type Inner = String;

    fn wrap(inner: String) -> Self {
        Self(inner)
    }
}
