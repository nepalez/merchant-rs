use std::fmt;
use std::str::FromStr;
use zeroize_derive::ZeroizeOnDrop;

use crate::Error;
use crate::internal::{AsUnsafeRef, Masked, Validated, sanitized::*};

/// Email address
///
/// # Sanitization
/// * trims whitespaces,
/// * removes all ASCII control characters like newlines, tabs, etc.
///
/// # Validation
/// * validates email address format correctness via the [email-address](https://crates.io/crates/email-address) crate
///
/// # Data Protection
/// Email addresses enable communication with users
/// and can be used for account takeover or phishing attacks,
/// making them PII (Personal Identifiable Information).
///
/// As such, they are:
/// * masked in logs (via `Debug` implementation) to display
///   the first character along with the domain name,
/// * not exposed publicly except for a part of a request or response
///   via **unsafe** method `with_exposed_secret`.
#[derive(Clone, ZeroizeOnDrop)]
pub struct EmailAddress(String);

impl<'a> TryFrom<&'a str> for EmailAddress {
    type Error = Error;

    #[inline]
    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        Self::sanitize(input).validate()
    }
}

impl fmt::Debug for EmailAddress {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <Self as Masked>::masked_debug(self, f)
    }
}

impl AsUnsafeRef<str> for EmailAddress {
    #[inline]
    unsafe fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

// --- Sealed traits (not parts of the public API) ---

impl Sanitized for EmailAddress {
    #[inline]
    fn sanitize(input: &str) -> Self {
        let mut output = Self(String::with_capacity(input.len()));
        trim_whitespaces(&mut output.0, input);
        output
    }
}

impl Validated for EmailAddress {
    #[inline]
    fn validate(self) -> Result<Self, Error> {
        let secret = email_address::EmailAddress::from_str(self.0.as_str())
            .map(Secret)
            .map_err(|_| Error::InvalidInput(format!("{self:?} is invalid")))?;
        // ensure the validator is not optimized out
        // and the drop is called on the secret wrapper.
        std::hint::black_box(secret);
        Ok(self)
    }
}

// The wrapper is needed to guarantee zeroization of the `iban::Iban` object used by validator
struct Secret(email_address::EmailAddress);

impl Drop for Secret {
    fn drop(&mut self) {
        let s = self.0.as_str();
        unsafe {
            let ptr = s.as_ptr() as *mut u8;
            let len = s.len();
            std::ptr::write_bytes(ptr, 0, len);
        }
    }
}

// SAFETY: The trait is safely implemented because exposing the first 1 character
// along with the domain name:
// 1. Neither causes out-of-bounds access to potentially INVALID (empty) data,
//    due to fallbacks to the empty strings,
// 2. Nor leaks the real data due to hiding the real length of the email address.
unsafe impl Masked for EmailAddress {
    const TYPE_WRAPPER: &'static str = "EmailAddress";

    fn masked_debug(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (local, domain) = self.0.split_once('@').unwrap_or_default();
        write!(
            f,
            "{}{}@{}",
            local.chars().next().unwrap_or_default(),
            Self::MASKING_STR,
            domain,
        )
    }
}
