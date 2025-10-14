use std::convert::TryFrom;
use std::fmt;

use crate::error::*;
use crate::internal::*;

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

// SAFETY
//
// The trait is safely implemented because:
// 1. The wrapper uses a string as inner type because this code is not a PII.
// 2. Exposition of the uppercased first and last characters won't leak the code in total.
// 3. The validation rules guarantee that the inner data has at least 6 characters.
unsafe impl SafeWrapper for AuthorizationCode {
    type Inner = String;

    const FIRST_CHARS: usize = 1;
    const LAST_CHARS: usize = 1;

    #[inline]
    fn wrap(inner: Self::Inner) -> Self {
        Self(inner)
    }

    #[inline]
    unsafe fn inner(&self) -> &Self::Inner {
        &self.0
    }
}

impl Sanitized for AuthorizationCode {
    const CHARS_TO_REMOVE: Option<&'static str> = Some("-");
}

impl Validated for AuthorizationCode {
    const TYPE_NAME: &'static str = "AuthorizationCode";
    const MIN_LENGTH: usize = 6;
    const MAX_LENGTH: usize = 10;
    // Strictly alphanumeric (letters and digits only)
    const EXTRA_CHARS: Option<&'static str> = Some("");
}

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
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.masked_debug(f)
    }
}
